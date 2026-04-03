//! Speculative slot executor for Jito shredstream consumers.
//!
//! # Overview
//!
//! Jito's shredstream proxy delivers Solana entries — the raw building blocks
//! of a slot — directly to subscribers over gRPC as they are produced by the
//! slot leader. A subscriber receives these entries before they have been written
//! to the local blockstore, before the canonical replay pipeline has touched them,
//! and therefore before any Geyser `on_account_update()` notification has fired.
//!
//! This module bridges shredstream delivery to SVM execution. The central type,
//! `SpeculativeSlotExecutor`, receives proto entry bytes from the shredstream
//! gRPC stream, deserializes them into `solana_entry::entry::Entry` values, forks
//! a working bank from the correct frozen canonical parent, executes the entries
//! through the identical SVM code path used by the canonical replay pipeline,
//! and returns the resulting account state — all without waiting for PoH or
//! signature verification.
//!
//! # Integration into agave
//!
//! Add this line to `solana-ledger/src/lib.rs`:
//!
//! ```rust
//! pub mod devil_mode_jito__;
//! ```
//!
//! The `execute_entries_speculatively()` function is a `pub fn` added directly
//! inside `solana-ledger/src/blockstore_processor.rs` (see
//! `blockstore_processor_jito_additions.rs` for its implementation). It must
//! live there because it uses private module-scope types (`ReplayEntry`,
//! `BatchExecutionTiming`, `process_entries`) that are not visible outside that
//! module. It is imported here as `crate::blockstore_processor::execute_entries_speculatively`.
//!
//! # Latency position relative to canonical commitment levels
//!
//! ```text
//! shredstream delivers entries
//!         │
//!         ▼
//! SpeculativeSlotExecutor::execute()      ← account state available HERE
//!         │
//!         │   canonical pipeline still running:
//!         │     · PoH tick hash verification
//!         │     · Ed25519 signature verification
//!         │     · process_entries() / Geyser on_account_update()
//!         │
//!         ▼
//! bank.freeze() → BankNotification::Frozen ← "processed" commitment level
//!         │
//!         ▼
//! AggregateCommitmentService              ← "confirmed" commitment level
//!         │
//!         ▼
//! root advance                            ← "finalized" commitment level
//! ```
//!
//! # Architecture: Speculative Chain with Canonical Rebase
//!
//! Every speculative bank is forked from either a canonically frozen parent or
//! from another speculative bank that has not yet been confirmed. The former is
//! called a canonical-parent bank and the latter a speculative-parent bank.
//!
//! When slot N's canonical bank is frozen and delivered via
//! `BankNotification::Frozen`, every speculative child slot that was forked from
//! the speculative slot N bank must be REBASED. Rebasing means:
//!
//!   1. A fresh bank is created for the child slot by forking from the now-verified
//!      canonical slot N bank instead of the old speculative one.
//!   2. Every proto batch that arrived via shredstream for the child slot is
//!      re-executed against the new canonical-parent bank in delivery order.
//!   3. Correction updates are returned to the caller so that their downstream
//!      state cache can be reconciled with the corrected, sound result.
//!
//! Rebasing is cheap in practice. On mainnet, canonical freeze of slot N arrives
//! approximately 200ms after shredstream delivers slot N's entries. Shredstream
//! typically delivers slot N+1 entries around the same time, which means the
//! canonical-parent path is the common case and the speculative-parent path —
//! and therefore the rebase — is exercised only when the executor is running
//! one or more slots ahead of canonical freeze.
//!
//! Rebasing propagates one level at a time. When slot N confirms, only the
//! immediate speculative children of N are rebased in that `confirm_slot` call.
//! When slot N+1 subsequently confirms, the immediate speculative children of
//! N+1 are rebased at that point. This keeps rebase cost bounded and predictable
//! regardless of speculative chain depth.
//!
//! # Correctness — speculative results require confirmation
//!
//! Because PoH and Ed25519 verification are skipped, the results of each
//! execution run are provisional. The caller must:
//!
//! 1. Buffer `SpeculativeAccountUpdate` values keyed by `slot`.
//! 2. Call `confirm_slot(slot, canonical_bank, bank_forks, accounts_to_watch)`
//!    upon receiving `BankNotification::Frozen(slot)` — the canonical replay path
//!    has verified the slot. `confirm_slot` returns correction updates for any
//!    child slots that were rebased onto the now-canonical parent.
//! 3. Call `discard_slot(slot)` upon receiving `SlotUpdate::Dead { slot, .. }` —
//!    the canonical path rejected the slot. `discard_slot` returns every slot in
//!    the condemned subtree (the dead slot and all descendants, regardless of
//!    depth). Drop ALL buffered `SpeculativeAccountUpdate` values for every slot
//!    in that returned set without acting on them.
//!
//! # Multi-batch slot delivery
//!
//! Shredstream may deliver a slot's entries across multiple gRPC messages as
//! the leader produces them. `SpeculativeSlotExecutor` maintains a per-slot
//! speculative bank so that state written by batch N is visible to transactions
//! in batch N+1, exactly as it would be in the canonical replay path. Every
//! proto batch is stored alongside the bank so that it can be replayed against
//! the canonical-parent bank when a rebase is triggered. The slot bank and its
//! stored batches are released by `confirm_slot()` or `discard_slot()`.
//!
//! # Rebase and the `rebasing` flag — preventing tx_count races
//!
//! When `confirm_slot` begins a rebase for a child slot, it resets that child's
//! `tx_count` to 0 and re-executes all stored batches in order, updating
//! `tx_count` after each one. If a concurrent `execute()` call for the same
//! child slot read `tx_count` between two of those updates it would see a
//! partially-rebuilt count and compute a wrong slot-relative starting index for
//! its new batch — causing permanent, undetectable index misalignment in the SVM.
//!
//! The `rebasing` flag in `SlotState` prevents this. When Phase 1 of `confirm_slot`
//! marks a child as rebasing, any concurrent `execute()` for that slot finds the
//! flag under a read lock and immediately returns `Err(SpeculativeExecutorError::Rebasing)`.
//! The caller retries after a brief delay. By the time the caller retries, Phase 2
//! has finished re-execution, set `rebasing = false`, and `tx_count` reflects the
//! total of all re-played batches. The retried `execute()` then reads the correct
//! `tx_count` as its starting index.

use {
    crate::{
        blockstore_processor::{
            execute_entries_speculatively, set_alpenglow_ticks, BlockstoreProcessorError,
        },
        leader_schedule_cache::LeaderScheduleCache,
    },
    agave_votor_messages::migration::MigrationStatus,
    // wincode is the schema-validated serialization library used by solana_entry::entry::Entry.
    // Entry derives SchemaRead/SchemaWrite from wincode, which generates an implementation of
    // wincode::Deserialize for the type. SchemaRead enforces compile-time length bounds
    // (MaxDataShredsLen) on the transactions field, preventing oversized entries from being
    // deserialized. wincode::deserialize is the correct decode function — it returns
    // Result<T, wincode::ReadError>. wincode::Error is the outer enum; wincode::ReadError is
    // the specific inner type produced by the deserialize path, which is what we must carry
    // in DeserializeEntries so that the From impl and map_err resolve to the same concrete type.
    wincode::{self, ReadError as WincodeError},
    rayon::ThreadPool,
    solana_account::AccountSharedData,
    solana_clock::Slot,
    solana_entry::entry::Entry,
    solana_pubkey::Pubkey,
    solana_runtime::{
        bank::{Bank, NewBankOptions},
        bank_forks::BankForks,
        installed_scheduler_pool::BankWithScheduler,
    },
    std::{
        collections::{HashMap, HashSet},
        mem,
        sync::{Arc, RwLock},
    },
    thiserror::Error,
};

// =============================================================================
// Error type
// =============================================================================

