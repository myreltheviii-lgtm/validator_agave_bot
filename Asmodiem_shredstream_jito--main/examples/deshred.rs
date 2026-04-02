// ────────────────────────────────────────────────────────────────────────────
// deshred — verbose diagnostic client for the ShredStream proxy gRPC service.
//
// The ShredStream proxy receives raw Solana shreds from Jito's Block Engine,
// runs Reed-Solomon FEC recovery to fill any missing data shreds, reassembles
// the recovered shred payloads into Solana Entry objects, and then broadcasts
// those entries over a local gRPC streaming endpoint.  This client connects to
// that endpoint and prints a deeply annotated trace of every message so you can
// observe the actual arrival order, slot continuity, entry composition, and
// transaction program mix in real time.
//
// Run with:
//   cargo run --example deshred
//
// Prerequisites:
//   shredstream-proxy must already be running with --grpc-service-port 9999
// ────────────────────────────────────────────────────────────────────────────

use std::{
    collections::HashSet,
    time::Instant,
};

use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient,
    SubscribeEntriesRequest,
};
use solana_entry::entry::Entry;
use solana_sdk::message::VersionedMessage;

// ── Well-known Solana program addresses ──────────────────────────────────────
// Rather than printing the full 32-byte base58 address of every program invoked
// in a transaction instruction, we map the most common ones to a short readable
// label.  Any address not in this table falls back to the first 8 characters of
// its base58 string so the output stays scannable without losing identity.
fn program_label(address: &str) -> String {
    match address {
        "11111111111111111111111111111111"                       => "System".into(),
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf8Ss623VQ5DA"         => "Token".into(),
        "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"         => "Token2022".into(),
        "ComputeBudget111111111111111111111111111111"            => "ComputeBudget".into(),
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJe1bRS"        => "ATA".into(),
        "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"        => "Jupiter".into(),
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"        => "RaydiumAMM".into(),
        "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK"        => "RaydiumCLMM".into(),
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"         => "Orca".into(),
        "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY"        => "Phoenix".into(),
        "Vote111111111111111111111111111111111111111p"           => "Vote".into(),
        "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"         => "Memo".into(),
        "Stake11111111111111111111111111111111111111"            => "Stake".into(),
        "SysvarRent111111111111111111111111111111111"            => "SysvarRent".into(),
        "SysvarC1ock11111111111111111111111111111111"            => "SysvarClock".into(),
        "So1endDqqlbe3uRmrCjFLMnhQ8GgTCnFxbG5DBAP7z"          => "Solend".into(),
        "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP"       => "OrcaV2".into(),
        other => {
            // For unknown programs, show just the first 8 characters of the
            // base58 address followed by ".." so you can still visually
            // identify it and look it up in a block explorer if needed.
            format!("{}..", other.chars().take(8).collect::<String>())
        }
    }
}

// ── Vote transaction detection ────────────────────────────────────────────────
// Validators produce a vote transaction for every slot they observe.  On
// mainnet this accounts for roughly 60-70% of all transactions by count.
// Because they carry no MEV signal and would completely dominate the log, we
// detect them and collapse consecutive vote transactions into a single summary
// line rather than printing each one individually.
fn is_vote_tx(tx: &solana_sdk::transaction::VersionedTransaction) -> bool {
    let keys = tx.message.static_account_keys();
    tx.message
        .instructions()
        .iter()
        .any(|ix| {
            keys.get(ix.program_id_index as usize)
                .map(|k| k.to_string() == "Vote111111111111111111111111111111111111111p")
                .unwrap_or(false)
        })
}

// ── Slot ordering classifier ──────────────────────────────────────────────────
// The proxy pushes entry groups as soon as each FEC-set is recovered, so the
// client receives multiple messages per slot (each message covers a different
// range of data shreds within that slot).  Between slots the proxy does NOT
// guarantee strict monotone delivery because FEC recovery for an older slot can
// complete after a newer one if shreds arrived late from the network.  This
// enum names every possible relationship between the current and previous slot.
#[derive(Debug)]
enum SlotOrdering {
    First,                       // no previous slot seen yet — very first message
    SameSlot,                    // same slot number, this is the next FEC group within it
    Sequential,                  // slot == prev + 1 — perfect forward chain
    Gap        { skipped: u64 }, // slot > prev + 1 — one or more slots were never delivered
    OutOfOrder { behind:  u64 }, // slot < prev     — late FEC recovery completed for a past slot
}

fn classify_ordering(slot: u64, prev: Option<u64>) -> SlotOrdering {
    match prev {
        None                      => SlotOrdering::First,
        Some(p) if slot == p      => SlotOrdering::SameSlot,
        Some(p) if slot == p + 1  => SlotOrdering::Sequential,
        Some(p) if slot >  p + 1  => SlotOrdering::Gap        { skipped: slot - p - 1 },
        Some(p)                   => SlotOrdering::OutOfOrder  { behind:  p    - slot  },
    }
}

