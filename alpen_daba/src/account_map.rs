// src/account_map.rs
//
// Wraps the WireAccount list from TwoHopSimRequest into a HashMap the sim
// functions query by pubkey.
//
// Returns owned solana_sdk::account::Account with the real `owner` field
// populated. This matters for pump and DLMM which check account.owner to
// distinguish Token-2022 accounts from classic SPL token accounts.

use std::collections::HashMap;

use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;

// TwoHopSimRequest and WireAccount are defined in the sim-client crate —
// the zero-SDK shared IPC contract that both the validator and this binary
// depend on. Importing them here rather than redefining them ensures both
// sides of the socket are always operating on identical struct layouts and
// bincode discriminants.
use sim_client::{TwoHopSimRequest, WireAccount};

pub struct AccountMap(HashMap<Pubkey, WireAccount>);

impl AccountMap {
    /// Build from the flat account list in the wire request.
    pub fn from_request(req: &TwoHopSimRequest) -> Self {
        let map = req
            .accounts
            .iter()
            .map(|wa| {
                let key = Pubkey::new_from_array(wa.pubkey);
                (key, WireAccount {
                    pubkey: wa.pubkey,
                    owner:  wa.owner,
                    data:   wa.data.clone(),
                })
            })
            .collect();
        AccountMap(map)
    }

    /// Look up an account by pubkey.
    ///
    /// The returned Account has `owner` set to the real program owner that was
    /// recorded by the Agave shard from Bank::get_account(). Lamports are left
    /// at 0 — no sim function inspects them.
    pub fn get_account(&self, key: &Pubkey) -> Option<Account> {
        self.0.get(key).map(|wa| Account {
            lamports:   0,
            data:       wa.data.clone(),
            owner:      Pubkey::new_from_array(wa.owner),
            executable: false,
            rent_epoch: u64::MAX,
        })
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}
