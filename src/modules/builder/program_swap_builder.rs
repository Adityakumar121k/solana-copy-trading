use crate::entities::trade::types::TradeAction;
use crate::modules::decoder::types::ParsedInstruction;
use anyhow::Result;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;

pub trait ProgramSwapBuilder {
    fn build_swap(parsed_instruction: &ParsedInstruction) -> Result<Vec<Instruction>> {
        match parsed_instruction.action {
            TradeAction::Buy => Self::build_buy(parsed_instruction),
            TradeAction::Sell => Self::build_sell(parsed_instruction),
        }
    }

    fn build_buy(parsed_instruction: &ParsedInstruction) -> Result<Vec<Instruction>>;

    fn build_sell(parsed_instruction: &ParsedInstruction) -> Result<Vec<Instruction>>;

    fn prepare_accounts(
        discriminator: &[u8],
        parsed_instruction: &ParsedInstruction,
        quote_ata: Pubkey,
    ) -> Vec<AccountMeta>;

    fn prepare_args(discriminator: &[u8], base_amount: u64, quote_amount: u64) -> Vec<u8>;
}