// ── Session-level statistics ──────────────────────────────────────────────────
// Accumulated across the lifetime of the stream so we can print a rolling
// health summary every SUMMARY_EVERY messages and a final summary on exit.
#[derive(Default)]
struct Stats {
    total_messages:     u64,
    total_entries:      u64,
    // A tick entry carries no transactions and only advances the Proof-of-History
    // clock by hashing num_hashes times from the previous entry's hash.
    // A data entry carries transactions AND a PoH step that commits to them.
    total_tick_entries: u64,
    total_data_entries: u64,
    total_txns:         u64,
    total_vote_txns:    u64,
    total_nonvote_txns: u64,
    out_of_order_count: u64,
    gap_count:          u64,
    deserialize_errors: u64,
}

impl Stats {
    fn print_summary(&self, elapsed_secs: f64) {
        println!();
        println!("  ┌─ CUMULATIVE STATS ─────────────────────────────────────────");
        println!("  │  messages          : {}", self.total_messages);
        println!("  │  entries total     : {}", self.total_entries);
        println!("  │    tick entries    : {}", self.total_tick_entries);
        println!("  │    data entries    : {}", self.total_data_entries);
        println!("  │  txns total        : {}", self.total_txns);
        println!("  │    vote txns       : {}", self.total_vote_txns);
        println!("  │    non-vote txns   : {}", self.total_nonvote_txns);
        println!("  │  slot gaps         : {}", self.gap_count);
        println!("  │  out-of-order msgs : {}", self.out_of_order_count);
        println!("  │  deser errors      : {}", self.deserialize_errors);
        println!("  │  session uptime    : {elapsed_secs:.1}s");
        println!("  └────────────────────────────────────────────────────────────");
        println!();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── Connect ───────────────────────────────────────────────────────────────
    // tonic will attempt a single TCP connection here.  If the proxy is not
    // listening on port 9999 this panics immediately — there is nothing to
    // observe if the data source is absent, so fast-fail is the correct behaviour.
    println!("[INIT] Connecting to shredstream proxy at http://127.0.0.1:9999 ...");
    let mut client = ShredstreamProxyClient::connect("http://127.0.0.1:9999")
        .await
        .expect("[FATAL] Cannot connect. Ensure the proxy is running with --grpc-service-port 9999");
    println!("[INIT] Connected successfully.\n");

    // ── Open the streaming RPC ────────────────────────────────────────────────
    // SubscribeEntries opens a server-streaming gRPC call.  The proxy keeps
    // this channel open indefinitely and pushes a new PbEntry message each time
    // a batch of data shreds for a slot has been fully reassembled by the deshred
    // pipeline inside the proxy.  The stream ends only when the proxy shuts down.
    let mut stream = client
        .subscribe_entries(SubscribeEntriesRequest {})
        .await
        .expect("[FATAL] subscribe_entries RPC failed — verify --grpc-service-port is set on the proxy")
        .into_inner();

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  STREAM OPEN — waiting for entries");
    println!("  A single Solana slot produces several messages here because the");
    println!("  proxy delivers entries as each FEC set is independently decoded.");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut stats     = Stats::default();
    let mut last_slot: Option<u64> = None;
    let session_start = Instant::now();
    let mut last_msg_at = Instant::now();

    // Print a cumulative health summary every this many messages so you can
    // assess gap rate, out-of-order rate, and error rate without scrolling.
    const SUMMARY_EVERY: u64 = 50;

    // ── Main receive loop ─────────────────────────────────────────────────────
    // stream.message() suspends the async task until the next protobuf frame
    // arrives from the proxy.  Returning None means the proxy closed the stream
    // cleanly (e.g. graceful shutdown via SIGTERM).  Returning Err means a
    // transport-level failure.  Both are treated as terminal here.
    while let Some(pb) = stream
        .message()
        .await
        .expect("[FATAL] Stream error — the proxy likely crashed or dropped the connection")
    {
        let now          = Instant::now();
        let since_last   = now.duration_since(last_msg_at);
        let session_secs = now.duration_since(session_start).as_secs_f64();
        last_msg_at      = now;

        stats.total_messages += 1;

        let slot        = pb.slot;
        let parent_slot = pb.parent_slot;
        let raw_bytes   = pb.entries.len();

        // ── Slot ordering analysis ────────────────────────────────────────
        // Classify the relationship between this slot and the previous one.
        // The tag is appended to the message header line so you can scan the
        // log vertically and spot gaps or out-of-order arrivals at a glance.
        let ordering = classify_ordering(slot, last_slot);
        let ordering_tag: String = match &ordering {
            SlotOrdering::First                  => "  [FIRST MESSAGE]".into(),
            SlotOrdering::SameSlot               => "  [SAME SLOT — next FEC group]".into(),
            SlotOrdering::Sequential             => "  [SEQUENTIAL ✓]".into(),
            SlotOrdering::Gap        { skipped } => {
                stats.gap_count += 1;
                format!("  [GAP — {skipped} slot(s) never delivered ⚠]")
            }
            SlotOrdering::OutOfOrder { behind }  => {
                stats.out_of_order_count += 1;
                format!("  [OUT OF ORDER — {behind} slot(s) behind current ⚠]")
            }
        };

        // ── Parent slot continuity check ──────────────────────────────────
        // Each slot declares its parent in the block header.  When the stream
        // advances to a new slot number, the declared parent_slot in the proto
        // message should equal the previous slot we processed.  A mismatch
        // means the new slot is building on a different fork branch — valid on
        // Solana but rare on mainnet and always worth flagging for diagnostics.
        let parent_tag: String = match last_slot {
            Some(prev_slot) if slot != prev_slot && parent_slot != prev_slot => {
                format!("  [PARENT MISMATCH ⚠ expected parent={prev_slot} got parent={parent_slot}]")
            }
            _ => String::new(),
        };

        // ── Deserialize the entry payload ─────────────────────────────────
        // The proxy stores the fully reassembled entry group as a raw
        // bincode-encoded Vec<Entry> inside the protobuf `entries` bytes field.
        // Bincode is not self-describing, so any shred-level data corruption or
        // a partial FEC recovery that produced bad bytes will surface here as an
        // opaque length or type mismatch error.
        let entries: Vec<Entry> = match bincode::deserialize(&pb.entries) {
            Ok(e) => e,
            Err(e) => {
                stats.deserialize_errors += 1;
                println!("┌─ MSG #{}  slot={slot}  [DESERIALIZE ERROR ✗]", stats.total_messages);
                println!("│  raw_bytes={raw_bytes}  error={e}");
                println!("└─\n");
                last_slot = Some(slot);
                continue;
            }
        };

        let entry_count      = entries.len();
        let txn_count: usize = entries.iter().map(|e| e.transactions.len()).sum();
        stats.total_entries += entry_count as u64;
        stats.total_txns    += txn_count   as u64;

        // ── Message header ────────────────────────────────────────────────
        println!(
            "┌─ MSG #{}  slot={slot}  parent={parent_slot}",
            stats.total_messages
        );
        println!("│  entries={entry_count}  txns={txn_count}  raw_payload={raw_bytes}B");
        println!(
            "│  since_last={ms:.1}ms  session={session_secs:.1}s{ordering_tag}{parent_tag}",
            ms = since_last.as_secs_f64() * 1000.0,
        );

        // ── Per-entry breakdown ───────────────────────────────────────────
        // A Solana Entry is one of two things:
        //
        //   TICK — carries zero transactions.  Its only purpose is to advance
        //   the Proof-of-History clock by running SHA-256 `num_hashes` times
        //   from the previous entry's hash.  Leaders emit ticks to prove time
        //   is passing even when no transactions are available to pack.
        //
        //   DATA — carries one or more transactions AND a PoH step.  The PoH
        //   hash in a data entry commits to both the elapsed tick count and the
        //   hash of all transactions in the entry, so you cannot reorder or
        //   substitute transactions without breaking the hash chain.  This is
        //   the tamper-evidence property of Proof-of-History.
        for (ei, entry) in entries.iter().enumerate() {
            let is_tick    = entry.transactions.is_empty();
            let num_hashes = entry.num_hashes;

            // The first 16 characters of the hash give enough visual entropy
            // to spot duplicates or unexpected repetitions in the log.
            let hash_short: String = entry.hash.to_string().chars().take(16).collect();

            if is_tick {
                stats.total_tick_entries += 1;
                println!("│  ├─ entry[{ei}]  type=TICK  num_hashes={num_hashes}  hash={hash_short}..");
                continue;
            }

            stats.total_data_entries += 1;
            println!(
                "│  ├─ entry[{ei}]  type=DATA  num_hashes={num_hashes}  txns={}  hash={hash_short}..",
                entry.transactions.len()
            );

            // ── Per-transaction breakdown ─────────────────────────────────
            // Vote transactions (validator consensus messages) make up the
            // majority of all transactions on mainnet.  We accumulate runs of
            // consecutive vote transactions and print them as a single collapsed
            // line so they do not drown out the non-vote transactions you
            // actually care about for MEV analysis.
            let mut vote_run: usize = 0;
            let tx_total = entry.transactions.len();

            for (ti, tx) in entry.transactions.iter().enumerate() {
                let this_is_vote = is_vote_tx(tx);

                if this_is_vote {
                    vote_run += 1;
                    stats.total_vote_txns += 1;

                    // Flush the accumulated vote run at the very last transaction
                    // in the entry so we never leave an un-printed tail when the
                    // entry ends with a run of votes.
                    if ti == tx_total - 1 {
                        let run_start = ti + 1 - vote_run;
                        println!(
                            "│  │   tx[{run_start}..{ti}]  VOTE ×{vote_run} (validator consensus — collapsed)"
                        );
                        vote_run = 0;
                    }
                    continue;
                }

                // Before printing this non-vote transaction, flush any
                // accumulated vote run that immediately preceded it.
                if vote_run > 0 {
                    let run_start = ti - vote_run;
                    let run_end   = ti - 1;
                    println!(
                        "│  │   tx[{run_start}..{run_end}]  VOTE ×{vote_run} (validator consensus — collapsed)"
                    );
                    vote_run = 0;
                }

                stats.total_nonvote_txns += 1;

                // The first signature is the canonical transaction id on-chain.
                // We print the first 12 and last 4 characters — enough to
                // uniquely identify it in a block explorer while keeping the
                // log line short.
                let sig_display = tx
                    .signatures
                    .first()
                    .map(|s| {
                        let full = s.to_string();
                        let tail = &full[full.len().saturating_sub(4)..];
                        format!("{}..{}", &full[..12.min(full.len())], tail)
                    })
                    .unwrap_or_else(|| "<unsigned>".into());

                let num_sigs         = tx.signatures.len();
                let account_keys     = tx.message.static_account_keys();
                let num_accounts     = account_keys.len();
                let instructions     = tx.message.instructions();
                let num_instructions = instructions.len();

                // v0 messages support Address Lookup Tables which allow
                // referencing more accounts than the 32 that fit in a legacy
                // message.  Most MEV transactions use v0 to pack in all the
                // AMM pool and token accounts they need to touch in one shot.
                let version = match &tx.message {
                    VersionedMessage::Legacy(_) => "legacy",
                    VersionedMessage::V0(_)     => "v0",
                };

                // Build the deduplicated list of programs invoked by this
                // transaction in instruction order.  Deduplication matters
                // because ComputeBudget appears as the first instruction of
                // almost every transaction and would otherwise repeat for every
                // multi-instruction swap, polluting the program list.
                let mut seen:            HashSet<String> = HashSet::new();
                let mut unique_programs: Vec<String>     = Vec::new();
                for ix in instructions {
                    if let Some(key) = account_keys.get(ix.program_id_index as usize) {
                        let label = program_label(&key.to_string());
                        if seen.insert(label.clone()) {
                            unique_programs.push(label);
                        }
                    }
                }

                println!(
                    "│  │   tx[{ti}]  sig={sig_display}  ver={version}  \
                     sigs={num_sigs}  accounts={num_accounts}  ixs={num_instructions}"
                );
                println!("│  │         programs={unique_programs:?}");

                // ── Instruction-level detail ──────────────────────────────
                // Each instruction names the program to call, which accounts
                // from the transaction's account list it operates on (as u8
                // indices), and an opaque data payload whose meaning is defined
                // by that program's ABI.  The account index array lets you see
                // how much account overlap exists between instructions — for
                // example a multi-leg swap where the same token account appears
                // in both legs.  The data length gives a rough sense of
                // instruction complexity without needing a full IDL decoder.
                for (ii, ix) in instructions.iter().enumerate() {
                    let prog_label = account_keys
                        .get(ix.program_id_index as usize)
                        .map(|k| program_label(&k.to_string()))
                        .unwrap_or_else(|| "?".into());
                    let data_len = ix.data.len();
                    println!(
                        "│  │         ix[{ii}]  prog={prog_label}  \
                         account_indices={:?}  data={data_len}B",
                        ix.accounts
                    );
                }
            }
        }

        // ── Periodic cumulative summary ───────────────────────────────────
        // Every SUMMARY_EVERY messages we print aggregate counters so you can
        // assess stream health (gap rate, out-of-order rate, error rate) at
        // a glance without grepping through the per-slot detail lines.
        if stats.total_messages % SUMMARY_EVERY == 0 {
            stats.print_summary(session_secs);
        }

        println!("└─");
        println!();

        last_slot = Some(slot);
    }

    // ── Clean shutdown ────────────────────────────────────────────────────────
    // stream.message() returned None, meaning the proxy sent a proper gRPC
    // end-of-stream frame (i.e. the proxy was sent SIGTERM and shut down
    // gracefully).  Print a final summary before the process exits.
    let session_secs = Instant::now().duration_since(session_start).as_secs_f64();
    println!("\n[STREAM CLOSED — proxy sent end-of-stream]");
    stats.print_summary(session_secs);

    Ok(())
}
