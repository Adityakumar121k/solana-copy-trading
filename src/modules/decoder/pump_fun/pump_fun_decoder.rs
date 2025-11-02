use crate::entities::trade::types::{TradeAction, TradeStatus};
use crate::kernel::utils::lamports::Lamports;
use crate::kernel::utils::price::Price;
use crate::modules::decoder::config::{FILTER_WALLETS, TOKEN_PROGRAM_ID_PUBKEY};
use crate::modules::decoder::program_decoder::ProgramDecoder;
use crate::modules::decoder::pump_fun::config::{
    PUMP_FUN_CREATE, PUMP_FUN_PROGRAM_ID_PUBKEY, TRADE_EVENT,
};
use crate::modules::decoder::types::{FilteredInstruction, ParsedInstruction};
use anyhow::{Result, bail};
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use solana_sdk::pubkey::Pubkey;
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransactionInfo;
use yellowstone_grpc_proto::prelude::TokenBalance;

pub struct PumpFunDecoder;

impl ProgramDecoder for PumpFunDecoder {
    fn filter_instructions<'a>(
        transaction: &'a SubscribeUpdateTransactionInfo,
        account_keys: &[Pubkey],
    ) -> Option<FilteredInstruction<'a>> {
        let mut count = 0;

        let mut accounts: &[u8] = &[];
        let mut data: &[u8] = &[];
        let mut event: &[u8] = &[];

        let mut check = |program_idx: u32, acc: &'a [u8], args: &'a [u8]| -> usize {
            let pid = match account_keys.get(program_idx as usize) {
                Some(pubkey) => pubkey,
                None => return count,
            };

            if &PUMP_FUN_PROGRAM_ID_PUBKEY == pid {
                if Self::check_discriminator(&PUMP_FUN_PROGRAM_ID_PUBKEY, args) {
                    accounts = acc;
                    data = args;

                    count += 1;
                }

                if args.len() >= 16 && args[8..].starts_with(TRADE_EVENT) {
                    event = &args[16..];
                }
            }

            count
        };

        if let Some(msg) = transaction
            .transaction
            .as_ref()
            .and_then(|t| t.message.as_ref())
        {
            for ix in &msg.instructions {
                let count = check(ix.program_id_index, &ix.accounts, &ix.data);

                if count > 1 {
                    return None;
                }
            }
        }

        if let Some(meta) = &transaction.meta {
            for ii in &meta.inner_instructions {
                for ix in &ii.instructions {
                    let count = check(ix.program_id_index, &ix.accounts, &ix.data);

                    if count > 1 {
                        return None;
                    }
                }
            }
        }

        if count == 0 || data.starts_with(PUMP_FUN_CREATE) {
            return None;
        }

        if FILTER_WALLETS.len() == 1 {
            return Some(FilteredInstruction { accounts, event });
        }

        for &idx in accounts {
            let Some(account) = account_keys.get(idx as usize) else {
                continue;
            };

            if FILTER_WALLETS.contains(account) {
                return Some(FilteredInstruction { accounts, event });
            }
        }

        None
    }

    fn parse_instruction(
        transaction_status: &TradeStatus,
        instruction: &FilteredInstruction,
        account_keys: &[Pubkey],
        pre_tb: &[TokenBalance],
        post_tb: &[TokenBalance],
    ) -> Result<ParsedInstruction> {
        let mut accounts = Vec::with_capacity(instruction.accounts.len());

        for &i in instruction.accounts.iter() {
            accounts.push(account_keys[i as usize]);
        }

        let token_mint_address = accounts[2].to_string();
        let token_program_id = &accounts[8];
        let token_decimals = Self::get_token_decimals(&token_mint_address, pre_tb, post_tb)?;

        let action = match token_program_id == &TOKEN_PROGRAM_ID_PUBKEY {
            true => TradeAction::Buy,
            false => TradeAction::Sell,
        };

        let mut sol_amount = Decimal::zero();
        let mut token_amount = Decimal::zero();
        let mut trade_fee = Decimal::zero();
        let mut price = Decimal::zero();

        if transaction_status == &TradeStatus::Success {
            (sol_amount, token_amount, trade_fee) =
                Self::parse_trade_event(instruction.event, token_decimals)?;

            price = Price::get_price(token_amount, sol_amount)?;
        }

        Ok(ParsedInstruction {
            program_id: PUMP_FUN_PROGRAM_ID_PUBKEY,
            wallet: accounts[6],
            mint: accounts[2],
            action,
            token_amount,
            token_decimals,
            sol_amount,
            price,
            trade_fee,
            accounts,
        })
    }
}

impl PumpFunDecoder {
    fn parse_trade_event(bytes: &[u8], token_decimals: u32) -> Result<(Decimal, Decimal, Decimal)> {
        const NEED: usize = 217; // 32+8+8+1+32+8+8+8+8+8+32+8+8+32+8+8

        if bytes.len() < NEED {
            bail!("Trade event not found");
        }

        let mut offset = 32;
        let sol_amount = u64::from_le_bytes(bytes[offset..offset + 8].try_into()?);

        offset = 40;
        let token_amount = u64::from_le_bytes(bytes[offset..offset + 8].try_into()?);

        offset = 161; // fee
        let fee = u64::from_le_bytes(bytes[offset..offset + 8].try_into()?);

        offset = 209; // creator fee
        let creator_fee = u64::from_le_bytes(bytes[offset..offset + 8].try_into()?);

        Ok((
            Lamports::lamports_to_decimal(sol_amount, 9),
            Lamports::lamports_to_decimal(token_amount, token_decimals),
            Lamports::lamports_to_decimal(fee + creator_fee, 9),
        ))
    }
}
