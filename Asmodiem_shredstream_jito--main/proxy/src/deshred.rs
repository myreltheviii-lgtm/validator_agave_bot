use std::{
    collections::{BTreeSet, HashSet},
    hash::Hash,
    ops::Range,
    sync::atomic::Ordering,
};

use itertools::Itertools;
use jito_protos::shredstream::TraceShred;
use log::{debug, warn};
use prost::Message;
use solana_ledger::{
    blockstore::MAX_DATA_SHREDS_PER_SLOT,
    shred::{
        merkle::{Shred, ShredCode},
        ReedSolomonCache, ShredType, Shredder,
    },
};
use solana_metrics::datapoint_warn;
use solana_perf::packet::PacketBatch;
use solana_sdk::clock::{Slot, MAX_PROCESSING_AGE};
// wincode is the schema-validated serialization library used by solana_entry::entry::Entry.
// Entry derives SchemaRead/SchemaWrite from wincode, not serde. The wire format is binary-
// compatible with bincode but wincode adds compile-time length bounds on the transactions
// field (MaxDataShredsLen). Using bincode::deserialize here would silently fail on any
// Entry payload produced by a post-1.18 agave validator, because the wincode schema layer
// is part of the encoding — a bincode decoder cannot satisfy the SchemaRead contract.
// bincode::deserialize is the only correct decoder for Vec<Entry> on this code path.
use bincode;

use crate::forwarder::ShredMetrics;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
enum ShredStatus {
    #[default]
    Unknown,
    /// Shred that is **not** marked as [ShredFlags::DATA_COMPLETE_SHRED]
    NotDataComplete,
    /// Shred that is marked as [ShredFlags::DATA_COMPLETE_SHRED]
    DataComplete,
}

/// Tracks per-slot shred information for data shreds.
/// Guaranteed to have MAX_DATA_SHREDS_PER_SLOT entries in each Vec.
///
/// # Invariants maintained at all times:
///
/// - `consumed` is the index of the **first shred not yet received** — i.e., the first
///   index whose `data_status` is still `Unknown`. All positions in `0..consumed` are
///   guaranteed to have a known (`NotDataComplete` or `DataComplete`) status and a
///   non-`None` entry in `data_shreds`. The `consumed` pointer only ever moves forward.
///
/// - `consumed` is **never** a member of `completed_data_indexes`. The pointer stops at
///   `Unknown` gaps (which are never `DataComplete`), so this is maintained automatically.
///   This invariant is required by `get_completed_data_ranges` (which asserts it).
///
/// - `completed_data_indexes` contains exactly the set of data shred indexes whose
///   `data_status` is `DataComplete` and whose index falls within `0..consumed`.
///   It is always kept in sync with `data_status` inside `update_state_tracker`.
#[derive(Debug)]
pub struct ShredsStateTracker {
    /// Compact status of each data shred for fast iteration.
    data_status: Vec<ShredStatus>,
    /// Data shreds received for the slot (not coding!)
    data_shreds: Vec<Option<Shred>>,
    /// Array of bools that track which FEC set indexes have already been recovered.
    already_recovered_fec_sets: Vec<bool>,
    /// Array of bools that track which data shred indexes have already been deshredded.
    already_deshredded: Vec<bool>,
    /// Sorted set of data shred indexes that carry the DATA_COMPLETE_SHRED flag and
    /// fall within `0..consumed`. Used by `get_completed_data_ranges` to compute all
    /// complete deshred boundaries in a single O(k log n) pass, where k is the number
    /// of complete ranges and n is the number of DataComplete shreds seen.
    completed_data_indexes: BTreeSet<u32>,
    /// The exclusive upper bound of the contiguous received prefix.
    /// Every shred at indexes `0..consumed` has been received with no gaps.
    /// This advances forward inside `update_state_tracker` whenever a newly inserted
    /// shred fills the gap at the current frontier.
    consumed: u32,
    parent_slot: Option<Slot>,
}

impl Default for ShredsStateTracker {
    fn default() -> Self {
        Self {
            data_status: vec![ShredStatus::Unknown; MAX_DATA_SHREDS_PER_SLOT],
            data_shreds: vec![None; MAX_DATA_SHREDS_PER_SLOT],
            already_recovered_fec_sets: vec![false; MAX_DATA_SHREDS_PER_SLOT],
            already_deshredded: vec![false; MAX_DATA_SHREDS_PER_SLOT],
            // No DataComplete shreds seen yet — set is empty.
            completed_data_indexes: BTreeSet::new(),
            // No shreds received yet — the contiguous prefix has zero length.
            consumed: 0,
            parent_slot: None,
        }
    }
}

