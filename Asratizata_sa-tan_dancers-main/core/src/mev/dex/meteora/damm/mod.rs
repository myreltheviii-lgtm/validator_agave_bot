// The meteora_damm_cpi submodule exposes the Pool account struct and all
// types it depends on. Naming this submodule meteora_damm_cpi mirrors the
// external crate name used in the standalone bot so that every call site
// that writes meteora_damm_cpi::Pool::deserialize_unchecked resolves to the
// same path whether the code runs inside the validator process or standalone.
pub mod meteora_damm_cpi;