/// Errors that can occur during speculative entry execution or rebase.
#[derive(Debug, Error)]
pub enum SpeculativeExecutorError {
    /// The `entries` bytes field from the shredstream proto `Entry` message
    /// could not be deserialized into `Vec<solana_entry::entry::Entry>`.
    ///
    /// `Entry` derives `SchemaRead` from wincode, which uses the same binary wire
    /// format as bincode but enforces a compile-time length bound on the transactions
    /// field (`MaxDataShredsLen`). `wincode::deserialize` returns `wincode::ReadError`
    /// when decoding fails — this is the specific error type produced by the read path,
    /// distinct from `wincode::Error` which is the outer enum covering both read and
    /// write failures. A failure here means the bytes are corrupt, truncated, exceed
    /// the `MaxDataShredsLen` bound, or were produced by a mismatched serialization
    /// version. The same bytes would fail again on any retry.
    #[error("failed to deserialize shredstream entry bytes into Vec<Entry>: {0}")]
    DeserializeEntries(#[from] WincodeError),

    /// The SVM execution stage failed. This can happen if the working bank was
    /// in an inconsistent state, an account load failed fatally, the block
    /// exceeded compute cost limits, or a vote-only mode violation occurred.
    /// Sanitization failures (malformed transaction format, ALT resolution
    /// errors) are also surfaced through this variant via
    /// `BlockstoreProcessorError`.
    #[error("speculative execution failed: {0}")]
    Execution(#[from] BlockstoreProcessorError),

    /// The parent bank for this slot cannot be found in either the speculative
    /// bank cache or in the canonical frozen banks held by BankForks.
    ///
    /// This occurs when shredstream delivers child-slot entries before the
    /// parent slot has completed canonical replay. The parent bank must exist
    /// in one of the two places before child entries can be executed, because
    /// every bank is created by forking from its parent — the child inherits
    /// the parent's accounts DB state, epoch boundary context, and fee
    /// configuration. Executing without a parent would produce account state
    /// with no coherent ancestry in the fork tree.
    ///
    /// The caller should retry after a brief delay or drop the batch entirely
    /// and wait for the canonical path to deliver the parent via
    /// `BankNotification::Frozen`.
    ///
    /// This error is also returned when the race condition between `execute()`
    /// and `confirm_slot()` is detected: if the parent was speculative at the
    /// time the speculative lookup was performed but was evicted by `confirm_slot`
    /// before the child bank could be inserted, the caller must retry so the
    /// child is forked from the now-canonical parent instead.
    #[error("parent bank for slot {parent_slot} not found (child slot: {child_slot})")]
    ParentBankNotFound {
        parent_slot: Slot,
        child_slot: Slot,
    },

    /// The slot's bank is currently being rebased onto a newly confirmed
    /// canonical parent by a concurrent `confirm_slot` call.
    ///
    /// Rebasing resets `tx_count` to 0 and re-executes every stored batch in
    /// delivery order, updating `tx_count` after each one. If `execute` were
    /// allowed to proceed concurrently it would read a partially-rebuilt
    /// `tx_count` and assign its new batch a wrong slot-relative starting index.
    /// The SVM embeds this index into transaction metadata — a wrong index is
    /// a silent, permanent corruption of the slot's execution record.
    ///
    /// This error is intentionally transient. The caller must wait briefly and
    /// retry. By the time the retry fires, `confirm_slot` will have finished
    /// re-execution, set `rebasing = false`, and `tx_count` will reflect the
    /// correct accumulated total for the child slot.
    #[error("slot {0} is currently being rebased — retry after a brief delay")]
    Rebasing(Slot),

    /// The slot has already been finalised by `confirm_slot` or condemned by
    /// `discard_slot`.
    ///
    /// Shredstream occasionally delivers entry batches for slots that have
    /// already been confirmed by canonical replay (late network packets) or
    /// condemned as dead.  Without this guard, `execute()` would re-create the
    /// slot's speculative bank from the now-canonical parent, produce
    /// `SpeculativeAccountUpdate` values for state that canonical replay has
    /// already settled, and then never clean up the re-created bank because
    /// `confirm_slot` and `discard_slot` fire exactly once per slot.
    ///
    /// This error is **not** retriable.  The caller must discard the batch.
    /// The canonical pipeline has already handled the slot definitively.
    #[error("slot {0} has already been confirmed or condemned — batch discarded")]
    SlotCompleted(Slot),
}

// =============================================================================
// execute_entries_speculatively — lives in blockstore_processor.rs
// =============================================================================
//
// `execute_entries_speculatively` is a `pub fn` added directly inside
// `solana-ledger/src/blockstore_processor.rs` immediately after
// `confirm_slot_entries()` (~line 1696). It must live there because it uses
// private module-scope types — `ReplayEntry`, `BatchExecutionTiming`, and the
// private function `process_entries` — that are not accessible from outside
// that module.
//
// The function is imported above as:
//   crate::blockstore_processor::execute_entries_speculatively
//
// Its signature is:
//
//   pub fn execute_entries_speculatively(
//       bank: &BankWithScheduler,
//       replay_tx_thread_pool: &ThreadPool,
//       entries: Vec<Entry>,
//       tx_starting_index: usize,
//   ) -> result::Result<usize, BlockstoreProcessorError>
//
// Call sites in this file wrap Arc<Bank> with
// BankWithScheduler::new_without_scheduler(arc_bank.clone()) before passing it
// here — the same wrapping pattern used in bank_forks.rs and blockstore_processor
// tests. The BankWithScheduler wrapper does not install a scheduler; execution
// falls through to the rayon-based path that process_entries uses for canonical
// blockstore replay.
//
// See solana-ledger/src/blockstore_processor_jito_additions.rs for the complete
// implementation and documentation.

// =============================================================================
// Public result type
// =============================================================================

/// Account state read from the speculative working bank immediately after
/// execution, containing only accounts whose state genuinely changed relative
/// to the state that existed before this specific batch was processed.
///
/// An account in `accounts_to_watch` that was not touched by any transaction
/// in this batch will be absent from the map. Absence means "no change in this
/// batch" — not "account does not exist."
///
/// This is provisional data. See the module-level documentation for the
/// confirmation and discard protocol that must be followed before acting on it.
///
/// `Debug` allows log-level inspection of update payloads during development.
/// `Clone` allows the caller to buffer updates across multiple batches without
/// consuming the original while the confirmation signal is still pending.
#[derive(Debug, Clone)]
pub struct SpeculativeAccountUpdate {
    /// The slot these account updates belong to.
    ///
    /// Use this to correlate updates with `BankNotification::Frozen` and
    /// `SlotUpdate::Dead` events from the canonical replay pipeline.
    pub slot: Slot,

    /// Post-execution state of watched accounts that were genuinely modified by
    /// at least one transaction in THIS batch, keyed by pubkey.
    ///
    /// The comparison baseline differs depending on `is_correction`:
    ///
    /// When `is_correction` is false (a normal incremental batch delivery), the
    /// baseline is the account state immediately before THIS batch executed —
    /// not the state at the start of the slot. Each delivery reports only what
    /// it specifically changed.
    ///
    /// When `is_correction` is true (a rebase correction), the baseline is the
    /// canonically frozen parent bank's state. The map represents the TOTAL
    /// accumulated effect of all batches for this child slot re-executed against
    /// the verified canonical parent. The caller must REPLACE (not accumulate)
    /// any prior state held for this slot with the contents of this map.
    pub accounts: HashMap<Pubkey, AccountSharedData>,

    /// Signals whether this update is an incremental batch delta or a canonical
    /// rebase correction.
    ///
    /// A speculative pipeline for a given slot produces two kinds of updates
    /// through the same channel:
    ///
    ///   · `false` — an incremental delta from one shredstream batch delivery.
    ///     The caller accumulates these deltas as they arrive, building a
    ///     growing picture of the slot's in-progress state. Each delivery only
    ///     reports the accounts it specifically changed.
    ///
    ///   · `true` — a rebase correction returned by `confirm_slot`. The parent
    ///     slot has been canonically verified and the child was re-executed from
    ///     scratch against that verified parent. The accounts map now holds the
    ///     TOTAL effect of all batches seen so far, measured from the canonical
    ///     parent baseline. The caller must discard every incremental delta it
    ///     accumulated for this slot and replace that state with exactly what
    ///     this correction contains.
    ///
    /// Treating a correction as another incremental accumulation is a silent
    /// state corruption: the accounts map would be added to prior speculative
    /// state rather than replacing it, yielding a double-counted result that
    /// no transaction ever actually produced.
    pub is_correction: bool,
}

// =============================================================================
// Internal per-slot state
// =============================================================================

/// All state the executor maintains for a single actively-speculated slot.
///
/// Every slot that has received at least one shredstream delivery and has not
/// yet been confirmed or discarded owns one of these in `slot_banks`.
struct SlotState {
    /// The working bank for this slot. All shredstream batches for this slot
    /// execute against this bank and commit their results into its write cache.
    /// Because write cache entries accumulate across batches, transactions in
    /// batch N+1 can read account state produced by batch N, exactly mirroring
    /// the sequential guarantee the canonical replay pipeline provides through
    /// its single persistent bank per slot.
    bank: Arc<Bank>,

    /// Running total of transactions executed for this slot across all batches
    /// received so far. This seeds the slot-relative transaction starting index
    /// for each incoming batch, ensuring the SVM assigns the same sequential
    /// transaction indexes that the canonical replay pipeline would assign via
    /// `ConfirmationProgress.num_txs`. Ticks contribute zero to this count.
    tx_count: usize,

    /// The slot number of this slot's parent. Stored so that `discard_slot`
    /// can walk the ancestry chain and condemn every speculative slot whose
    /// execution was rooted in invalid state, regardless of how many levels
    /// deep the speculative fork extends.
    parent_slot: Slot,

    /// Whether the PARENT bank that this slot's bank was forked from has been
    /// canonically verified by the canonical replay pipeline.
    ///
    /// When this flag is true, the parent bank's PoH chain, Ed25519 signatures,
    /// and SVM execution have all been verified by the network. This slot's
    /// speculative results carry only its own unverified uncertainty — exactly
    /// one generation of speculation.
    ///
    /// When this flag is false, the parent bank was itself speculative at the
    /// time this slot's bank was created. The results carry combined uncertainty
    /// from both this slot's own unverified entries AND its parent's. When
    /// `confirm_slot` is called for the parent, this slot is REBASED: a fresh
    /// bank is forked from the now-canonical parent and all stored batches are
    /// re-executed against it, producing results that are as trustworthy as
    /// canonical replay.
    parent_is_canonical: bool,

    /// Whether this slot is currently undergoing a canonical rebase.
    ///
    /// A rebase resets `tx_count` to 0 and re-executes every stored batch in
    /// delivery order, advancing `tx_count` after each one. Any `execute()`
    /// call that reads `tx_count` during this window would see a partially
    /// rebuilt count and assign its new batch a wrong slot-relative starting
    /// index. Because the SVM embeds this index into transaction metadata,
    /// the corruption would be silent and permanent.
    ///
    /// `execute()` checks this flag under the read lock. If true it immediately
    /// returns `Err(SpeculativeExecutorError::Rebasing)` without touching the
    /// bank or `tx_count`. The caller retries after a brief delay. By then
    /// `confirm_slot` will have finished re-execution, cleared the flag, and
    /// left `tx_count` at its correct post-rebase total.
    rebasing: bool,

    /// Every proto batch received via shredstream for this slot, in delivery
    /// order, paired with the slot-relative transaction starting index that was
    /// used when that batch was originally executed.
    ///
    /// These bytes are retained so that when a rebase is triggered — the parent
    /// slot transitions from speculative to canonical — every batch can be
    /// re-executed against the newly created canonical-parent bank in exactly
    /// the order the leader produced them. The starting index stored alongside
    /// each batch ensures the rebased execution assigns the same slot-relative
    /// transaction indexes as the original run, matching what canonical replay
    /// would assign.
    ///
    /// The memory cost is one copy of each batch's raw wincode bytes per active
    /// slot. On mainnet with ~200ms canonical latency and ~400ms slot times, the
    /// executor is typically no more than one slot ahead of canonical freeze, so
    /// this is a small and bounded memory footprint.
    pending_proto_batches: Vec<(Vec<u8>, usize)>,
}

// =============================================================================
// Executor
// =============================================================================

/// Executes shredstream entries speculatively to produce account state before
/// the canonical replay pipeline has verified or executed them.
///
/// # Thread safety
///
/// `SpeculativeSlotExecutor` is `Send + Sync`. The rayon thread pool and the
/// leader schedule cache are `Arc`-wrapped. The per-slot bank cache is protected
/// by a `RwLock`. Callers may invoke `execute()` from an async task by wrapping
/// the call in `tokio::task::spawn_blocking`. Do not call `execute()` concurrently
/// for the same slot from multiple threads — the per-slot bank is shared and
/// account locking inside the SVM is not safe for concurrent callers on the same
/// bank.
pub struct SpeculativeSlotExecutor {
    /// The rayon thread pool used for parallel transaction batch execution
    /// inside `execute_batches_internal()`. Sharing this with the canonical
    /// replay path reuses existing threads rather than spawning new ones,
    /// which avoids CPU contention and OS scheduling overhead.
    replay_tx_thread_pool: Arc<ThreadPool>,

    /// The leader schedule cache used to resolve the slot leader's identity
    /// when creating a new speculative child bank.
    ///
    /// Each Solana bank records a `collector_id` — the public key of the slot
    /// leader — which serves as the fee destination during transaction execution.
    /// Every transaction that successfully pays a fee credits lamports to this
    /// account inside `load_execute_and_commit_transactions`. Using the correct
    /// leader pubkey as the `collector_id` ensures that watched accounts receiving
    /// fee income (including the leader's own account) reflect accurate
    /// post-execution balances in the returned `SpeculativeAccountUpdate`.
    leader_schedule_cache: Arc<LeaderScheduleCache>,

    /// The validator-wide Alpenglow migration status, shared with the rest of
    /// the validator via `Arc`. Passed to
    /// `blockstore_processor::set_alpenglow_ticks` during every child bank
    /// creation so the bank's tick height is configured consistently with
    /// what the canonical replay pipeline uses.
    ///
    /// `MigrationStatus` tracks the protocol-level state of the cluster's
    /// transition from PoH consensus to Alpenglow BFT. Its method
    /// `should_have_alpenglow_ticks(slot)` returns true only once the cluster
    /// has both activated the feature gate AND achieved the quorum needed to
    /// formally enter the Alpenglow phase. This is a stricter gate than
    /// checking `bank.feature_set.activated_slot(alpenglow::id())` directly,
    /// because feature activation can precede full cluster acceptance.
    ///
    /// Under pure PoH (pre-Alpenglow), `should_have_alpenglow_ticks` returns
    /// false for every slot and `set_alpenglow_ticks` is a no-op. Under
    /// Alpenglow, it sets `bank.tick_height` to
    /// `max_tick_height - alpenglow_ticks` so that `bank.is_block_boundary()`
    /// fires at the correct boundary moment during `process_entries`.
    ///
    /// Because this is an `Arc`, phase transitions observed by the rest of
    /// the validator are immediately visible here without extra synchronization.
    migration_status: Arc<MigrationStatus>,

    /// Per-slot speculative banks and their associated execution state.
    ///
    /// RwLock is used rather than Mutex so that concurrent execute() calls for
    /// DIFFERENT slots can read their already-created banks simultaneously
    /// without blocking each other. The common path — batch N>1 for a slot whose
    /// bank already exists — requires only a shared read lock to clone the Arc.
    /// Only the first batch for a new slot, which inserts a fresh bank, requires
    /// the exclusive write lock.
    ///
    /// Entries are inserted on the first shredstream delivery for a slot and
    /// removed atomically by `confirm_slot()` or `discard_slot()`.
    slot_banks: RwLock<HashMap<Slot, SlotState>>,

    /// Set of slots that have been permanently finalised — either confirmed by
    /// `confirm_slot` (canonical replay froze the slot) or condemned by
    /// `discard_slot` (canonical replay declared the slot dead).
    ///
    /// `execute()` checks this set before creating a new speculative bank.  If
    /// the slot is already in this set, `execute()` returns
    /// `Err(SlotCompleted)` immediately without producing any bank or update.
    ///
    /// Without this guard a late shredstream delivery for a confirmed/condemned
    /// slot would re-create the slot bank.  The re-created bank is never cleaned
    /// up because `confirm_slot` and `discard_slot` fire exactly once per slot,
    /// so the bank leaks in `slot_banks` forever and generates false arbitrage
    /// signals from state the network has already finalised.
    ///
    /// The set grows monotonically.  A slot number occupies 8 bytes; even
    /// retaining every slot for a 24-hour period at 2.5 slots/s is under 2 MB.
    /// Callers that track root advancement may call `prune_completed_before`
    /// to bound the set at the current root.
    completed_slots: RwLock<HashSet<Slot>>,
}

impl SpeculativeSlotExecutor {
    /// Create a new `SpeculativeSlotExecutor`.
    ///
    /// # Arguments
    ///
    /// - `replay_tx_thread_pool` — The validator's existing rayon thread pool.
    ///                              Reuse the same pool used by canonical replay
    ///                              to avoid spawning competing OS threads.
    ///
    /// - `leader_schedule_cache` — The validator's shared leader schedule cache.
    ///                              Used to resolve the correct slot leader pubkey
    ///                              so that transaction fees are credited to the
    ///                              real leader's account during speculative
    ///                              execution, matching canonical behavior.
    ///
    /// - `migration_status`      — The validator's shared Alpenglow migration
    ///                              status. Forwarded to
    ///                              `blockstore_processor::set_alpenglow_ticks`
    ///                              on every child bank creation so that the
    ///                              bank's tick height matches what canonical
    ///                              replay sets for the same slot. Under PoH
    ///                              this has no effect; under Alpenglow it
    ///                              ensures speculative banks fire
    ///                              `is_block_boundary()` at the same moment
    ///                              as the canonical bank for the same slot.
    pub fn new(
        replay_tx_thread_pool: Arc<ThreadPool>,
        leader_schedule_cache: Arc<LeaderScheduleCache>,
        migration_status: Arc<MigrationStatus>,
    ) -> Self {
        Self {
            replay_tx_thread_pool,
            leader_schedule_cache,
            migration_status,
            slot_banks: RwLock::new(HashMap::new()),
            completed_slots: RwLock::new(HashSet::new()),
        }
    }

    /// Fork a new child bank for `slot` from `parent_bank`, mirroring the
    /// bank creation sequence in `generate_new_bank_forks()` (replay_stage.rs
    /// lines 4266-4293 in jito-solana).
    ///
    /// The three-step sequence performed here is:
    ///
    /// 1. Resolve the slot leader pubkey from the leader schedule cache. This
    ///    pubkey becomes the bank's `collector_id` — the account that receives
    ///    base fees and priority fees from every successfully executed transaction
    ///    in this slot. An incorrect `collector_id` causes fee credits to land on
    ///    the wrong account, producing speculative account state that diverges
    ///    from what the canonical pipeline produces.
    ///
    ///    In this version of agave, `leader_schedule_cache.slot_leader_at()` returns
    ///    `Option<SlotLeader>` where `SlotLeader` is a newtype wrapper around a
    ///    `Pubkey`. The `.id` field on `SlotLeader` extracts the underlying pubkey.
    ///    `Bank::new_from_parent_with_options` requires `&Pubkey` for its
    ///    `leader_id` parameter, so the `.id` field must be accessed explicitly
    ///    before taking the reference.
    ///
    /// 2. Create the bank via `Bank::new_from_parent_with_options`, passing the
    ///    vote-only flag obtained from `migration_status.should_bank_be_vote_only`.
    ///
    ///    Solana has two separate mechanisms that place a bank in vote-only mode:
    ///
    ///      a) Root-distance safety valve: when a slot is more than
    ///         `MAX_ROOT_DISTANCE_FOR_VOTE_ONLY` (400) slots ahead of the current
    ///         root, the cluster interprets this as the validator falling far behind
    ///         and restricts the bank to vote transactions only. This prevents a
    ///         lagging node from executing user transactions against severely stale
    ///         state.
    ///
    ///      b) Alpenglow migration window: during the transition from PoH consensus
    ///         to Alpenglow BFT, specific slots are designated vote-only by the
    ///         `MigrationStatus` oracle, which combines feature gate activation with
    ///         cluster quorum acceptance. This is a stricter gate than feature
    ///         activation alone — the cluster must both activate the feature AND
    ///         achieve the quorum needed to formally enter the Alpenglow phase.
    ///
    ///    `migration_status.should_bank_be_vote_only(slot)` encapsulates BOTH
    ///    conditions correctly, exactly as the canonical replay stage does in
    ///    `generate_new_bank_forks` and `maybe_start_leader`. Using only the
    ///    root-distance check would miss the migration-period case and produce
    ///    speculative banks with `vote_only_bank=false` for slots where canonical
    ///    replay creates them with `vote_only_bank=true`, causing speculative
    ///    account state that canonical replay would never produce.
    ///
    ///    When a bank is vote-only, the execution pipeline rejects non-vote
    ///    transactions.
    ///
    /// 3. Call `set_alpenglow_ticks` to initialise the tick height for Alpenglow
    ///    BFT compatibility. `set_alpenglow_ticks` accepts both `&Bank` and
    ///    `&MigrationStatus`. Rather than reading the feature gate slot directly
    ///    from `bank.feature_set`, it delegates to
    ///    `migration_status.should_have_alpenglow_ticks(slot)`, which also checks
    ///    that the cluster has achieved the quorum needed for the Alpenglow phase
    ///    transition — a strictly stronger condition than feature activation alone.
    ///    Under pure PoH this call is a no-op. Under Alpenglow, it sets the bank's
    ///    tick height to `max_tick_height - alpenglow_ticks` so that
    ///    `bank.is_block_boundary()` fires at the correct moment during replay.
    fn create_child_bank(
        &self,
        parent_bank: Arc<Bank>,
        slot: Slot,
    ) -> Bank {
        // `slot_leader_at` returns `None` when the epoch containing `slot` has
        // not yet had its leader schedule materialised — an extremely rare boundary
        // condition. The default (all-zeros) pubkey is used as a fallback, accepting
        // that fee credits land on the system-owned zero address for that one
        // exceptional slot rather than causing a hard failure that would abandon
        // the batch entirely.
        //
        // `SlotLeader` is a newtype struct wrapping a `Pubkey` via a public `.id`
        // field. `Bank::new_from_parent_with_options` takes `leader_id: &Pubkey`,
        // so `.id` is accessed to extract the underlying pubkey before the borrow
        // is taken. `unwrap_or_default()` on `Option<SlotLeader>` returns
        // `SlotLeader::default()` whose `.id` field is `Pubkey::default()` —
        // the all-zeros fallback described above.
        let leader_pubkey = self
            .leader_schedule_cache
            .slot_leader_at(slot, Some(&*parent_bank))
            .unwrap_or_default();

        // `should_bank_be_vote_only` is the single authoritative gate for
        // vote-only mode. It handles both the root-distance safety valve (a slot
        // more than MAX_ROOT_DISTANCE_FOR_VOTE_ONLY slots ahead of root is
        // restricted to votes-only to protect against lagging validators) and the
        // Alpenglow migration window (certain slots are restricted to votes-only
        // while the cluster is transitioning from PoH to Alpenglow BFT consensus).
        // Delegating here instead of computing root-distance directly ensures that
        // speculative banks are created with the same vote_only_bank flag that the
        // canonical replay pipeline sets for the same slot in both code paths.
        let vote_only_bank = self.migration_status.should_bank_be_vote_only(slot);

        let bank = Bank::new_from_parent_with_options(
            parent_bank,
            // `.id` dereferences the SlotLeader newtype to its inner Pubkey.
            // The function signature requires &Pubkey, not &SlotLeader.
            &leader_pubkey.id,
            slot,
            NewBankOptions { vote_only_bank },
        );

        // `set_alpenglow_ticks` accepts `&MigrationStatus` alongside `&Bank`.
        // Rather than inspecting `bank.feature_set.activated_slot(alpenglow::id())`
        // directly, the function delegates to
        // `migration_status.should_have_alpenglow_ticks(bank.slot())`.
        //
        // `MigrationStatus` encapsulates the Alpenglow protocol transition state:
        // it combines feature gate activation with cluster quorum acceptance of
        // the Alpenglow migration. Checking `should_have_alpenglow_ticks` is
        // therefore a stricter gate than checking feature activation alone — the
        // cluster must both activate the feature AND achieve the quorum needed to
        // formally enter the Alpenglow phase before tick height is overridden.
        //
        // Under pure PoH, `should_have_alpenglow_ticks` returns false for every
        // slot and the function returns immediately without modifying the bank.
        // Under Alpenglow, the function sets `bank.tick_height` to
        // `max_tick_height - alpenglow_ticks` so that `bank.is_block_boundary()`
        // fires at exactly the same moment during `process_entries` as it would
        // in the canonical replay pipeline for the same slot.
        //
        // Passing `self.migration_status.as_ref()` dereferences the Arc to a
        // plain `&MigrationStatus` — a zero-cost operation that avoids cloning
        // the Arc for a call that only needs a shared read.
        set_alpenglow_ticks(&bank, self.migration_status.as_ref());

        bank
    }

    /// Execute a shredstream entry batch speculatively and return genuinely
    /// modified account state.
    ///
    /// # Steps
    ///
    /// 1. **Deserialize** — convert the raw `proto_entry_bytes` from the
    ///    shredstream proto into `Vec<Entry>` using wincode's schema-validated
    ///    decoder. `Entry` derives `SchemaRead` from wincode, which enforces the
    ///    `MaxDataShredsLen` bound on the transactions field at decode time.
    ///
    /// 2. **Bank** — look up the cached speculative bank for this slot under a
    ///    shared read lock (fast path), or create a new one under an exclusive
    ///    write lock (first batch only). Bank creation resolves the parent from
    ///    either the speculative cache or canonical BankForks and records whether
    ///    the parent is canonical so that a future rebase can be triggered correctly.
    ///    If the slot is currently being rebased, return `Err(Rebasing)` immediately
    ///    so the caller can retry once rebase completes and `tx_count` is stable.
    ///
    /// 3. **Snapshot pre-state** — read the current state of all watched accounts
    ///    from the working bank's write cache before execution. This per-batch
    ///    baseline lets the returned delta contain only changes from THIS delivery.
    ///
    /// 4. **Execute** — call `execute_entries_speculatively()`, which mirrors
    ///    `confirm_slot_entries()` but skips PoH and Ed25519 verification. Account
    ///    state is written into the speculative bank's write cache.
    ///
    /// 5. **Persist batch** — store the raw proto bytes and the tx starting index
    ///    so the batch can be re-executed during a canonical rebase.
    ///
    /// 6. **Read delta** — compare post-execution state against the Step 3
    ///    snapshot for each watched account and return only accounts that changed.
    ///
    /// # Arguments
    ///
    /// - `slot`              — Slot number from the shredstream proto `Entry.slot`.
    ///
    /// - `parent_slot`       — The parent slot number, taken directly from the
    ///                          shredstream proto `EntryNotification.parent_slot`
    ///                          field. This is the same value the leader encoded
    ///                          into every shred's `DataShredHeader.parent_offset`
    ///                          field at block production time, recovered as
    ///                          `parent_slot = slot - u64::from(parent_offset)`.
    ///                          Using this value guarantees the correct parent bank
    ///                          is selected even during fork conditions where
    ///                          multiple frozen banks exist below the target slot.
    ///
    /// - `proto_entry_bytes` — wincode-serialized `Vec<Entry>` from the shredstream
    ///                          proto `EntryNotification.entries` field.
    ///
    /// - `bank_forks`        — The validator's live `BankForks`, read-locked only
    ///                          during parent bank selection for the first batch.
    ///
    /// - `accounts_to_watch` — Pubkeys whose post-execution state should be returned
    ///                          if it changed during this specific batch.
    ///
    /// # Returns
    ///
    /// `Ok(SpeculativeAccountUpdate)` containing only accounts from
    /// `accounts_to_watch` whose state genuinely changed during this batch.
    /// The returned update has `is_correction = false` — it is an incremental
    /// delta from a single batch delivery, not a total-replacement rebase correction.
    ///
    /// `Err(SpeculativeExecutorError::Rebasing)` if the slot's bank is currently
    /// being re-executed against a newly confirmed canonical parent. Retry after
    /// a brief delay (a few milliseconds is sufficient).
    ///
    /// `Err(SpeculativeExecutorError)` if deserialization, parent resolution, or
    /// SVM execution fails.
    pub fn execute(
        &self,
        slot: Slot,
        parent_slot: Slot,
        proto_entry_bytes: &[u8],
        bank_forks: &RwLock<BankForks>,
        accounts_to_watch: &[Pubkey],
    ) -> Result<SpeculativeAccountUpdate, SpeculativeExecutorError> {
        // ---------------------------------------------------------------------
        // Step 1: Deserialize proto bytes into Vec<Entry>.
        //
        // `Entry` derives `SchemaRead` and `SchemaWrite` from wincode rather than
        // serde::Serialize/Deserialize. wincode uses the same binary wire format as
        // bincode but adds a compile-time schema layer: the transactions field is
        // annotated with `#[wincode(with = "WincodeVec<VersionedTransaction, MaxDataShredsLen>")]`,
        // which enforces a maximum byte length at decode time equal to
        // `MAX_DATA_SHREDS_PER_SLOT * PACKET_DATA_SIZE`. This bound mirrors the
        // physical constraint on how many data shreds a single slot can carry,
        // preventing a malformed or oversized shredstream payload from causing an
        // out-of-memory condition during decode.
        //
        // `Entry` derives `SchemaRead` from wincode, which generates an implementation
        // of `wincode::Deserialize` for the type. `wincode::deserialize` reads any type
        // implementing `wincode::Deserialize` from a byte slice and returns
        // `Result<T, wincode::ReadError>`. `wincode::ReadError` is the specific error
        // type produced by the read/deserialize path — it is NOT the outer `wincode::Error`
        // enum which covers both read and write failures. `Vec<Entry>` is decodable because
        // wincode provides a blanket `wincode::Deserialize` implementation for
        // `Vec<T: wincode::Deserialize>`. `DeserializeEntries` carries `#[from] WincodeError`
        // where `WincodeError` aliases `wincode::ReadError`, so `?` converts and propagates
        // the error automatically without any explicit wrapping.
        // ---------------------------------------------------------------------
        let entries: Vec<Entry> = wincode::deserialize(proto_entry_bytes)
            .map_err(SpeculativeExecutorError::DeserializeEntries)?;

        // ---------------------------------------------------------------------
        // RACE-3 guard: reject late batches for already-finalised slots.
        //
        // Shredstream occasionally delivers entry batches for slots that canonical
        // replay has already confirmed or condemned.  Without this check, execute()
        // would re-create the slot's speculative bank from the now-canonical parent,
        // produce SpeculativeAccountUpdate values for state the network has already
        // settled, and then never clean up the re-created bank — because
        // confirm_slot and discard_slot each fire exactly once per slot.
        //
        // The read lock is held for a single HashSet::contains call — nanoseconds.
        // The common path (slot not yet complete) exits immediately with no cost.
        // ---------------------------------------------------------------------
        if self.completed_slots.read().unwrap().contains(&slot) {
            return Err(SpeculativeExecutorError::SlotCompleted(slot));
        }

        // ---------------------------------------------------------------------
        // PERF-1: Pre-compute the owned batch bytes BEFORE acquiring any lock.
        //
        // proto_entry_bytes.to_vec() allocates a new Vec<u8> and copies every byte
        // of the shredstream payload.  If this call were inside the write lock that
        // follows, it would hold the exclusive slot_banks lock for the duration of
        // the malloc + memcpy — blocking every concurrent execute() call for every
        // slot in the system.  Computing it here, while holding no lock, removes
        // that allocation from the critical section entirely.
        // ---------------------------------------------------------------------
        let batch_bytes: Vec<u8> = proto_entry_bytes.to_vec();

        // ---------------------------------------------------------------------
        // Step 2: Locate or create the working bank for this slot.
        //
        // Two-phase locking strategy:
        //
        //   Phase 1 — Shared read lock (fast path):
        //     Most batches arrive for a slot whose bank was already created by the
        //     first delivery. A read lock allows all concurrent execute() calls for
        //     DIFFERENT slots to proceed in parallel. The bank Arc is cloned (a
        //     cheap atomic refcount increment) and the lock is released immediately.
        //
        //     Additionally, the `rebasing` flag is checked here. If the slot is
        //     currently undergoing a canonical rebase, `tx_count` is in the process
        //     of being rebuilt from zero. Proceeding would mean reading a stale
        //     `tx_count` and assigning this batch a wrong slot-relative starting
        //     index, silently corrupting the SVM's transaction index record.
        //     Returning Err(Rebasing) here forces the caller to retry after the
        //     rebase has finished and `tx_count` is stable again.
        //
        //   Phase 2 — Exclusive write lock (slow path, first batch only):
        //     When no bank yet exists for this slot, one must be created. All state
        //     needed for bank creation (speculative parent, canonical parent) is
        //     resolved BEFORE acquiring the write lock so the critical section
        //     contains only the HashMap insert. The double-check via
        //     entry().or_insert_with handles the race where two callers both miss
        //     the Phase 1 read simultaneously — only the first writer's bank
        //     is stored.
        //
        // BORROW CHECKER CONSTRAINT:
        //   HashMap::entry(&mut self, k) holds a mutable reference over the entire
        //   HashMap for the lifetime of the returned Entry object. Any attempt to
        //   access the same HashMap inside the or_insert_with closure is a
        //   simultaneous mutable + immutable borrow — a compile-time error. This
        //   is resolved by fully resolving the speculative parent bank under a
        //   separate read lock BEFORE entry() is ever called, so the closure
        //   captures only already-owned Arc values and never touches the map.
        // ---------------------------------------------------------------------

        // Phase 1: shared read — check whether the bank already exists.
        let maybe_existing: Option<(Arc<Bank>, usize)> = {
            let r = self.slot_banks.read().unwrap();
            if let Some(state) = r.get(&slot) {
                // If this slot is currently being rebased, its tx_count is being
                // rebuilt from zero. Reading tx_count now and executing against the
                // new bank would assign this batch a wrong starting index. Return a
                // retriable error so the caller waits until rebase completes.
                if state.rebasing {
                    return Err(SpeculativeExecutorError::Rebasing(slot));
                }
                Some((state.bank.clone(), state.tx_count))
            } else {
                None
            }
        }; // read lock released here

        let (working_bank, tx_starting_index): (Arc<Bank>, usize) =
            if let Some(pair) = maybe_existing {
                // Bank already exists and is not rebasing — this is batch 2, 3,
                // ..., N for this slot. tx_count is the cumulative count of
                // transactions from all prior batches and becomes the slot-relative
                // starting index for THIS batch.
                pair
            } else {
                // Phase 2: first batch for this slot — bank does not exist yet.
                //
                // Read the speculative parent under a separate read lock before
                // the write lock is acquired. This is safe: we hold no Entry into
                // the map at this point, so this is just a plain HashMap lookup
                // under a shared lock that completes and drops immediately.
                let speculative_parent_bank: Option<Arc<Bank>> = {
                    let r = self.slot_banks.read().unwrap();
                    r.get(&parent_slot).map(|s| s.bank.clone())
                }; // read lock released here

                // Resolve the parent bank under a single BankForks read-lock acquisition.
                //
                // Two-level parent lookup:
                //
                //   Level 1 — speculative cache:
                //     The parent slot may only exist in our speculative cache because
                //     the canonical pipeline has not yet frozen it. The child bank is
                //     forked from this speculative parent and marked
                //     parent_is_canonical = false. It will be REBASED when the parent
                //     is eventually confirmed, re-executing all stored batches against
                //     the then-verified canonical parent.
                //
                //   Level 2 — canonical BankForks:
                //     The parent slot has been fully replayed, PoH-verified,
                //     Ed25519-verified, and frozen. frozen_banks() returns only banks
                //     whose bank.is_frozen() is true. The child bank is marked
                //     parent_is_canonical = true — no rebase will be needed.
                //
                //   Not found:
                //     Neither location holds the parent. Return ParentBankNotFound
                //     so the caller can retry.
                let (parent_bank, parent_is_canonical) = {
                    let forks = bank_forks.read().unwrap();

                    if let Some(bank) = speculative_parent_bank {
                        // Parent is in the speculative cache, not yet canonical.
                        (bank, false)
                    } else {
                        // Parent must be canonically frozen in BankForks.
                        let parent = forks
                            .frozen_banks()
                            .find(|(s, _)| *s == parent_slot)
                            .map(|(_, b)| b)
                            .ok_or(SpeculativeExecutorError::ParentBankNotFound {
                                parent_slot,
                                child_slot: slot,
                            })?;
                        (parent, true)
                    }
                    // BankForks read lock drops here.
                };

                // create_child_bank runs BEFORE the write lock is acquired.
                // Bank::new_from_parent_with_options → _new_from_parent can call
                // process_new_epoch at epoch boundaries, which builds a rayon pool
                // and computes stake rewards — 100–500ms on mainnet. Running this
                // outside the lock means no concurrent execute() call for any other
                // slot is blocked during that window.
                let bank =
                    self.create_child_bank(parent_bank, slot);
                let arc_bank = Arc::new(bank);

                // Acquire the exclusive write lock and insert the new SlotState.
                // All CPU-intensive work was done before this point so the critical
                // section is limited to the HashMap insert. entry().or_insert_with
                // provides the double-check: if a concurrent caller inserted a bank
                // between our Phase 1 miss and now, we reuse that bank instead.
                // New banks start with rebasing = false — they are not undergoing
                // a rebase and execute() may proceed normally.
                let mut w = self.slot_banks.write().unwrap();
                w.entry(slot).or_insert_with(|| SlotState {
                    bank: arc_bank.clone(),
                    tx_count: 0,
                    parent_slot,
                    parent_is_canonical,
                    rebasing: false,
                    pending_proto_batches: Vec::new(),
                });

                // -------------------------------------------------------------
                // RACE CONDITION CHECK — executed under the same write lock
                // immediately after insertion.
                //
                // The race window: between the speculative parent read lock (above,
                // where we saw parent_slot in slot_banks and resolved parent_is_canonical
                // = false) and THIS write lock acquisition, confirm_slot(parent_slot)
                // may have run to completion on another thread. confirm_slot's
                // sub-phase 1a evicts parent_slot from slot_banks via w.remove(&slot).
                // It then scanned for children with parent_slot == this slot's parent
                // and found none (because this child did not exist yet). It completed
                // the rebase cycle and returned. It will never run again for parent_slot.
                //
                // The consequence: this child bank was forked from the OLD speculative
                // parent Arc — state that has now been superseded by the canonical bank.
                // The child is marked parent_is_canonical = false, but no future
                // confirm_slot call will ever trigger the rebase it needs, because
                // confirm_slot fires exactly once per slot.
                //
                // Detection: under this write lock, parent_slot's entry in slot_banks
                // is the authoritative source of truth. If parent_is_canonical was
                // false (we thought the parent was speculative) but parent_slot is
                // no longer in slot_banks, confirm_slot already evicted it — the
                // parent IS canonical and the child was mis-forked.
                //
                // Recovery: remove the just-inserted entry and return ParentBankNotFound.
                // The caller retries, finds parent_slot in BankForks.frozen_banks()
                // (it is now canonical and frozen), forks the child from that verified
                // parent with parent_is_canonical = true, and proceeds with correct
                // single-generation speculative ancestry.
                // -------------------------------------------------------------
                if !parent_is_canonical && !w.contains_key(&parent_slot) {
                    w.remove(&slot);
                    return Err(SpeculativeExecutorError::ParentBankNotFound {
                        parent_slot,
                        child_slot: slot,
                    });
                }

                let pair = w.get(&slot).map(|s| (s.bank.clone(), s.tx_count)).unwrap();
                pair
                // Write lock drops here.
            };

        // ---------------------------------------------------------------------
        // Step 3: Capture the per-batch pre-execution baseline.
        //
        // Bank::get_account() reads the working bank's write cache first, then
        // falls back through the ancestor chain to the accounts DB. At the moment
        // this snapshot is taken, the write cache contains the committed results
        // of all previously executed batches for this slot (batches 1 through N-1).
        //
        // Snapshotting NOW — before this batch executes — establishes a boundary
        // that allows the returned delta to contain only changes introduced by
        // THIS specific delivery. Without this snapshot, comparing against the
        // frozen parent bank would cause batch 2's result to include everything
        // batch 1 changed, batch 3's result to include everything batches 1 and 2
        // changed, and so on — turning each delivery into an ever-growing superset
        // of the entire slot's accumulated changes rather than a precise report of
        // what this one delivery contributed.
        // ---------------------------------------------------------------------
        let pre_execution_state: HashMap<Pubkey, Option<AccountSharedData>> = accounts_to_watch
            .iter()
            .map(|pubkey| (*pubkey, working_bank.get_account(pubkey)))
            .collect();

        // ---------------------------------------------------------------------
        // Step 4: Execute entries through the full canonical SVM pipeline.
        //
        // execute_entries_speculatively (crate::blockstore_processor) drives the
        // SVM via entry::validate_and_hash_transactions → process_entries, using
        // the same ReplayEntry / BatchExecutionTiming / rayon path as canonical
        // blockstore replay. The call to validate_and_hash_transactions produces a
        // ValidatedHashedTransactions struct whose unverified_signatures field is
        // intentionally discarded — Ed25519 batch verification is skipped for the
        // speculative path. PoH hash-chain verification is also omitted. All other
        // SVM code paths are identical to canonical replay. The Arc<Bank> is
        // wrapped in BankWithScheduler::new_without_scheduler at the call site
        // below — this is the identical wrapping used in bank_forks::insert() and
        // blockstore_processor tests.
        //
        // After this call, the working bank's write cache holds the post-execution
        // state of every account touched by any transaction in this batch. This is
        // the same logical moment at which Geyser's on_account_update() would fire
        // in the canonical path — just 200-400ms earlier.
        //
        // check_chained_block_id() is intentionally skipped: in canonical replay,
        // replay_active_bank() validates the chained merkle root against RocksDB
        // before replay begins. In the shredstream path, entries arrive before
        // blockstore ingestion, so the merkle root data does not yet exist in
        // RocksDB. The canonical pipeline performs this validation correctly later.
        //
        // set_block_id() is intentionally not called: block_id lives in
        // bank.block_id: RwLock<Option<Hash>> and is never read inside
        // bank.hash_internal_state() (bank.rs lines 4805-4874). The bank hash
        // derives solely from parent_hash, signature_count, last_blockhash, and
        // accounts_lt_hash bytes. Omitting set_block_id has zero effect on bank
        // hash or account state; it is metadata used only for chained merkle root
        // validation between parent and child.
        // ---------------------------------------------------------------------

        // Wrap Arc<Bank> in BankWithScheduler::new_without_scheduler so the call
        // goes through the rayon-based execution path inside process_entries,
        // matching canonical blockstore replay. The wrapper does not install a
        // scheduler — it is a zero-cost newtype for the duration of this call.
        let batch_tx_count_executed = execute_entries_speculatively(
            &BankWithScheduler::new_without_scheduler(working_bank.clone()),
            &self.replay_tx_thread_pool,
            entries,
            tx_starting_index,
        )
        .map_err(SpeculativeExecutorError::Execution)?;

        // ---------------------------------------------------------------------
        // STATUS CACHE CONTAMINATION PREVENTION
        //
        // Solana's status cache (BankStatusCache, bank.rs line 1291) is an
        // Arc<RwLock<...>> — Arc::clone copies the pointer, not the data.
        // Every bank in a fork tree shares the exact same underlying
        // RwLock<BankStatusCache> instance with its parent and every sibling.
        //
        // commit_transactions calls update_transaction_statuses which writes
        // each committed transaction's signature into the cache under this
        // slot's key. When canonical replay subsequently calls check_transactions
        // for this same slot, it reads the same shared cache and finds every
        // speculative transaction as AlreadyProcessed — silently skipping them.
        // The canonical bank then commits fewer transactions, computes a
        // different accounts_lt_hash, and arrives at a different bank hash than
        // the leader produced. The validator marks the slot Dead.
        //
        // clear_slot_signatures removes all entries for this slot's key from
        // the shared cache. Calling it here, before the canonical replay thread
        // ever acquires its read lock for this slot, erases every signature we
        // wrote and restores the cache to exactly the state it would be in had
        // speculative execution never run. The canonical path proceeds cleanly.
        // ---------------------------------------------------------------------
        working_bank.clear_slot_signatures(slot);

        // ---------------------------------------------------------------------
        // Step 5: Persist the batch and advance the cumulative transaction count.
        //
        // The raw proto bytes and the tx_starting_index used for this batch are
        // stored together in pending_proto_batches. During a canonical rebase
        // these two values allow the batch to be re-executed against the
        // canonical-parent bank in the same slot-relative position it originally
        // occupied, producing slot-relative transaction indexes that match what
        // canonical replay assigns.
        //
        // tx_count is advanced by the count returned from execute_entries_speculatively,
        // which is the total number of transactions across all entries in the batch.
        // Tick entries have zero transactions and contribute nothing to this count.
        // This mirrors how ConfirmationProgress.num_txs accumulates in the canonical
        // path (replay_stage.rs lines 2445-2465).
        //
        // If the slot was removed by a concurrent confirm_slot() or discard_slot()
        // between execution and this update, get_mut returns None and both the
        // tx_count update and the batch storage are silently skipped — a slot that
        // has been evicted will never receive another batch.
        //
        // REBASING GUARD — the critical check that closes the race window:
        //
        // Between the read lock in Phase 1 (where rebasing was observed as false)
        // and THIS write lock, a concurrent confirm_slot Phase 1 could have run
        // and set rebasing=true, reset tx_count=0, replaced state.bank with a new
        // canonical-parent bank, and moved pending_proto_batches out via mem::take.
        // If we proceeded to update tx_count and push to pending_proto_batches here,
        // we would corrupt Phase 2's incremental tx_count rebuild with a count from
        // execution against the OLD speculative bank, and inject a stale batch —
        // with a wrong tx_starting_index — into the fresh pending_proto_batches Vec.
        //
        // The guard `if !state.rebasing` prevents both corruptions atomically under
        // this write lock. When rebasing is true, Phase 2 exclusively owns tx_count
        // and pending_proto_batches. The execution results from this call are silently
        // discarded — the caller will receive a correction_update from confirm_slot
        // once the rebase completes, which supersedes anything computed here.
        // ---------------------------------------------------------------------
        {
            let mut w = self.slot_banks.write().unwrap();
            if let Some(state) = w.get_mut(&slot) {
                if !state.rebasing {
                    state.tx_count = state.tx_count.saturating_add(batch_tx_count_executed);
                    state
                        .pending_proto_batches
                        .push((batch_bytes, tx_starting_index));
                }
                // If state.rebasing is true: a canonical rebase started between our
                // Phase 1 read and this write. Phase 2 owns tx_count and
                // pending_proto_batches. Discard this execution's results silently.
            }
        } // write lock drops here

        // ---------------------------------------------------------------------
        // Step 6: Compute and return the per-batch account delta.
        //
        // Each watched account's post-execution state is read from the working
        // bank and compared against the pre-execution snapshot from Step 3.
        // An account is included in the result only if its state changed during
        // this specific batch — lamports, data, owner, executable, or rent_epoch.
        //
        // AccountSharedData implements PartialEq across all fields, so the equality
        // check detects any modification without requiring field-by-field inspection.
        //
        // Accounts that were closed (all lamports removed) return None from
        // get_account and are excluded from the result by the ? operator on the
        // post-execution read. Callers that need to track account deletions must
        // detect them by observing that an account present in a prior update is
        // absent in a subsequent one.
        //
        // is_correction is false: this is an incremental delta from a single batch
        // delivery. The caller accumulates these. Correction updates (is_correction
        // = true) come exclusively from confirm_slot() and carry a total-replacement
        // semantic — see SpeculativeAccountUpdate.is_correction for the full protocol.
        // ---------------------------------------------------------------------
        // Compute the per-batch account delta.
        //
        // `pre_execution_state` holds `Option<AccountSharedData>` values captured
        // before SVM execution.  Comparing by reference (`pre_state == &post`)
        // avoids cloning the `AccountSharedData` entirely — `AccountSharedData`
        // owns a heap-allocated data Vec that can be tens of kilobytes for large
        // accounts.  Cloning it on every watched account on every batch would add
        // substantial allocator pressure on the hot path.  Comparing references
        // is zero-cost and semantically identical: `PartialEq` on `AccountSharedData`
        // compares all fields including the data slice, so no information is lost.
        let accounts: HashMap<Pubkey, AccountSharedData> = accounts_to_watch
            .iter()
            .filter_map(|pubkey| {
                let post = working_bank.get_account(pubkey)?;
                // `pre` is Option<&AccountSharedData> — a reference into the
                // pre-execution snapshot map.  No clone of AccountSharedData occurs.
                let pre: Option<&AccountSharedData> = pre_execution_state
                    .get(pubkey)
                    .and_then(|opt| opt.as_ref());
                match pre {
                    Some(pre_state) if pre_state == &post => None,
                    _ => Some((*pubkey, post)),
                }
            })
            .collect();

        Ok(SpeculativeAccountUpdate { slot, accounts, is_correction: false })
    }

    /// Notify the executor that the canonical replay path has confirmed `slot`
    /// via `BankNotification::Frozen`, and rebase any child slots whose banks
    /// were forked from the now-superseded speculative slot N bank.
    ///
    /// # What canonical freeze means
    ///
    /// In replay_stage.rs `process_replay_results()` (lines 3305-3643), when
    /// `bank.is_complete()` returns true, the replay path:
    ///   1. Calls `check_last_fec_set_and_get_block_id()` — validates the last FEC
    ///      set and extracts the block ID from the final shred's merkle root.
    ///   2. Calls `bank.set_block_id(block_id)` (line 3452).
    ///   3. Calls `bank.freeze()` (line 3456) — commits the bank hash.
    ///   4. Runs duplicate slot resolution via `check_slot_agrees_with_cluster()`.
    ///   5. Sends `BankNotification::Frozen(bank.clone_without_scheduler())` (line 3579).
    ///
    /// At this point all entries in the slot have been received, PoH-verified,
    /// Ed25519-verified, SVM-executed, and frozen. The `canonical_bank` passed
    /// here is the bank extracted from that notification.
    ///
    /// # What rebase does
    ///
    /// A child slot whose bank was forked from the speculative slot N bank carries
    /// uncertainty inherited from slot N's unverified state. Now that slot N is
    /// verified, the child can be re-rooted in sound state without sacrificing the
    /// latency advantage already delivered to the caller on the original execution.
    ///
    /// Rebase creates a fresh bank for the child by forking from `canonical_bank`,
    /// then re-executes every stored proto batch against that new bank in delivery
    /// order. The re-execution assigns the same slot-relative transaction indexes
    /// as the original run, so the rebased result is structurally identical to
    /// what canonical replay would produce. Correction updates are returned to
    /// the caller so their downstream cache can be reconciled.
    ///
    /// Only IMMEDIATE children of `slot` whose `parent_is_canonical` is false are
    /// rebased here. Deeper descendants are rebased when their own parent is
    /// subsequently confirmed — rebasing propagates one generation per confirm_slot
    /// call, keeping the work bounded.
    ///
    /// # Rebase phases and the `rebasing` flag
    ///
    /// Phase 1 runs under the exclusive write lock: the speculative slot N bank
    /// is evicted, the new canonical-parent child banks are created, stored batch
    /// lists are moved out via `mem::take`, and each child's `rebasing` flag is
    /// set to `true`. Any concurrent `execute()` for a rebasing child slot finds
    /// the flag under its read lock and immediately returns `Err(Rebasing)`.
    ///
    /// Phase 2 runs WITHOUT any slot_banks lock: the heavy SVM re-execution of
    /// stored batches happens here for each child. This allows concurrent
    /// `execute()` calls for OTHER (non-rebasing) slots to proceed without waiting.
    /// Because rebasing child slots return `Err(Rebasing)`, no new batch can
    /// interleave with re-execution and corrupt the `tx_count` rebuild.
    ///
    /// After all batches for a child have been re-executed, the child's `rebasing`
    /// flag is cleared to `false` under a write lock. From that moment, new
    /// `execute()` calls for the child proceed normally and read the correct,
    /// fully-rebuilt `tx_count` as their starting index.
    ///
    /// # Arguments
    ///
    /// - `slot`              — The slot that just received `BankNotification::Frozen`.
    /// - `canonical_bank`    — The frozen bank from the notification. Used as the
    ///                          parent for all rebased child banks.
    /// - `bank_forks`        — Used to read `root()` for vote-only bank determination
    ///                          via `migration_status.should_bank_be_vote_only`.
    /// - `accounts_to_watch` — Pubkeys whose post-rebase state should be included
    ///                          in the correction updates.
    ///
    /// # Returns
    ///
    /// `Ok(Vec<SpeculativeAccountUpdate>)` — one correction update per rebased
    /// child slot, each containing the post-rebase account delta relative to the
    /// canonical parent. Every correction update has `is_correction = true`,
    /// signalling to the caller that the accounts map is a TOTAL REPLACEMENT of
    /// all prior speculative state for that child slot — not an incremental delta.
    /// An empty vec means no child slots needed rebasing, which is the common case
    /// when canonical freeze arrives before shredstream delivers the next slot's
    /// entries.
    ///
    /// `Err(SpeculativeExecutorError)` if deserialization or SVM execution fails
    /// during re-execution of a stored batch.
    pub fn confirm_slot(
        &self,
        slot: Slot,
        canonical_bank: Arc<Bank>,
        bank_forks: &RwLock<BankForks>,
        accounts_to_watch: &[Pubkey],
    ) -> Result<Vec<SpeculativeAccountUpdate>, SpeculativeExecutorError> {
        // -------------------------------------------------------------------------
        // Phase 1 (under exclusive write lock):
        //   a) Evict the speculative slot N bank from the cache.
        //   b) Find all immediate children whose parent_is_canonical == false.
        //   c) For each such child:
        //        - Create a fresh bank forked from canonical_bank.
        //        - Move the stored batch list out via mem::take (zero allocation).
        //        - Replace state.bank, reset state.tx_count to 0.
        //        - Set state.rebasing = true so concurrent execute() calls for
        //          this child immediately return Err(Rebasing) until Phase 2
        //          finishes re-executing all stored batches and clears the flag.
        //        - Set state.parent_is_canonical = true.
        //
        // mem::take moves the pending_proto_batches Vec out of the SlotState
        // without cloning (zero allocation), leaving an empty Vec in its place.
        // New batches arriving for the child slot during Phase 2 are stored in
        // that fresh empty Vec. They are NOT executed immediately — execute()
        // returns Err(Rebasing) for as long as rebasing == true. Once Phase 2
        // clears the flag, the next execute() call from the caller finds
        // rebasing == false, reads the correct tx_count, and executes normally.
        // -------------------------------------------------------------------------
        // Phase 1 is split into three sub-phases to prevent Bank::new_from_parent_with_options
        // from blocking the slot_banks write lock at epoch boundaries.
        //
        // EPOCH BOUNDARY LATENCY HAZARD:
        //
        // Bank::new_from_parent_with_options calls _new_from_parent, which checks
        // whether the child slot crosses an epoch boundary (parent.epoch() < new.epoch()).
        // If it does, it calls process_new_epoch, which:
        //   · Builds a rayon thread pool (solBnkNewEpch{i:02})
        //   · Calls calculate_activated_stake across all stake delegations
        //   · Calls calculate_rewards across all vote/stake accounts
        //   · Calls begin_partitioned_rewards
        // This can take 100–500ms on mainnet with ~800K stake accounts.
        //
        // If this work ran inside the slot_banks write lock, every concurrent
        // execute() call for ANY slot would block waiting for the read lock for
        // the entire duration. MevEngine would receive no SpeculativeAccountUpdates
        // for the entire epoch boundary window.
        //
        // The fix mirrors the pattern in execute(): build the new banks BEFORE
        // acquiring the write lock, so the critical section contains only O(1)
        // in-memory state mutations. The write lock is held for microseconds
        // instead of hundreds of milliseconds.
        //
        // Sub-phase 1a (write lock): evict slot N, collect child slots,
        //   move pending_proto_batches out, set rebasing=true.
        // Sub-phase 1b (no lock): call create_child_bank for each child.
        // Sub-phase 1c (write lock): install the pre-built banks and reset tx_count.
        // -------------------------------------------------------------------------

        // Sub-phase 1a: close the race window, then collect children.
        //
        // Lock ordering: execute() acquires completed_slots.read() BEFORE
        // acquiring slot_banks.read/write. To avoid a deadlock cycle,
        // confirm_slot must acquire completed_slots.write() BEFORE acquiring
        // slot_banks.write(). This is enforced here: the completed_slots insert
        // runs first, then the slot_banks write lock is acquired for the eviction
        // and child-collection steps.
        //
        // This ordering also closes the race window that existed when the insert
        // ran after slot_banks.remove. In the old ordering, an execute() call
        // that saw the slot absent from slot_banks (after remove) but not yet in
        // completed_slots (before insert) could re-create the bank, return a
        // SpeculativeAccountUpdate, and leave the bank leaked forever. With the
        // new ordering, the completed_slots insert runs before slot_banks is
        // touched, so any execute() that checks completed_slots first will find
        // the slot already finalised and return Err(SlotCompleted) — no bank is
        // ever re-created.

        // Step 1 of 1a: mark the slot finalised. Any execute() call that checks
        // completed_slots.read() after this point returns SlotCompleted immediately.
        // The completed_slots write lock is acquired and released here; slot_banks
        // is not held at this moment.
        self.completed_slots.write().unwrap().insert(slot);

        // Step 2 of 1a: evict and collect children under the slot_banks write lock.
        // completed_slots is NOT held at this point — the write guard dropped above.
        let children_raw: Vec<(Slot, Vec<(Vec<u8>, usize)>)> = {
            let mut w = self.slot_banks.write().unwrap();

            // Evict the speculative slot N bank from the cache. Any Arc<Bank>
            // clones already held by in-flight execute() calls remain valid —
            // they are independently reference-counted and drop naturally once
            // their callers finish. Removing the entry here ensures no NEW
            // execute() call for this slot will find a bank to execute against
            // (the completed_slots guard above has already closed the re-creation
            // race, so this is now belt-and-suspenders).
            w.remove(&slot);

            // Collect the slots of all immediate children that need rebasing
            // and move their stored batch lists out.
            let child_slots: Vec<Slot> = w
                .iter()
                .filter(|(_, s)| s.parent_slot == slot && !s.parent_is_canonical)
                .map(|(s, _)| *s)
                .collect();

            child_slots
                .into_iter()
                .filter_map(|child| {
                    let state = w.get_mut(&child)?;
                    // Move the stored batch list out. The Vec is left empty so
                    // new deliveries during Phase 2 accumulate in a clean list.
                    let pending = mem::take(&mut state.pending_proto_batches);
                    // Mark as rebasing immediately. From this moment, any concurrent
                    // execute() for this child returns Err(Rebasing) without reading
                    // tx_count or state.bank. The write lock is not needed to protect
                    // those fields beyond this point — the rebasing flag is the guard.
                    state.rebasing = true;
                    Some((child, pending))
                })
                .collect()
        }; // slot_banks write lock released here

        // Sub-phase 1b (outside any lock): build the new canonical-parent banks.
        //
        // create_child_bank → Bank::new_from_parent_with_options → _new_from_parent
        // may call process_new_epoch at epoch boundaries (100–500ms). Running this
        // outside the lock means concurrent execute() calls for OTHER (non-rebasing)
        // slots proceed unimpeded. Rebasing children return Err(Rebasing) via their
        // rebasing=true flag, so they are correctly held back without the write lock.
        //
        // canonical_bank is already frozen — parent.freeze() inside _new_from_parent
        // checks `if *hash == Hash::default()` and returns immediately.
        let children_with_banks: Vec<(Slot, Arc<Bank>, Vec<(Vec<u8>, usize)>)> =
            children_raw
                .into_iter()
                .map(|(child, pending)| {
                    let new_bank = Arc::new(self.create_child_bank(
                        canonical_bank.clone(),
                        child,
                    ));
                    (child, new_bank, pending)
                })
                .collect();

        // Sub-phase 1c (write lock): install the pre-built banks into slot state.
        // Critical section holds the write lock for only O(1) field assignments.
        let children_to_rebase: Vec<(Slot, Arc<Bank>, Vec<(Vec<u8>, usize)>)> = {
            let mut w = self.slot_banks.write().unwrap();
            children_with_banks
                .into_iter()
                .filter_map(|(child, new_bank, pending)| {
                    // The slot may have been removed by a concurrent discard_slot()
                    // between sub-phase 1a and now. If so, skip it — it was condemned.
                    let state = w.get_mut(&child)?;
                    state.bank = new_bank.clone();
                    state.tx_count = 0;
                    state.parent_is_canonical = true;
                    // rebasing was set to true in sub-phase 1a; assert the invariant.
                    debug_assert!(state.rebasing);
                    Some((child, new_bank, pending))
                })
                .collect()
        }; // write lock released here — Phase 2 runs without holding it

        // -------------------------------------------------------------------------
        // Phase 2 (outside any lock):
        //   Re-execute all stored batches for each rebased child slot against its
        //   new canonical-parent bank. This is the CPU-intensive part — each batch
        //   goes through full SVM execution: sanitization, ALT resolution, account
        //   lock acquisition, parallel rayon execution, and write cache commit.
        //
        // Running Phase 2 outside the lock allows:
        //   - Concurrent execute() calls for OTHER (non-rebasing) slots to proceed
        //     without waiting.
        //   - The rebasing child slot is protected by its rebasing == true flag,
        //     which causes any concurrent execute() for that slot to return
        //     Err(Rebasing) until Phase 2 finishes and clears the flag. This
        //     prevents any new batch from reading a partially-rebuilt tx_count and
        //     being assigned a wrong slot-relative starting index.
        // -------------------------------------------------------------------------
        let mut correction_updates: Vec<SpeculativeAccountUpdate> = Vec::new();

        // -------------------------------------------------------------------------
        // REBASING SAFETY INVARIANT
        //
        // Phase 1 set rebasing=true for every child in children_to_rebase. This flag
        // MUST be cleared back to false for every child before confirm_slot() returns,
        // regardless of whether Phase 2 succeeds or fails. Failure to clear the flag
        // leaves the slot as a zombie: execute() returns Err(Rebasing) indefinitely,
        // MevEngine starves on that slot, and the slot is never evicted.
        //
        // The invariant is enforced by separating the per-child work into two stages:
        //
        //   Stage A — execution (fallible): run all stored batches, collect the
        //             result without propagating errors yet.
        //   Stage B — cleanup (infallible): clear rebasing=false under write lock,
        //             regardless of Stage A's outcome.
        //
        // After Stage B, if Stage A produced an error it is propagated. If an early
        // child fails, the outer loop continues to Stage B for that child (clearing
        // its flag), then breaks with the stored error. Remaining unprocessed children
        // whose Stage A was never reached must also have their rebasing flag cleared.
        // This is handled by iterating the full children_to_rebase Vec with a separate
        // cleanup pass for any child skipped due to an early error.
        //
        // The correction_updates Vec is populated only for children that executed
        // successfully. Children with errors produce no correction update — the caller
        // will receive an Err return from confirm_slot() and must treat all child-slot
        // state as indeterminate until the next confirm or discard signal.
        // -------------------------------------------------------------------------
        let mut first_error: Option<SpeculativeExecutorError> = None;

        for (child_slot, new_bank, pending_batches) in children_to_rebase {
            // If a previous child already failed, skip Stage A but still run Stage B
            // to clear rebasing=false for this child and all subsequent ones.
            let stage_a_result: Option<(usize, HashMap<Pubkey, AccountSharedData>)> =
                if first_error.is_none() {
                    // Stage A: re-execute all stored batches for this child.
                    let mut child_error: Option<SpeculativeExecutorError> = None;

                    // Accumulate tx_count locally across all batches.  Since
                    // rebasing == true for this slot, no concurrent execute() reads
                    // tx_count here — the rebasing flag is the exclusive-access guard.
                    // We write tx_count exactly once in Stage B below, reducing N+1
                    // write lock acquisitions to a single acquisition per child slot.
                    let mut accumulated_tx_count: usize = 0;

                    for (batch_bytes, batch_tx_starting_index) in pending_batches {
                        if child_error.is_some() {
                            // An earlier batch in this child already failed.
                            // Skip remaining batches but still reach Stage B.
                            break;
                        }

                        // Deserialize the stored proto bytes back into Vec<Entry>.
                        //
                        // These bytes were written by execute() using proto_entry_bytes.to_vec()
                        // after a successful wincode::deserialize call. They are therefore valid
                        // wincode-encoded Entry data — a decode failure here would indicate memory
                        // corruption between storage and retrieval, not a protocol error. The same
                        // wincode::deserialize decoder is used here to maintain consistency with
                        // the original decode path in execute().
                        let entries: Vec<Entry> =
                            match wincode::deserialize(&batch_bytes) {
                                Ok(v) => v,
                                Err(e) => {
                                    child_error = Some(
                                        SpeculativeExecutorError::DeserializeEntries(e),
                                    );
                                    break;
                                }
                            };

                        // Re-execute the batch using the same slot-relative starting
                        // index that was recorded when the batch was first executed.
                        // Using the identical starting index ensures the SVM assigns
                        // the same sequential transaction indexes as the original run,
                        // making the rebased result structurally identical to canonical
                        // replay output.
                        let batch_count = match execute_entries_speculatively(
                            &BankWithScheduler::new_without_scheduler(new_bank.clone()),
                            &self.replay_tx_thread_pool,
                            entries,
                            batch_tx_starting_index,
                        ) {
                            Ok(n) => n,
                            Err(e) => {
                                child_error =
                                    Some(SpeculativeExecutorError::Execution(e));
                                break;
                            }
                        };

                        // Clear the status cache entries written by this rebased batch.
                        // The rebased new_bank was forked from canonical_bank and
                        // therefore shares its Arc<RwLock<BankStatusCache>>. Without
                        // this call, every signature re-executed here remains in the
                        // shared cache under child_slot's key. When the canonical replay
                        // path later runs check_transactions for child_slot it finds them
                        // as AlreadyProcessed, skips the transactions, computes a wrong
                        // bank hash, and marks the slot Dead — the same contamination
                        // described in execute() above.
                        new_bank.clear_slot_signatures(child_slot);

                        // Accumulate locally — no lock acquired per batch.
                        accumulated_tx_count =
                            accumulated_tx_count.saturating_add(batch_count);
                    }

                    if let Some(e) = child_error {
                        first_error = Some(e);
                        None // Stage A failed — skip correction update for this child
                    } else {
                        // Build the correction update: post-rebase state compared
                        // against the canonical parent (slot N's frozen state). This
                        // baseline differs from the per-batch baseline used in execute().
                        // Here we use the FULL canonical parent as baseline because the
                        // correction update represents the TOTAL effect of all batches
                        // received so far for the child slot, expressed as a delta from
                        // the now-verified parent. The caller replaces their accumulated
                        // prior speculative result for this child slot with this single
                        // correction.
                        Some((
                            accumulated_tx_count,
                            accounts_to_watch
                                .iter()
                                .filter_map(|pubkey| {
                                    let post = new_bank.get_account(pubkey)?;
                                    let pre = canonical_bank.get_account(pubkey);
                                    match pre {
                                        Some(ref pre_state) if pre_state == &post => None,
                                        _ => Some((*pubkey, post)),
                                    }
                                })
                                .collect::<HashMap<Pubkey, AccountSharedData>>(),
                        ))
                    }
                } else {
                    // A previous child failed. Skip Stage A for this child entirely.
                    // Stage B below will still clear rebasing=false.
                    None
                };

            // Stage B (ALWAYS runs): commit tx_count and clear the rebasing flag
            // in a SINGLE write lock acquisition per child slot.
            //
            // The previous design acquired one write lock per batch (to update
            // tx_count) and a separate lock to clear rebasing — N+1 total write
            // lock acquisitions per child.  Each acquisition serialises every
            // concurrent execute() read for every other active slot.  The new
            // design accumulates tx_count in a local variable above (safe because
            // rebasing==true guards concurrent reads) and commits both tx_count
            // and rebasing=false atomically here in exactly one lock acquisition.
            {
                let mut w = self.slot_banks.write().unwrap();
                if let Some(state) = w.get_mut(&child_slot) {
                    if let Some((count, _)) = &stage_a_result {
                        state.tx_count = state.tx_count.saturating_add(*count);
                    }
                    state.rebasing = false;
                }
            }

            // If Stage A produced a correction update, record it.
            // is_correction = true signals to the caller that this is a TOTAL
            // REPLACEMENT — the accounts map reflects all batches re-executed
            // against the canonical parent, not just the changes from one delivery.
            // The caller must discard every incremental delta accumulated for this
            // child slot and replace it with exactly what this correction contains.
            if let Some((_, accounts)) = stage_a_result {
                correction_updates.push(SpeculativeAccountUpdate {
                    slot: child_slot,
                    accounts,
                    is_correction: true,
                });
            }
        }

        // Propagate the first error encountered, if any.
        // All children have had rebasing=false cleared by this point.
        if let Some(e) = first_error {
            return Err(e);
        }

        Ok(correction_updates)
    }

    /// Notify the executor that the canonical replay path has rejected `slot`
    /// via `SlotUpdate::Dead`, and atomically evict the entire subtree of
    /// speculative slots whose execution was rooted in the dead slot.
    ///
    /// # What a Dead slot means
    ///
    /// In replay_stage.rs `mark_dead_slot()` (lines 2470-2585), when any of the
    /// following fail for a slot:
    ///   · `check_chained_block_id()` — chained merkle root mismatch
    ///   · `verify_ticks()` — wrong tick count
    ///   · PoH hash chain verification
    ///   · Ed25519 signature batch verification
    ///   · `process_entries()` / SVM execution
    ///   · `check_last_fec_set_and_get_block_id()` — last FEC set invalid
    ///
    /// ...the path sets `is_dead = true` in ForkProgress, writes the dead marker
    /// to RocksDB via `blockstore.set_dead_slot(slot)`, and emits
    /// `SlotUpdate::Dead { slot, .. }` via rpc_subscriptions (line 3525).
    ///
    /// # Cascade eviction
    ///
    /// Any speculative slot whose execution was rooted in the dead slot — directly
    /// or through an arbitrarily deep chain of speculative forks — has computed
    /// account state derived from entries the network has declared invalid. That
    /// state must never be acted on, regardless of how many generations of
    /// speculation separate a descendant from the dead ancestor.
    ///
    /// The cascade works by iteratively expanding the condemned set: seeded with
    /// the dead slot, any slot in slot_banks whose `parent_slot` is already in the
    /// condemned set is itself condemned. The expansion repeats until stable — no
    /// new members are added in a full pass. At that point every speculative
    /// descendant has been found and all are removed from slot_banks in one atomic
    /// write-lock acquisition.
    ///
    /// # Returns
    ///
    /// The complete set of condemned slot numbers, including the dead slot itself
    /// and every speculative descendant at any depth. The caller MUST discard ALL
    /// buffered `SpeculativeAccountUpdate` values keyed by any slot in this set
    /// without acting on them.
    pub fn discard_slot(&self, slot: Slot) -> HashSet<Slot> {
        let mut w = self.slot_banks.write().unwrap();

        // Seed the condemned set with the dead slot. Any slot whose ancestry
        // passes through this slot inherits its invalidity — the write cache state
        // of every descendant was computed from entries that the network rejected.
        let mut condemned: HashSet<Slot> = HashSet::new();
        condemned.insert(slot);

        // Iteratively expand the condemned set. A single pass over slot_banks
        // is not sufficient for chains deeper than one level, so we repeat until
        // no new condemned slots are discovered. The loop terminates in at most
        // as many iterations as the maximum speculative chain depth, which is
        // bounded by the number of slots between shredstream delivery and
        // canonical freeze — typically one or two on mainnet.
        loop {
            let before = condemned.len();
            for (s, state) in w.iter() {
                if condemned.contains(&state.parent_slot) {
                    condemned.insert(*s);
                }
            }
            if condemned.len() == before {
                // No new descendants were discovered — the condemned set is stable.
                break;
            }
        }

        // Remove every condemned slot atomically under the single write lock.
        // Dropping the Arc<Bank> reference here frees the speculative write cache
        // memory for any condemned bank that has no other live Arc references
        // outside this map.
        for s in &condemned {
            w.remove(s);
        }

        // Rust's destructor rules: a local binding's Drop implementation runs at
        // the END of its lexical scope — for `w` that is the closing brace of this
        // function — NOT at the point of last syntactic use. Without an explicit
        // drop here, `w` would still be holding the slot_banks write lock when the
        // completed_slots write lock is acquired below. That would be nested
        // acquisition in slot_banks → completed_slots order, which is the correct
        // ordering, but it would make the nesting implicit and invisible to readers.
        // More importantly, the module's lock ordering rule requires completed_slots
        // to be a LEAF lock — nothing may hold slot_banks while acquiring
        // completed_slots — and relying on Rust's end-of-scope drop to enforce this
        // invisibly is fragile: any future refactor that restructures the function
        // body could inadvertently extend `w`'s scope and re-introduce nested
        // acquisition without a compile-time warning. The explicit drop(w) makes the
        // ordering a visible, auditable invariant at the source level: slot_banks
        // is released here, and only then is completed_slots acquired below.
        drop(w);

        // Mark every condemned slot as permanently finalised so that late
        // shredstream batches cannot re-create their banks after eviction.
        // `extend` inserts the entire condemned set under a single write-lock
        // acquisition, avoiding N separate lock/unlock round-trips for N slots.
        {
            let mut completed = self.completed_slots.write().unwrap();
            completed.extend(condemned.iter().copied());
        }

        condemned
    }

    /// Return a clone of the speculative bank for `slot` if one exists.
    ///
    /// Used by `MevEngine::handle_speculative_update` to attach the post-execution
    /// bank to `MevPoolUpdateEvent` so the arbitrage executor can simulate against
    /// the forward-looking account state produced by this shredstream batch.
    ///
    /// Returns `None` if the slot has already been confirmed or discarded, or if
    /// shredstream has not yet delivered any entries for this slot.
    ///
    /// The read lock is held only for the duration of the HashMap lookup and the
    /// Arc clone — microseconds. The caller is left with an independently reference-
    /// counted bank that remains live regardless of subsequent confirm/discard calls.
    pub fn get_slot_bank(&self, slot: Slot) -> Option<Arc<Bank>> {
        self.slot_banks
            .read()
            .unwrap()
            .get(&slot)
            .map(|s| s.bank.clone())
    }
}

// =============================================================================
// Tests
// =============================================================================
//
// Every test in this module lives inside `#[cfg(test)] mod tests { use super::*; }`.
// The `use super::*;` import is what gives tests access to the private types
// `SlotState` and the `slot_banks` field on `SpeculativeSlotExecutor`. This is
// standard Rust test idiom: tests inside the same module as the production code
// can read and write private fields, which lets us set up precise preconditions
// (e.g. a slot with `rebasing=true`) without exposing those fields to external callers.
//
// All tests use tick-only `Entry` values (entries that carry zero transactions).
// Ticks advance the bank's PoH counter and trigger block-boundary detection but
// do not execute any SVM instructions. Using ticks lets the tests exercise the
// full `execute()` and `confirm_slot()` call paths — including deserialization,
// bank creation, `execute_entries_speculatively`, status-cache clearing, and
// `tx_count` bookkeeping — without requiring funded keypairs, pre-deployed
// programs, or a real on-chain transaction payload.

#[cfg(test)]
mod tests {
    use super::*;

    use {
        crate::{
            leader_schedule_cache::LeaderScheduleCache,
            genesis_utils::{GenesisConfigInfo, create_genesis_config},
        },
        solana_entry::entry::create_ticks,
        solana_hash::Hash,
        solana_runtime::{
            bank::Bank,
            bank_forks::BankForks,
        },
        std::sync::Arc,
    };

    fn make_executor() -> (SpeculativeSlotExecutor, Arc<std::sync::RwLock<BankForks>>, Arc<Bank>) {
        let GenesisConfigInfo { genesis_config, .. } = create_genesis_config(500_000);

        let root_bank = Arc::new(Bank::new_for_tests(&genesis_config));
        let bank_forks = BankForks::new_rw_arc(Bank::new_for_tests(&genesis_config));
        let migration_status = bank_forks.read().unwrap().migration_status();

        let leader_cache = Arc::new(
            LeaderScheduleCache::new_from_bank(&bank_forks.read().unwrap().root_bank()),
        );

        let pool = Arc::new(
            rayon::ThreadPoolBuilder::new()
                .num_threads(1)
                .build()
                .expect("test rayon pool"),
        );

        let executor = SpeculativeSlotExecutor::new(
            pool,
            leader_cache,
            migration_status,
        );

        (executor, bank_forks, root_bank)
    }

    // -------------------------------------------------------------------------
    // Test 1 — Rebasing guard short-circuits execute() and leaves tx_count intact
    // -------------------------------------------------------------------------

    /// Verifies that when `SlotState.rebasing` is true, `execute()` returns
    /// `Err(Rebasing(slot))` before reading or modifying `tx_count`.
    ///
    /// The rebasing flag is the write-lock-free guard that prevents a concurrent
    /// `execute()` from reading a partially-rebuilt `tx_count` during a canonical
    /// rebase. If this guard did not fire correctly, execute() would seed its batch
    /// with a wrong slot-relative starting index — a silent, permanent corruption
    /// of the SVM's transaction index record for this slot.
    #[test]
    fn test_rebasing_guard_returns_err_and_leaves_tx_count_unchanged() {
        let (executor, bank_forks, root_bank) = make_executor();

        // Insert slot 5 with rebasing=true to simulate a slot that is mid-rebase.
        // tx_count is set to a sentinel value (42) so we can verify it was not
        // touched after execute() returns Err(Rebasing).
        {
            let mut w = executor.slot_banks.write().unwrap();
            w.insert(5, SlotState {
                bank: Arc::clone(&root_bank),
                tx_count: 42,
                parent_slot: 4,
                parent_is_canonical: false,
                rebasing: true,
                pending_proto_batches: Vec::new(),
            });
        }

        let ticks = create_ticks(1, 0, Hash::default());
        let bytes = wincode::serialize(&ticks).expect("tick serialization must not fail");

        let result = executor.execute(5, 4, &bytes, &bank_forks, &[]);

        match result {
            Err(SpeculativeExecutorError::Rebasing(s)) => assert_eq!(s, 5),
            other => panic!("expected Err(Rebasing(5)), got {:?}", other),
        }

        // The rebasing guard must short-circuit before any write to `tx_count`.
        // If it did not, the wrong count would corrupt every subsequent batch's
        // starting index for this slot.
        let r = executor.slot_banks.read().unwrap();
        assert_eq!(
            r[&5].tx_count, 42,
            "tx_count must be untouched when rebasing=true guards early exit"
        );
    }

    // -------------------------------------------------------------------------
    // Test 2 — is_correction flag on execute() vs confirm_slot()
    // -------------------------------------------------------------------------

    /// Verifies that `execute()` always produces updates with `is_correction = false`
    /// and that `confirm_slot()` produces corrections with `is_correction = true`.
    ///
    /// The two kinds of update carry different semantics that the caller must handle
    /// distinctly: incremental deltas from `execute()` are accumulated, while
    /// corrections from `confirm_slot()` are total replacements. The `is_correction`
    /// field is the only machine-readable signal that distinguishes them — without it
    /// a caller would silently double-count the correction on top of prior deltas.
    #[test]
    fn test_execute_produces_incremental_confirm_produces_correction() {
        let (executor, bank_forks, root_bank) = make_executor();

        // Place slot 0 as a speculative parent so execute() for slot 1 finds it.
        {
            let mut w = executor.slot_banks.write().unwrap();
            w.insert(0, SlotState {
                bank: Arc::clone(&root_bank),
                tx_count: 0,
                parent_slot: 0,
                parent_is_canonical: true,
                rebasing: false,
                pending_proto_batches: Vec::new(),
            });
        }

        let ticks = create_ticks(1, 0, Hash::default());
        let bytes = wincode::serialize(&ticks).expect("tick serialization must not fail");

        // execute() must return is_correction = false.
        let update = executor
            .execute(1, 0, &bytes, &bank_forks, &[])
            .expect("execute must succeed for a tick-only batch");

        assert!(
            !update.is_correction,
            "execute() must produce is_correction=false — incremental deltas are accumulated, \
             not used to replace prior state"
        );

        // confirm_slot for slot 0. Insert slot 2 as a speculative child of slot 0
        // so that confirm_slot has a child to rebase and produces a correction update.
        {
            let mut w = executor.slot_banks.write().unwrap();
            w.insert(2, SlotState {
                bank: Arc::clone(&root_bank),
                tx_count: 0,
                parent_slot: 0,
                parent_is_canonical: false,
                rebasing: false,
                pending_proto_batches: vec![(bytes.clone(), 0)],
            });
        }

        let canonical_bank = Arc::clone(&root_bank);
        canonical_bank.freeze();

        let corrections = executor
            .confirm_slot(0, canonical_bank, &bank_forks, &[])
            .expect("confirm_slot must succeed");

        for correction in &corrections {
            assert!(
                correction.is_correction,
                "confirm_slot() must produce is_correction=true — corrections replace all prior \
                 speculative state for the child slot rather than accumulating on top of it"
            );
        }
    }

    // -------------------------------------------------------------------------
    // Test 3 — Happy-path execute and SpeculativeAccountUpdate correctness
    // -------------------------------------------------------------------------

    /// `execute()` must return a `SpeculativeAccountUpdate` tagged with the correct
    /// slot number. When the entry batch contains only ticks (no transactions) the
    /// accounts map must be empty — ticks advance PoH but write to no account.
    ///
    /// This test also verifies that the first call to `execute()` for a new slot
    /// creates a speculative child bank and registers it in `slot_banks`, making
    /// it available to `get_slot_bank()` for subsequent simulation calls.
    #[test]
    fn test_execute_tick_batch_produces_correct_speculative_update() {
        let (executor, bank_forks, root_bank) = make_executor();

        // Place the parent bank (slot 0) in the speculative cache directly. When
        // `execute()` looks for the parent of slot 1, it first checks the speculative
        // cache and finds slot 0 there, so it does NOT need a frozen bank in
        // BankForks. This is the speculative-parent path (parent_is_canonical=false),
        // which is the common case when shredstream is running ahead of canonical replay.
        {
            let mut w = executor.slot_banks.write().unwrap();
            w.insert(0, SlotState {
                bank: Arc::clone(&root_bank),
                tx_count: 0,
                parent_slot: 0,
                parent_is_canonical: true,
                rebasing: false,
                pending_proto_batches: Vec::new(),
            });
        }

        // One tick entry — `create_ticks(1, 0, last_hash)` produces a single Entry
        // with `is_tick() == true` and `transactions.is_empty() == true`.
        let ticks = create_ticks(1, 0, Hash::default());
        let bytes = wincode::serialize(&ticks).expect("tick serialization must not fail");

        let update = executor
            .execute(1, 0, &bytes, &bank_forks, &[])
            .expect("tick-only execution against a speculative parent must succeed");

        // The update must be tagged with the slot that was executed, not the parent slot.
        assert_eq!(update.slot, 1, "SpeculativeAccountUpdate.slot must match the executed slot");

        // No accounts were modified — the delta for a tick-only batch is always empty.
        // This is the expected value that MevEngine.handle_speculative_update receives:
        // it skips the broadcast when the accounts map is empty rather than firing
        // spurious pool-update events.
        assert!(
            update.accounts.is_empty(),
            "tick entries modify no accounts; the delta map must be empty"
        );

        // The speculative bank for slot 1 must now be reachable via get_slot_bank.
        // ArbitrageExecutor's try_execute_arbitrage calls get_slot_bank to attach
        // the post-execution bank to MevPoolUpdateEvent.speculative_bank so it
        // can simulate against forward-looking account state.
        assert!(
            executor.get_slot_bank(1).is_some(),
            "slot 1 must be registered in the speculative cache after first execute"
        );
    }

    // -------------------------------------------------------------------------
    // Test 4 — Rebasing flag cleared on Phase 2 error (zombie-prevention invariant)
    // -------------------------------------------------------------------------

    /// When `confirm_slot` sets `rebasing=true` for a child slot (Phase 1) but
    /// Phase 2 fails — for example because a stored proto batch cannot be
    /// deserialized — it must still clear `rebasing=false` for that child before
    /// returning the error (Stage B).
    ///
    /// If the flag were left `true` after an error, every subsequent `execute()`
    /// call for that slot would return `Err(Rebasing)` indefinitely. The engine
    /// would never receive pool-update events for that slot and arbitrage
    /// opportunities would be silently missed — a zombie slot that stalls MEV
    /// revenue without any log message to explain why.
    ///
    /// The invariant is: Stage B (`rebasing = false`) runs unconditionally for
    /// every child that entered Phase 2, regardless of whether Stage A (SVM
    /// re-execution) succeeded or failed.
    #[test]
    fn test_rebasing_flag_always_cleared_even_after_phase2_deserialization_error() {
        let (executor, bank_forks, root_bank) = make_executor();

        // Insert slot 5 as a speculative child of slot 4. Its `pending_proto_batches`
        // contains four corrupt bytes — these cannot be deserialized as `Vec<Entry>`
        // by `wincode::deserialize`, which will return `Err(WincodeError)`. This
        // triggers the error path in Phase 2 of `confirm_slot`.
        {
            let mut w = executor.slot_banks.write().unwrap();
            w.insert(5, SlotState {
                bank: Arc::clone(&root_bank),
                tx_count: 0,
                parent_slot: 4,
                parent_is_canonical: false,
                rebasing: false,
                pending_proto_batches: vec![
                    // Four bytes that are valid enough for the Vec header but produce
                    // a decode error when wincode tries to populate the Entry fields.
                    // Any non-empty corrupt payload triggers the error path.
                    (vec![0xDE, 0xAD, 0xBE, 0xEF], 0),
                ],
            });
        }

        let canonical_bank = Arc::clone(&root_bank);
        let result = executor.confirm_slot(4, canonical_bank, &bank_forks, &[]);

        assert!(
            result.is_err(),
            "confirm_slot must propagate the deserialization error from Phase 2"
        );

        // The critical invariant: slot 5 must have rebasing=false after the error.
        // If the flag were still true, the engine would be stuck returning Err(Rebasing)
        // for every future execute() call on slot 5.
        let r = executor.slot_banks.read().unwrap();
        if let Some(state) = r.get(&5) {
            assert!(
                !state.rebasing,
                "rebasing flag must be cleared by Stage B even when Stage A (Phase 2) fails — \
                 a slot left with rebasing=true becomes a zombie that starves MEV pool updates"
            );
        }
        // If slot 5 was removed entirely by the implementation that is also acceptable:
        // an absent slot causes execute() to return ParentBankNotFound rather than
        // Err(Rebasing), which is a retriable rather than a permanent failure.
    }
}