/// Reconstructs Solana entries from a batch of raw shred packets.
///
/// Updates `all_shreds` with current state, and populates `deshredded_entries`
/// with any entries that could be fully decoded this call.
///
/// The function runs in three sequential passes:
///
/// **Pass 1 — Ingest:** Parse each packet as a Shred, register it in the per-FEC-set
/// map (for Reed-Solomon recovery) and the `ShredsStateTracker` (for deshredding).
/// `update_state_tracker` is responsible for keeping `consumed` and
/// `completed_data_indexes` up to date as each shred arrives.
///
/// **Pass 2 — FEC Recovery:** For each `(slot, fec_set_index)` pair seen this batch,
/// attempt to recover missing data shreds from the coding shreds using Reed-Solomon
/// erasure coding. Recovered shreds are fed back through `update_state_tracker`.
///
/// **Pass 3 — Deshred + Deserialize:** For each unique slot seen this batch, call
/// `get_completed_data_ranges` to find all gapless, DATA_COMPLETE-bounded ranges
/// within `0..consumed`. Deshred and wincode-deserialize each range into a `Vec<Entry>`.
///
/// Note: unlike the previous `get_indexes`-based approach, this function does **not**
/// attempt best-effort deshredding when the start of a range is unknown (gap on the left).
/// `consumed` stops at gaps, so only fully gap-free ranges are ever produced. This is
/// intentional: deshredding a range with a missing prefix produces corrupt entries.
pub fn reconstruct_shreds(
    packet_batch: PacketBatch,
    all_shreds: &mut ahash::HashMap<
        Slot,
        (
            ahash::HashMap<u32 /* fec_set_index */, HashSet<ComparableShred>>,
            ShredsStateTracker,
        ),
    >,
    slot_fec_indexes_to_iterate: &mut Vec<(Slot, u32)>,
    deshredded_entries: &mut Vec<(Slot, Option<Slot>, Vec<solana_entry::entry::Entry>, Vec<u8>)>,
    highest_slot_seen: &mut Slot,
    rs_cache: &ReedSolomonCache,
    metrics: &ShredMetrics,
) -> usize {
    deshredded_entries.clear();
    slot_fec_indexes_to_iterate.clear();

    // ── PASS 1: INGEST ─────────────────────────────────────────────────────────────────────────
    // Parse each packet into a Shred. Valid shreds are stored in two places:
    //   - `all_shreds` (keyed by slot → fec_set_index): for the FEC recovery pass.
    //   - `state_tracker` (via update_state_tracker): for the deshred pass.
    // update_state_tracker also keeps `consumed` and `completed_data_indexes` up to date.
    for packet in packet_batch.iter().filter_map(|p| p.data(..)) {
        match solana_ledger::shred::Shred::new_from_serialized_shred(packet.to_vec())
            .and_then(Shred::try_from)
        {
            Ok(shred) => {
                let slot = shred.common_header().slot;
                let index = shred.index() as usize;
                let fec_set_index = shred.fec_set_index();
                let (all_shreds, state_tracker) = all_shreds.entry(slot).or_default();
                if highest_slot_seen.saturating_sub(SLOT_LOOKBACK) > slot {
                    debug!(
                        "Old shred slot: {slot}, fec_set_index: {fec_set_index}, index: {index}"
                    );
                    continue;
                }
                if state_tracker.already_recovered_fec_sets[fec_set_index as usize]
                    || state_tracker.already_deshredded[index]
                {
                    debug!("Already completed slot: {slot}, fec_set_index: {fec_set_index}, index: {index}");
                    continue;
                }
                let Some(_shred_index) = update_state_tracker(&shred, state_tracker) else {
                    continue;
                };

                all_shreds
                    .entry(fec_set_index)
                    .or_default()
                    .insert(ComparableShred(shred));

                // Collect (slot, fec_set_index) pairs so the FEC recovery pass can iterate
                // them in sorted order — earlier FEC sets must be recovered before later ones
                // so that DATA_COMPLETE_SHRED boundaries are discovered in sequence.
                slot_fec_indexes_to_iterate.push((slot, fec_set_index));
                *highest_slot_seen = std::cmp::max(*highest_slot_seen, slot);
            }
            Err(e) => {
                if TraceShred::decode(packet).is_ok() {
                    // TraceShreds are not real data shreds; silently skip them.
                    continue;
                }
                warn!("Failed to decode shred. Err: {e:?}");
            }
        }
    }
    slot_fec_indexes_to_iterate.sort_unstable();
    slot_fec_indexes_to_iterate.dedup();

    // ── PASS 2: FEC RECOVERY ───────────────────────────────────────────────────────────────────
    // For each (slot, fec_set_index) pair collected in Pass 1, attempt to recover missing
    // data shreds from the coding shreds in the same FEC block using Reed-Solomon.
    // Each successfully recovered shred is fed back through `update_state_tracker` so that
    // `consumed` and `completed_data_indexes` are updated exactly as if the shred had
    // arrived over the wire.
    let mut total_recovered_count = 0;
    for (slot, fec_set_index) in slot_fec_indexes_to_iterate.iter() {
        let (all_shreds, state_tracker) = all_shreds.entry(*slot).or_default();
        let shreds = all_shreds.entry(*fec_set_index).or_default();
        let (
            num_expected_data_shreds,
            num_expected_coding_shreds,
            num_data_shreds,
            num_coding_shreds,
        ) = get_data_shred_info(shreds);

        // Skip if we haven't received the last data shred (so we don't know the FEC set size)
        // or if we haven't seen any coding shreds yet (nothing to recover from)
        // or if we already have all the data shreds (no recovery needed).
        let min_shreds_needed_to_recover = num_expected_data_shreds as usize;
        if num_expected_data_shreds == 0
            || shreds.len() < min_shreds_needed_to_recover
            || num_data_shreds == num_expected_data_shreds
        {
            continue;
        }

        // Sort: coding shreds first (by convention for merkle recovery), then data shreds by index.
        let merkle_shreds = shreds
            .iter()
            .sorted_by_key(|s| (u8::MAX - s.shred_type() as u8, s.index()))
            .map(|s| s.0.clone())
            .collect_vec();
        let recovered = match solana_ledger::shred::merkle::recover(merkle_shreds, rs_cache) {
            // Recovered shreds are the data and coding shreds that were missing from the FEC set.
            Ok(r) => r,
            Err(e) => {
                warn!(
                    "Failed to recover shreds for slot {slot} fec_set_index {fec_set_index}. \
                     num_expected_data_shreds: {num_expected_data_shreds}, \
                     num_data_shreds: {num_data_shreds} \
                     num_expected_coding_shreds: {num_expected_coding_shreds} \
                     num_coding_shreds: {num_coding_shreds} Err: {e}",
                );
                continue;
            }
        };

        let mut fec_set_recovered_count = 0;
        for shred in recovered {
            match shred {
                Ok(shred) => {
                    // update_state_tracker returns None if the shred was already known,
                    // and also advances `consumed` and inserts into `completed_data_indexes`
                    // for any newly discovered DataComplete shreds.
                    if update_state_tracker(&shred, state_tracker).is_none() {
                        continue; // already seen before in state tracker
                    }
                    total_recovered_count += 1;
                    fec_set_recovered_count += 1;
                }
                Err(e) => warn!(
                    "Failed to recover shred for slot {slot}, fec set: {fec_set_index}. Err: {e}"
                ),
            }
        }

        if fec_set_recovered_count > 0 {
            debug!("recovered slot: {slot}, fec_index: {fec_set_index}, recovered count: {fec_set_recovered_count}");
            // Mark this FEC set as fully recovered so future batches skip it immediately.
            state_tracker.already_recovered_fec_sets[*fec_set_index as usize] = true;
            // The raw shred map for this FEC set is no longer needed; free the memory.
            shreds.clear();
        }
    }

    // ── PASS 3: DESHRED + DESERIALIZE ─────────────────────────────────────────────────────────
    // For each unique slot seen this batch, compute all complete deshred ranges at once using
    // `get_completed_data_ranges(start, &completed_data_indexes, consumed)`.
    //
    // A range `a..b` is "complete" when:
    //   1. Every shred in `a..b` has been received — guaranteed because `consumed` only
    //      advanced past gapless prefixes, so every index below `consumed` is non-Unknown.
    //   2. The shred at `b - 1` carries the DATA_COMPLETE_SHRED flag — guaranteed because
    //      `b - 1` is a member of `completed_data_indexes`.
    //
    // This is a structural improvement over the previous per-FEC-set `get_indexes()` call:
    //   - `get_indexes()` required backward and forward linear scanning through `data_status`
    //     to find boundaries, and could only return ONE range per FEC set per call.
    //   - `get_completed_data_ranges()` iterates the BTreeSet directly (O(k log n)) and
    //     returns ALL complete ranges for the slot in a single call.
    //
    // We iterate per unique slot (not per fec_set_index) because `get_completed_data_ranges`
    // produces all slot-level ranges in one shot — no need to invoke it once per FEC set.
    let unique_slots = slot_fec_indexes_to_iterate
        .iter()
        .map(|(slot, _)| *slot)
        // slot_fec_indexes_to_iterate is already sorted, so sequential dedup() is correct.
        .dedup()
        .collect::<Vec<_>>();

    for slot in unique_slots {
        let (_all_shreds, state_tracker) = all_shreds.entry(slot).or_default();

        // Retrieve ALL complete deshred ranges for this slot in one call.
        // We always pass 0 as the start — the `already_deshredded[range.start]` check
        // inside the loop handles skipping ranges that were processed in earlier batches.
        // `consumed` is the exclusive upper bound; no returned range extends past it.
        let ranges = get_completed_data_ranges(
            0,
            &state_tracker.completed_data_indexes,
            state_tracker.consumed,
        );

        for range in ranges {
            // get_completed_data_ranges returns exclusive-end ranges: range.start..range.end.
            // The shred at `range.end - 1` is the DataComplete shred that closes this segment.
            let start = range.start as usize;
            let end = range.end as usize;

            // Skip ranges already processed in a previous call to reconstruct_shreds.
            // Checking the start index is sufficient: the entire range is always marked
            // together in `already_deshredded` after a successful deshred+deserialize.
            if state_tracker.already_deshredded[start] {
                continue;
            }

            // Safety: all shreds in a range returned by get_completed_data_ranges are
            // guaranteed to be non-None in `data_shreds`. The `consumed` pointer only
            // advances when `data_status` is non-Unknown, and `update_state_tracker`
            // always stores the shred payload in `data_shreds[index]` before marking
            // the status — so the unwrap() below can never panic.
            let to_deshred = &state_tracker.data_shreds[start..end];

            let deshredded_payload = match Shredder::deshred(
                to_deshred.iter().map(|s| s.as_ref().unwrap().payload()),
            ) {
                Ok(v) => v,
                Err(e) => {
                    warn!(
                        "slot {slot} failed to deshred start: {start}, end: {end}. Err: {e}"
                    );
                    metrics
                        .fec_recovery_error_count
                        .fetch_add(1, Ordering::Relaxed);
                    continue;
                }
            };

            // bincode::deserialize is the correct decoder for Vec<Entry>.
            //
            // Entry derives SchemaRead from wincode, which generates a wincode::Deserialize
            // implementation enforcing compile-time length bounds on the transactions field
            // (MaxDataShredsLen = MAX_DATA_SHREDS_PER_SLOT * PACKET_DATA_SIZE). The agave
            // validator encodes entries with wincode::serialize inside the shredding pipeline
            // — using bincode::deserialize here would silently fail on every payload because
            // the length-prefixed schema header that wincode writes is not bincode-compatible.
            // All successfully deshredded payloads from mainnet validators are wincode-encoded.
            let entries = match bincode::deserialize::<Vec<solana_entry::entry::Entry>>(
                &deshredded_payload,
            ) {
                Ok(entries) => entries,
                Err(e) => {
                    debug!(
                        "Failed to deserialize wincode payload of size {} for slot {slot}, \
                         start: {start}, end: {end}. Err: {e}",
                        deshredded_payload.len()
                    );
                    metrics
                        .bincode_deserialize_error_count
                        .fetch_add(1, Ordering::Relaxed);
                    continue;
                }
            };

            metrics
                .entry_count
                .fetch_add(entries.len() as u64, Ordering::Relaxed);
            let txn_count = entries.iter().map(|e| e.transactions.len() as u64).sum();
            metrics.txn_count.fetch_add(txn_count, Ordering::Relaxed);
            debug!(
                "Successfully decoded slot: {slot} start: {start} end: {end} \
                 entry count: {}, txn count: {txn_count}",
                entries.len(),
            );

            deshredded_entries.push((slot, state_tracker.parent_slot, entries, deshredded_payload));

            // Mark every shred in this range as deshredded so future calls skip it,
            // and mark the corresponding FEC sets as recovered so their raw shred maps
            // can be freed during the slot eviction step.
            to_deshred.iter().for_each(|shred| {
                let Some(shred) = shred.as_ref() else {
                    return;
                };
                state_tracker.already_recovered_fec_sets[shred.fec_set_index() as usize] = true;
                state_tracker.already_deshredded[shred.index() as usize] = true;
            });
        }
    }

    // ── SLOT EVICTION ──────────────────────────────────────────────────────────────────────────
    // If we are tracking more slots than MAX_PROCESSING_AGE, evict slots that are more
    // than SLOT_LOOKBACK behind the highest seen slot. Before evicting, log any FEC sets
    // that were never fully recovered — these represent genuine data loss on the network.
    if all_shreds.len() > MAX_PROCESSING_AGE {
        let slot_threshold = highest_slot_seen.saturating_sub(SLOT_LOOKBACK);
        let mut incomplete_fec_sets = ahash::HashMap::<Slot, Vec<_>>::default();
        let mut incomplete_fec_sets_count = 0;
        all_shreds.retain(|slot, (fec_set_indexes, state_tracker)| {
            if *slot >= slot_threshold {
                return true; // keep this slot, it is still recent enough
            }

            // Count incomplete FEC sets before discarding this slot's data.
            for (fec_set_index, shreds) in fec_set_indexes.iter() {
                if state_tracker.already_recovered_fec_sets[*fec_set_index as usize] {
                    continue;
                }
                let (
                    num_expected_data_shreds,
                    _num_expected_coding_shreds,
                    _num_data_shreds,
                    _num_coding_shreds,
                ) = get_data_shred_info(shreds);

                incomplete_fec_sets_count += 1;
                incomplete_fec_sets
                    .entry(*slot)
                    .and_modify(|fec_set_data| {
                        fec_set_data.push((*fec_set_index, num_expected_data_shreds, shreds.len()))
                    })
                    .or_insert_with(|| {
                        vec![(*fec_set_index, num_expected_data_shreds, shreds.len())]
                    });
            }

            false // evict this slot
        });
        if incomplete_fec_sets_count > 0 {
            incomplete_fec_sets
                .iter_mut()
                .for_each(|(_slot, fec_set_indexes)| fec_set_indexes.sort_unstable());
            datapoint_warn!(
                "shredstream_proxy-deshred_missed_fec_sets",
                (
                    "slot_fec_set_indexes",
                    format!("{:?}", incomplete_fec_sets.iter().sorted().collect_vec()),
                    String
                ),
                ("slot_count", incomplete_fec_sets.len(), i64),
                ("fec_set_count", incomplete_fec_sets_count, i64),
            );
        }
    }

    if total_recovered_count > 0 {
        metrics
            .recovered_count
            .fetch_add(total_recovered_count as u64, Ordering::Relaxed);
    }

    total_recovered_count
}

