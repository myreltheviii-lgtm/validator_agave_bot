//! Local re-export of `SpeculativeAccountUpdate` from `solana_ledger`.
//!
//! # Why this module exists
//!
//! `MevEngine` (in `solana-core`) and `SpeculativeSlotExecutor` (in `solana-ledger`)
//! exchange `SpeculativeAccountUpdate` values over a crossbeam channel. The channel's
//! generic type parameter must resolve to exactly one concrete type — both the sender
//! and the receiver must name the same type. The authoritative declaration lives in
//! `solana_ledger::devil_mode_jito__` because that is where `execute()` and
//! `confirm_slot()` produce the values.
//!
//! This module re-exports the ledger declaration so callers inside `solana-core`
//! can write `crate::mev::SpeculativeAccountUpdate` instead of the fully qualified
//! path. There is no local struct definition — re-exporting the ledger type prevents
//! any possibility of a struct layout drift between the producer and the consumer.
pub use solana_ledger::devil_mode_jito__::SpeculativeAccountUpdate;
