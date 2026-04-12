pub mod arbitrage_executor;
pub mod smb_instruction_builder;
pub mod token_flow_validator;

// The four concrete types that live in arbitrage_executor are re-exported flat
// so that engine.rs can write `use crate::mev::executor::{…}` without knowing
// the internal submodule layout.  Keeping the public surface at the executor
// level (rather than forcing callers to reach into arbitrage_executor directly)
// means the submodule can be renamed or split later without touching engine.rs.
//
// ShardWorkItem  — the tagged union of events the engine pushes into each
//                  shard's rtrb ring buffer: pool updates, new-mint registrations,
//                  and pool-graduation notifications.
//
// HttpWorkItem   — the serialised, signed transaction bytes that a MevShard
//                  pushes to its paired HttpWorker after simulation confirms
//                  a profitable opportunity.  All CPU-bound work (signing,
//                  bincode serialisation) is done on the shard thread before
//                  the push so the HttpWorker thread does I/O only.
//
// MevShard       — one of twelve dedicated OS threads, each owning 1/12 of the
//                  tracked mints.  Spins on its Consumer<ShardWorkItem> ring
//                  buffer, performs inline SVM simulation or inline tx-build,
//                  and pushes to HttpWorker via a second ring buffer.
//
// HttpWorker     — the blocking HTTP thread paired with one MevShard.  Spins on
//                  its Consumer<HttpWorkItem> ring buffer and fires synchronous
//                  reqwest::blocking POST requests to the Helius Sender FRA
//                  endpoint — one persistent TCP connection, zero Tokio
//                  involvement.
pub use arbitrage_executor::{HttpWorker, HttpWorkItem, MevShard, ShardWorkItem};

pub use smb_instruction_builder::SmbInstructionBuilder;
pub use token_flow_validator::{TokenFlowStep, TokenFlowValidator};