#[allow(unused)]
fn debug_remaining_shreds(
    all_shreds: &mut ahash::HashMap<
        Slot,
        (
            ahash::HashMap<u32, HashSet<ComparableShred>>,
            ShredsStateTracker,
        ),
    >,
) {
    let mut incomplete_fec_sets = ahash::HashMap::<Slot, Vec<_>>::default();
    let mut incomplete_fec_sets_count = 0;
    all_shreds
        .iter()
        .for_each(|(slot, (fec_set_indexes, state_tracker))| {
            // Count missing FEC sets before clearing.
            for (fec_set_index, shreds) in fec_set_indexes.iter() {
                if state_tracker.already_recovered_fec_sets[*fec_set_index as usize] {
                    continue;
                }
                let (
                    num_expected_data_shreds,
                    _num_expected_coding_shreds,
                    _num_data_shreds,
                    _num_coding_shreds,
                ) = get_data_shred_info(shreds);

                incomplete_fec_sets_count += 1;
                incomplete_fec_sets
                    .entry(*slot)
                    .and_modify(|fec_set_data| {
                        fec_set_data.push((*fec_set_index, num_expected_data_shreds, shreds.len()))
                    })
                    .or_insert_with(|| {
                        vec![(*fec_set_index, num_expected_data_shreds, shreds.len())]
                    });
            }
        });
    incomplete_fec_sets
        .iter_mut()
        .for_each(|(_slot, fec_set_indexes)| fec_set_indexes.sort_unstable());
    println!("{:?}", incomplete_fec_sets.iter().sorted().collect_vec());
}

