// The meteora_vault_cpi submodule exposes the Vault account struct and all
// types it depends on. Naming this submodule meteora_vault_cpi mirrors the
// external crate name used in the standalone bot so that every call site
// that writes meteora_vault_cpi::Vault::deserialize_unchecked resolves to
// the same path whether the code runs inside the validator process or
// standalone.
pub mod meteora_vault_cpi;
