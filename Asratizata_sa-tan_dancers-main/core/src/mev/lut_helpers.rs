use anyhow::Result;
// `solana_address_lookup_table_interface` is the disaggregated crate that owns
// the on-chain address lookup table state layout. It provides `AddressLookupTable::deserialize`
// which parses the raw account bytes into a typed struct exposing the `.addresses` slice.
use solana_address_lookup_table_interface::state::AddressLookupTable;
// `AddressLookupTableAccount` is the client-side struct that pairs a LUT's on-chain
// address with its decoded address list. It lives in `solana_message` because it is
// consumed by `v0::Message::try_compile` when building versioned transactions.
use solana_message::AddressLookupTableAccount;
use solana_pubkey::Pubkey;
use solana_client::rpc_client::RpcClient;
use std::sync::Arc;

pub fn fetch_address_lookup_table_account(
    rpc: &Arc<RpcClient>,
    lookup_table_address: &Pubkey,
) -> Result<AddressLookupTableAccount> {
    let account = rpc.get_account(lookup_table_address)?;
    let lookup_table = AddressLookupTable::deserialize(&account.data)?;
    
    Ok(AddressLookupTableAccount {
        key: *lookup_table_address,
        addresses: lookup_table.addresses.to_vec(),
    })
}

pub fn fetch_multiple_address_lookup_tables(
    rpc: &Arc<RpcClient>,
    lookup_table_addresses: &[Pubkey],
) -> Result<Vec<AddressLookupTableAccount>> {
    // RpcClient::get_multiple_accounts expects &[Pubkey] — a slice of owned Pubkeys,
    // not &Vec<&Pubkey>. Passing the input slice directly satisfies this signature.
    let accounts = rpc.get_multiple_accounts(lookup_table_addresses)?;
    let mut luts = Vec::with_capacity(accounts.len());
    
    for (addr, account_opt) in lookup_table_addresses.iter().zip(accounts.iter()) {
        if let Some(account) = account_opt {
            let lookup_table = AddressLookupTable::deserialize(&account.data)?;
            luts.push(AddressLookupTableAccount {
                key: *addr,
                addresses: lookup_table.addresses.to_vec(),
            });
        } else {
            return Err(anyhow::anyhow!("LUT account not found: {}", addr));
        }
    }
    
    Ok(luts)
}