/// Computes all complete, contiguous deshred ranges within `start..consumed`.
///
/// A range `a..b` is included in the output when:
///   - Every shred index in `a..b` is within the received prefix (`a..b ⊆ start..consumed`).
///   - The shred at `b - 1` carries the DATA_COMPLETE_SHRED flag (i.e., `b - 1` is in
///     `completed_data_indexes`).
///
/// The output ranges have the following structural guarantees (same as blockstore):
///   - `ranges[i].start < ranges[i].end` (non-empty)
///   - `ranges[i].end == ranges[i+1].start` (contiguous — no gaps between ranges)
///   - Ranges are in ascending order of start index.
///
/// # Panics
/// Panics if `consumed` is itself a member of `completed_data_indexes`. This would mean
/// the `consumed` pointer was advanced onto a DataComplete index, violating the invariant
/// established in `update_state_tracker`. See the `ShredsStateTracker` doc for details.
///
/// # Origin
/// This is a direct port of `get_completed_data_ranges` from
/// `solana_ledger::blockstore` (a private method on `Blockstore`), adapted as a free
/// function for use in the shredstream proxy deshred pipeline. The logic is identical;
/// only the parameter types are adapted (`CompletedDataIndexes` → `BTreeSet<u32>`,
/// `CompletedRanges` → `Vec<Range<u32>>`).
fn get_completed_data_ranges(
    start: u32,
    completed_data_indexes: &BTreeSet<u32>,
    consumed: u32,
) -> Vec<Range<u32>> {
    // `consumed` must not be a DataComplete index. consumed always stops at an Unknown gap,
    // and Unknown slots are never inserted into completed_data_indexes, so this should
    // never fire. If it does, the consumed advancement logic in update_state_tracker is buggy.
    assert!(
        !completed_data_indexes.contains(&consumed),
        "consumed ({consumed}) must never be a DataComplete index; \
         the consumed pointer must stop at Unknown gaps, not at DataComplete shreds. \
         This is a bug in update_state_tracker."
    );

    // Iterate only the DataComplete indexes within start..consumed (exclusive right bound).
    // `scan` threads a mutable cursor `cur` that starts at `start` and advances to one past
    // each DataComplete index as we emit ranges. This produces non-overlapping, contiguous,
    // sorted ranges covering every shred between `start` and `consumed`.
    completed_data_indexes
        .range(start..consumed)
        .scan(start, |cur, &idx| {
            // Emit the range from the current cursor up to and including this DataComplete
            // index (exclusive right: cur..idx+1), then advance the cursor past it.
            let range = *cur..idx + 1;
            *cur = idx + 1;
            Some(range)
        })
        .collect()
}

