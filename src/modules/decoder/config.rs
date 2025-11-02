use crate::kernel::wallet::signer::SignerKeypair;
use crate::modules::decoder::pump_fun::config::{
    PUMP_FUN_BUY, PUMP_FUN_CREATE, PUMP_FUN_PROGRAM_ID_PUBKEY, PUMP_FUN_SELL,
};
use dashmap::DashSet;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::env;
use std::str::FromStr;
use std::sync::LazyLock;

pub const TOKEN_PROGRAM_ID_PUBKEY: Pubkey =
    Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

pub static PROGRAM_ID_PUBKEYS: [Pubkey; 1] = [PUMP_FUN_PROGRAM_ID_PUBKEY];

pub static DISCRIMINATORS: LazyLock<HashMap<Pubkey, &[&[u8]]>> = LazyLock::new(|| {
    HashMap::from([(
        PUMP_FUN_PROGRAM_ID_PUBKEY,
        [PUMP_FUN_CREATE, PUMP_FUN_BUY, PUMP_FUN_SELL].as_slice(),
    )])
});

pub static FILTER_WALLETS: LazyLock<DashSet<Pubkey>> = LazyLock::new(|| {
    let wallets = env::var("FOLLOW_WALLETS").expect("FOLLOW_WALLETS env not set");

    let iter = wallets
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| match Pubkey::from_str(s) {
            Ok(key) => Some(key),
            Err(error) => {
                tracing::error!("{}", error);
                None
            }
        });

    let set = DashSet::from_iter(iter);

    set.insert(*SignerKeypair::pubkey());

    set
});
