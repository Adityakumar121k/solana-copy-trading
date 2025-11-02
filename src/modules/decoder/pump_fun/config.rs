use solana_sdk::pubkey::Pubkey;

pub const PUMP_FUN_PROGRAM_ID_PUBKEY: Pubkey =
    Pubkey::from_str_const("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

pub const PUMP_FUN_CREATE: &[u8] = &[24, 30, 200, 40, 5, 28, 7, 119];

pub const PUMP_FUN_BUY: &[u8] = &[102, 6, 61, 18, 1, 218, 235, 234];

pub const PUMP_FUN_SELL: &[u8] = &[51, 230, 133, 164, 1, 127, 131, 173];

pub const TRADE_EVENT: &[u8] = &[189, 219, 127, 211, 78, 230, 97, 238];