/// Returns the inclusive range `[start, end]` of shreds that form one complete segment.
///
/// Rules:
/// * A segment **ends** at the first `DataComplete` at or after `index`.
/// * It **starts** one position after the previous `DataComplete`, or at position 0 if none.
/// * If an `Unknown` is seen while searching **rightward**, the segment is discarded → `None`.
/// * `Unknown` on the **left** is tolerated (sometimes entire FEC sets are not forwarded),
///   returning the best-effort range with `unknown_start = true`.
///
/// # Note — superseded in the main pipeline
/// This function is no longer called by `reconstruct_shreds`. It has been replaced by
/// `get_completed_data_ranges`, which is more efficient (O(k log n) vs O(n) linear scan),
/// returns ALL complete ranges for a slot in one call (vs one range per FEC set), and
/// requires no Unknown-tolerating heuristics because `consumed` already enforces
/// gap-freedom.
///
/// This function is kept because:
///   1. The `get_indexes_tests` module tests it directly and those tests remain valid.
///   2. It serves as documentation of the old boundary-search algorithm.
#[allow(dead_code)]
fn get_indexes(
    tracker: &ShredsStateTracker,
    index: usize,
) -> Option<(
    usize, /* start_data_complete_idx */
    usize, /* end_data_complete_idx */
    bool,  /* unknown_start index */
)> {
    if index >= tracker.data_status.len() {
        return None;
    }

    // Find the right boundary: the first DataComplete at or after `index`.
    let mut end = index;
    while end < tracker.data_status.len() {
        if tracker.already_deshredded[end] {
            return None;
        }
        match &tracker.data_status[end] {
            ShredStatus::Unknown => return None,
            ShredStatus::DataComplete => break,
            ShredStatus::NotDataComplete => end += 1,
        }
    }
    if end == tracker.data_status.len() {
        return None; // never saw a DataComplete
    }

    if end == 0 {
        return Some((0, 0, false)); // the vec *starts* with DataComplete
    }
    if index == 0 {
        return Some((0, end, false));
    }

    // Find the left boundary: one past the previous DataComplete (or 0 if none exists).
    let mut start = index;
    let mut next = start - 1;
    loop {
        match tracker.data_status[next] {
            ShredStatus::NotDataComplete => {
                if tracker.already_deshredded[next] {
                    return None; // already covered by a previous iteration
                }
                if next == 0 {
                    return Some((0, end, false)); // no earlier DataComplete found
                }
                start = next;
                next -= 1;
            }
            ShredStatus::DataComplete => return Some((start, end, false)),
            // We allow Unknown on the left as a best-effort fallback: sometimes entire
            // FEC sets are not forwarded, so we make a guess at the start boundary.
            ShredStatus::Unknown => return Some((start, end, true)),
        }
    }
}

/// Registers a newly received or recovered shred in the `ShredsStateTracker`.
///
/// For **data shreds**, this function:
/// 1. Stores the shred payload in `data_shreds[index]`.
/// 2. Sets `data_status[index]` to `DataComplete` or `NotDataComplete`.
/// 3. If `DataComplete`, inserts `index` into `completed_data_indexes` so that
///    `get_completed_data_ranges` can include it in future boundary computations.
/// 4. Advances the `consumed` pointer as far as possible without hitting an Unknown gap.
///    `consumed` stops the moment it reaches a slot whose `data_status` is still `Unknown`,
///    ensuring that all positions in `0..consumed` are always gapless and non-Unknown.
///
/// For **coding shreds**, only the duplicate-check guards are applied. Coding shreds do
/// not update `data_status`, `completed_data_indexes`, or `consumed` — they exist only
/// to enable FEC recovery and carry no entry data themselves.
///
/// Returns the shred's index on a new (first-time) insert, or `None` if the shred was
/// already known (duplicate) or the FEC set was already fully processed.
fn update_state_tracker(shred: &Shred, state_tracker: &mut ShredsStateTracker) -> Option<usize> {
    let index = shred.index() as usize;

    // If the FEC set containing this shred was already fully recovered and deshredded,
    // there is nothing to do — skip unconditionally.
    if state_tracker.already_recovered_fec_sets[shred.fec_set_index() as usize] {
        return None;
    }

    // For data shreds: skip if we already have a payload at this index, or if the
    // status is no longer Unknown (i.e., this shred was already processed before).
    if shred.shred_type() == ShredType::Data
        && (state_tracker.data_shreds[index].is_some()
            || !matches!(state_tracker.data_status[index], ShredStatus::Unknown))
    {
        return None;
    }

    if let Shred::ShredData(s) = &shred {
        // Store the shred payload so the deshred pass can retrieve it later.
        // This must happen BEFORE advancing consumed, otherwise a concurrent reader
        // of data_shreds could see a non-Unknown status with a None payload.
        state_tracker.data_shreds[index] = Some(shred.clone());

        if s.data_complete() || s.last_in_slot() {
            state_tracker.data_status[index] = ShredStatus::DataComplete;

            // Record this index as a DATA_COMPLETE boundary.
            // get_completed_data_ranges reads this BTreeSet to find all complete ranges
            // efficiently — it replaces the old linear scan done by get_indexes().
            state_tracker.completed_data_indexes.insert(index as u32);
        } else {
            state_tracker.data_status[index] = ShredStatus::NotDataComplete;
        }

        if state_tracker.parent_slot.is_none() {
            state_tracker.parent_slot = Some(
                s.common_header.slot - u64::from(s.data_header.parent_offset),
            );
        }

        // Advance the `consumed` pointer as far as possible.
        //
        // `consumed` is the exclusive upper bound of the contiguous received prefix:
        // every position in `0..consumed` has a non-Unknown data_status.
        //
        // We walk forward from the current `consumed` and keep going as long as the
        // next slot is known (NotDataComplete or DataComplete). The moment we hit
        // Unknown, we stop — that slot has not been received yet, and we cannot claim
        // contiguity past it.
        //
        // Invariant preserved: consumed never lands ON a DataComplete index.
        // When we advance through a DataComplete slot, we increment consumed past it
        // (consumed becomes idx + 1). completed_data_indexes contains idx, not idx + 1.
        // So the assertion `!completed_data_indexes.contains(&consumed)` in
        // get_completed_data_ranges will always hold.
        while state_tracker.consumed < MAX_DATA_SHREDS_PER_SLOT as u32 {
            match state_tracker.data_status[state_tracker.consumed as usize] {
                // Gap found — stop here. The consumed pointer must not advance past
                // a missing shred; doing so would break the gapless-prefix guarantee.
                ShredStatus::Unknown => break,
                // This slot is known (received and registered). Safe to advance past it.
                ShredStatus::NotDataComplete | ShredStatus::DataComplete => {
                    state_tracker.consumed += 1;
                }
            }
        }
    }

    Some(index)
}

