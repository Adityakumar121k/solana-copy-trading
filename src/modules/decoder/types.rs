use crate::entities::trade::types::{TradeAction, TradeStatus};
use rust_decimal::Decimal;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub struct FilteredInstruction<'a> {
    pub accounts: &'a [u8],
    pub event: &'a [u8],
}

#[derive(Debug)]
pub struct ParsedTransaction {
    pub status: TradeStatus,
    pub signature: Vec<u8>,
    pub instruction: ParsedInstruction,
    pub tx_fee: Decimal,
    pub slot: i32,
    pub priority_fee: u64,
}

#[derive(Debug)]
pub struct ParsedInstruction {
    pub program_id: Pubkey,
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub action: TradeAction,
    pub token_amount: Decimal,
    pub token_decimals: u32,
    pub sol_amount: Decimal,
    pub price: Decimal,
    pub trade_fee: Decimal,
    pub accounts: Vec<Pubkey>,
}
