// src/mev/stages/mod.rs
//
// The stages module contains the two pre-flight pipeline components that
// sit between pool-update detection and transaction submission.
//
// `fangzhen_jieduan` (仿真阶段) — Simulation Stage
//   Evaluates all qualifying arbitrage pairs using the fast off-chain sim
//   server and returns the single most profitable opportunity above the
//   profit threshold, or None if no pair is worth firing.
//
// `sanre_jieduan` (冷却阶段) — Cooling Stage
//   Suppresses duplicate submissions for the same pair within one Solana
//   slot window (~400 ms) to prevent the microsecond-speed engine from
//   firing multiple transactions on the same opportunity before the first
//   one lands.

pub mod fangzhen_jieduan;
pub mod sanre_jieduan;

pub use fangzhen_jieduan::BestSimResult;
pub use sanre_jieduan::CoolingStage;