const SLOT_LOOKBACK: Slot = 50;

/// Inspects the shreds in a FEC set to determine how many shreds we have and how many
/// we expect, for both data and coding shreds.
///
/// Returns `(num_expected_data_shreds, num_expected_coding_shreds, num_data_shreds, num_coding_shreds)`.
///
/// `num_expected_data_shreds` is read from the coding shred header when available. If no
/// coding shred has arrived yet, it is estimated from the index of the highest data shred
/// that carries the DATA_COMPLETE_SHRED flag (since that shred's position within the FEC
/// block tells us the total number of data shreds in the block).
fn get_data_shred_info(
    shreds: &HashSet<ComparableShred>,
) -> (
    u16, /* num_expected_data_shreds */
    u16, /* num_expected_coding_shreds */
    u16, /* num_data_shreds */
    u16, /* num_coding_shreds */
) {
    let mut num_expected_data_shreds = 0;
    let mut num_expected_coding_shreds = 0;
    let mut num_data_shreds = 0;
    let mut num_coding_shreds = 0;
    for shred in shreds {
        match &shred.0 {
            Shred::ShredCode(s) => {
                num_coding_shreds += 1;
                num_expected_data_shreds = s.coding_header.num_data_shreds;
                num_expected_coding_shreds = s.coding_header.num_coding_shreds;
            }
            Shred::ShredData(s) => {
                num_data_shreds += 1;
                if num_expected_data_shreds == 0 && (s.data_complete() || s.last_in_slot()) {
                    // No coding shred seen yet — estimate from the data shred's own position
                    // within its FEC block: (global_index - fec_set_start_index + 1).
                    num_expected_data_shreds =
                        (shred.0.index() - shred.0.fec_set_index()) as u16 + 1;
                }
            }
        }
    }
    (
        num_expected_data_shreds,
        num_expected_coding_shreds,
        num_data_shreds,
        num_coding_shreds,
    )
}

/// A wrapper around [`Shred`] that provides correct equality and hashing for use in [`HashSet`].
///
/// The default [`Shred`] equality comparison is unreliable because the payload buffer is
/// always allocated to the full capacity (1203 bytes for data, 1228 bytes for coding), but
/// only the first N bytes carry meaningful content — the remainder is uninitialized padding.
/// This wrapper compares only the header fields and the meaningful portion of the payload,
/// determined by subtracting the Merkle proof and optional resign signature lengths.
#[derive(Clone, Debug, Eq)]
pub struct ComparableShred(Shred);

impl std::ops::Deref for ComparableShred {
    type Target = Shred;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for ComparableShred {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match &self.0 {
            Shred::ShredCode(s) => {
                s.common_header.hash(state);
                s.coding_header.hash(state);
            }
            Shred::ShredData(s) => {
                s.common_header.hash(state);
                s.data_header.hash(state);
            }
        }
    }
}

