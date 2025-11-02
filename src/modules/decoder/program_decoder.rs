use crate::entities::trade::types::TradeStatus;
use crate::modules::decoder::config::DISCRIMINATORS;
use crate::modules::decoder::types::{FilteredInstruction, ParsedInstruction};
use anyhow::Result;
use anyhow::bail;
use solana_sdk::pubkey::Pubkey;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;
use yellowstone_grpc_proto::prelude::TokenBalance;

pub trait ProgramDecoder {
    fn filter_instructions<'a>(
        transaction: &'a SubscribeUpdateTransactionInfo,
        accounts: &[Pubkey],
    ) -> Option<FilteredInstruction<'a>>;

    fn parse_instruction(
        transaction_status: &TradeStatus,
        instruction: &FilteredInstruction,
        accounts: &[Pubkey],
        pre_tb: &[TokenBalance],
        post_tb: &[TokenBalance],
    ) -> Result<ParsedInstruction>;

    fn check_discriminator(program_id: &Pubkey, args: &[u8]) -> bool {
        DISCRIMINATORS
            .get(program_id)
            .is_some_and(|list| list.iter().any(|d| args.starts_with(d)))
    }

    fn get_token_decimals(
        token_mint_address: &str,
        pre_tb: &[TokenBalance],
        post_tb: &[TokenBalance],
    ) -> Result<u32> {
        for tb in pre_tb.iter().chain(post_tb) {
            if tb.mint == token_mint_address {
                if let Some(ui_token_amount) = &tb.ui_token_amount {
                    return Ok(ui_token_amount.decimals);
                }
            }
        }

        bail!("token_mint_address not found");
    }
}