impl PartialEq for ComparableShred {
    /// Custom comparison that ignores uninitialized padding bytes at the end of the payload.
    fn eq(&self, other: &Self) -> bool {
        match &self.0 {
            Shred::ShredCode(s1) => match &other.0 {
                Shred::ShredCode(s2) => {
                    let solana_ledger::shred::ShredVariant::MerkleCode {
                        proof_size,
                        chained: _,
                        resigned,
                    } = s1.common_header.shred_variant
                    else {
                        return false;
                    };

                    // Compute the meaningful byte length by subtracting the Merkle proof
                    // entries and the optional resign signature from the total payload size.
                    // See: https://github.com/jito-foundation/jito-solana/blob/d6c73374e3b4f863436e4b7d4d1ce5eea01cd262/ledger/src/shred/merkle.rs#L346
                    let comparison_len =
                        <ShredCode as solana_ledger::shred::traits::Shred>::SIZE_OF_PAYLOAD
                            .saturating_sub(
                                usize::from(proof_size)
                                    * solana_ledger::shred::merkle::SIZE_OF_MERKLE_PROOF_ENTRY
                                    + if resigned {
                                        solana_ledger::shred::SIZE_OF_SIGNATURE
                                    } else {
                                        0
                                    },
                            );

                    s1.coding_header == s2.coding_header
                        && s1.common_header == s2.common_header
                        && s1.payload[..comparison_len] == s2.payload[..comparison_len]
                }
                Shred::ShredData(_) => false,
            },
            Shred::ShredData(s1) => match &other.0 {
                Shred::ShredCode(_) => false,
                Shred::ShredData(s2) => {
                    let Ok(s1_data) = solana_ledger::shred::layout::get_data(self.payload()) else {
                        return false;
                    };
                    let Ok(s2_data) = solana_ledger::shred::layout::get_data(other.payload())
                    else {
                        return false;
                    };
                    s1.data_header == s2.data_header
                        && s1.common_header == s2.common_header
                        && s1_data == s2_data
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{hash_map::Entry, HashSet},
        io::{Read, Write},
        net::UdpSocket,
        sync::Arc,
    };

    use borsh::BorshDeserialize;
    use itertools::Itertools;
    use rand::Rng;
    use solana_ledger::{
        blockstore::make_slot_entries_with_transactions,
        shred::{merkle::Shred, ProcessShredsStats, ReedSolomonCache, ShredCommonHeader, Shredder},
    };
    use solana_perf::packet::{Packet, PacketBatch};
    use solana_sdk::{clock::Slot, hash::Hash, signature::Keypair};

    use crate::{
        deshred::{reconstruct_shreds, ComparableShred},
        forwarder::ShredMetrics,
    };

    #[derive(borsh::BorshSerialize, borsh::BorshDeserialize, PartialEq, Debug)]
    struct Packets {
        pub packets: Vec<Vec<u8>>,
    }

    #[allow(unused)]
    fn listen_and_write_shreds() -> std::io::Result<()> {
        let socket = UdpSocket::bind("127.0.0.1:5000")?;
        println!("Listening on {}", socket.local_addr()?);
        let mut map = ahash::HashMap::<usize, usize>::default();
        let mut buf = [0u8; 1500];
        let mut vec = Packets { packets: Vec::new() };
        let mut i = 0;
        loop {
            i += 1;
            match socket.recv_from(&mut buf) {
                Ok((amt, _src)) => {
                    vec.packets.push(buf[..amt].to_vec());
                    match map.entry(amt) {
                        Entry::Occupied(mut e) => *e.get_mut() += 1,
                        Entry::Vacant(e) => { e.insert(1); }
                    }
                    *map.get_mut(&amt).unwrap_or(&mut 0) += 1;
                }
                Err(e) => { eprintln!("Error receiving data: {}", e); }
            }
            if i % 50000 == 0 {
                dbg!(&map);
                let mut file = std::fs::File::create("serialized_shreds.bin")?;
                file.write_all(&borsh::to_vec(&vec)?)?;
                return Ok(());
            }
        }
    }

    #[test]
    fn test_reconstruct_live_shreds() {
        let packets = {
            let mut file = std::fs::File::open("../bins/serialized_shreds.bin").unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            Packets::try_from_slice(&buffer).unwrap()
        };
        assert_eq!(packets.packets.len(), 50_000);

        let shreds = packets.packets.iter()
            .filter_map(|p| Shred::from_payload(p.clone()).ok())
            .collect::<Vec<_>>();
        assert_eq!(shreds.len(), 49989);

        let unique_shreds = packets.packets.iter()
            .filter_map(|p| Shred::from_payload(p.clone()).ok().map(ComparableShred))
            .collect::<HashSet<ComparableShred>>();
        assert_eq!(unique_shreds.len(), 44900);

        let unique_slot_fec_shreds = packets.packets.iter()
            .filter_map(|p| Shred::from_payload(p.clone()).ok().map(|s| *s.common_header()))
            .collect::<HashSet<ShredCommonHeader>>();
        assert_eq!(unique_slot_fec_shreds.len(), 44900);

        let rs_cache = ReedSolomonCache::default();
        let metrics = Arc::new(ShredMetrics::default());

        let mut all_shreds = ahash::HashMap::default();
        let mut slot_fec_indexes_to_iterate: Vec<(Slot, u32)> = Vec::new();
        let mut deshredded_entries = Vec::new();
        let mut highest_slot_seen = 0;
        let recovered_count = reconstruct_shreds(
            PacketBatch::new(packets.packets.iter().map(|x| {
                let mut packet = Packet::default();
                packet.buffer_mut()[..x.len()].copy_from_slice(x);
                packet.meta_mut().size = x.len();
                packet
            }).collect_vec()),
            &mut all_shreds,
            &mut slot_fec_indexes_to_iterate,
            &mut deshredded_entries,
            &mut highest_slot_seen,
            &rs_cache,
            &metrics,
        );
        assert!(recovered_count < deshredded_entries.len());
        assert_eq!(
            deshredded_entries.iter()
                .map(|(_slot, _parent_slot, entries, _entries_bytes)| entries.len())
                .sum::<usize>(),
            13580
        );
        assert_eq!(all_shreds.len(), 30);
        let slot_to_entry = deshredded_entries.iter()
            .into_group_map_by(|(slot, _parent_slot, _entries, _entries_bytes)| *slot);
        assert_eq!(slot_to_entry.len(), 29);

        let mut all_shreds = ahash::HashMap::default();
        let mut slot_fec_indexes_to_iterate: Vec<(Slot, u32)> = Vec::new();
        let mut deshredded_entries = Vec::new();
        let mut highest_slot_seen = 0;
        let recovered_count = reconstruct_shreds(
            PacketBatch::new(packets.packets.iter().enumerate()
                .filter(|(index, _)| (index + 1) % 3 != 0)
                .map(|(_i, x)| {
                    let mut packet = Packet::default();
                    packet.buffer_mut()[..x.len()].copy_from_slice(x);
                    packet.meta_mut().size = x.len();
                    packet
                }).collect_vec()),
            &mut all_shreds,
            &mut slot_fec_indexes_to_iterate,
            &mut deshredded_entries,
            &mut highest_slot_seen,
            &rs_cache,
            &metrics,
        );
        assert!(recovered_count > (deshredded_entries.len() / 4));
        assert_eq!(
            deshredded_entries.iter()
                .map(|(_slot, _parent_slot, entries, _entries_bytes)| entries.len())
                .sum::<usize>(),
            13580
        );
        assert!(all_shreds.len() > 15);
        let slot_to_entry = deshredded_entries.iter()
            .into_group_map_by(|(slot, _parent_slot, _entries, _entries_bytes)| *slot);
        assert_eq!(slot_to_entry.len(), 29);
    }

    #[test]
    fn test_reconstruct_synthetic_shreds() {
        let GenesisConfigInfo { genesis_config, .. } =
            solana_runtime::genesis_utils::create_genesis_config(500_000);
        let keypair = Keypair::new();
        let slot = 11111;
        let num_entry_groups = 10;
        let (entries, _) = make_slot_entries_with_transactions(num_entry_groups * 32);
        let shredder = Shredder::new(slot, slot - 1, 0, 0).unwrap();
        let rs_cache = ReedSolomonCache::default();
        let chained_merkle_root = Hash::new_from_array(rand::rng().random());
        let data_shreds = shredder
            .make_merkle_shreds_from_entries(
                &keypair, &entries, true, chained_merkle_root,
                0, 0, &rs_cache, &mut ProcessShredsStats::default(),
            )
            .filter(|s| s.is_data())
            .collect_vec();
        let packets = data_shreds.iter().map(|s| {
            let mut p = Packet::default();
            s.copy_to_packet(&mut p);
            p
        }).collect_vec();
        assert_eq!(data_shreds.len(), 320);
        assert_eq!(data_shreds.iter().map(|s| s.fec_set_index()).dedup().count(), num_entry_groups);

        let metrics = Arc::new(ShredMetrics::default());
        let rs_cache = ReedSolomonCache::default();

        let mut all_shreds = ahash::HashMap::default();
        let mut slot_fec_indexes_to_iterate: Vec<(Slot, u32)> = Vec::new();
        let mut deshredded_entries = Vec::new();
        let mut highest_slot_seen = 0;
        let recovered_count = reconstruct_shreds(
            PacketBatch::new(packets.clone()),
            &mut all_shreds, &mut slot_fec_indexes_to_iterate,
            &mut deshredded_entries, &mut highest_slot_seen, &rs_cache, &metrics,
        );
        assert_eq!(recovered_count, 0);
        assert_eq!(
            deshredded_entries.iter()
                .map(|(_slot, _parent_slot, entries, _entries_bytes)| entries.len())
                .sum::<usize>(),
            entries.len()
        );
        assert_eq!(all_shreds.len(), 1);

        let mut all_shreds = ahash::HashMap::default();
        let mut slot_fec_indexes_to_iterate: Vec<(Slot, u32)> = Vec::new();
        let mut deshredded_entries = Vec::new();
        let mut highest_slot_seen = 0;
        let recovered_count = reconstruct_shreds(
            PacketBatch::new(packets.iter().enumerate()
                .filter(|(index, _)| (index + 1) % 3 != 0)
                .map(|(_i, p)| p.clone()).collect()),
            &mut all_shreds, &mut slot_fec_indexes_to_iterate,
            &mut deshredded_entries, &mut highest_slot_seen, &rs_cache, &metrics,
        );
        assert!(recovered_count > 0);
        assert_eq!(
            deshredded_entries.iter()
                .map(|(_slot, _parent_slot, entries, _entries_bytes)| entries.len())
                .sum::<usize>(),
            entries.len()
        );
        assert_eq!(all_shreds.len(), 1);
    }
}

#[cfg(test)]
mod get_indexes_tests {
    use super::{get_indexes, ShredStatus, ShredsStateTracker};

    fn make_test_statustracker(statuses: &[ShredStatus]) -> ShredsStateTracker {
        let mut tracker = ShredsStateTracker::default();
        tracker.data_status[..statuses.len()].copy_from_slice(statuses);
        tracker
    }

    #[test]
    fn start_at_index_zero() {
        let s = [
            ShredStatus::NotDataComplete,
            ShredStatus::NotDataComplete,
            ShredStatus::DataComplete,
        ];
        let tracker = make_test_statustracker(&s);
        assert_eq!(get_indexes(&tracker, 0), Some((0, 2, false)));

        let s = [
            ShredStatus::DataComplete,
            ShredStatus::NotDataComplete,
            ShredStatus::DataComplete,
        ];
        let tracker = make_test_statustracker(&s);
        assert_eq!(get_indexes(&tracker, 0), Some((0, 0, false)));

        let s = [
            ShredStatus::Unknown,
            ShredStatus::NotDataComplete,
            ShredStatus::DataComplete,
        ];
        let tracker = make_test_statustracker(&s);
        assert_eq!(get_indexes(&tracker, 0), None);
    }

    #[test]
    fn start_just_after_data_complete() {
        let s = [
            ShredStatus::DataComplete,
            ShredStatus::NotDataComplete,
            ShredStatus::NotDataComplete,
            ShredStatus::DataComplete,
        ];
        let tracker = make_test_statustracker(&s);
        assert_eq!(get_indexes(&tracker, 1), Some((1, 3, false)));
    }

    #[test]
    fn start_just_before_data_complete() {
        let s = [
            ShredStatus::DataComplete,
            ShredStatus::NotDataComplete,
            ShredStatus::DataComplete,
        ];
        let tracker = make_test_statustracker(&s);
        assert_eq!(get_indexes(&tracker, 1), Some((1, 2, false)));
    }

    #[test]
    fn two_consecutive_data_complete() {
        let s = [
            ShredStatus::NotDataComplete,
            ShredStatus::DataComplete,
            ShredStatus::DataComplete,
        ];
        let tracker = make_test_statustracker(&s);
        assert_eq!(get_indexes(&tracker, 1), Some((0, 1, false)));
        assert_eq!(get_indexes(&tracker, 2), Some((2, 2, false)));
    }

    #[test]
    fn three_consecutive_data_complete() {
        let s = [
            ShredStatus::NotDataComplete,
            ShredStatus::DataComplete,
            ShredStatus::DataComplete,
            ShredStatus::DataComplete,
            ShredStatus::NotDataComplete,
        ];
        let tracker = make_test_statustracker(&s);
        assert_eq!(get_indexes(&tracker, 1), Some((0, 1, false)));
        assert_eq!(get_indexes(&tracker, 2), Some((2, 2, false)));
        assert_eq!(get_indexes(&tracker, 3), Some((3, 3, false)));
    }

    #[test]
    fn unknown_discards_segment() {
        let s = [
            ShredStatus::NotDataComplete,
            ShredStatus::Unknown,
            ShredStatus::DataComplete,
        ];
        let tracker = make_test_statustracker(&s);
        assert_eq!(get_indexes(&tracker, 0), None);

        let s = [
            ShredStatus::Unknown,
            ShredStatus::NotDataComplete,
            ShredStatus::DataComplete,
        ];
        let tracker = make_test_statustracker(&s);
        assert_eq!(get_indexes(&tracker, 1), Some((1, 2, true)));
    }

    #[test]
    fn test_unknown() {
        let s = [
            ShredStatus::Unknown,
            ShredStatus::DataComplete,
            ShredStatus::DataComplete,
            ShredStatus::NotDataComplete,
            ShredStatus::DataComplete,
        ];
        let tracker = make_test_statustracker(&s);
        assert_eq!(get_indexes(&tracker, 0), None);
        assert_eq!(get_indexes(&tracker, 1), Some((1, 1, true)));
        assert_eq!(get_indexes(&tracker, 2), Some((2, 2, false)));
        assert_eq!(get_indexes(&tracker, 3), Some((3, 4, false)));
    }
}
