use crate::entities::position::types::PositionStatus;
use crate::kernel::cache::position_cache::PositionCache;
use crate::kernel::utils::amounts::Amounts;
use crate::kernel::utils::lamports::Lamports;
use crate::kernel::wallet::signer::SignerKeypair;
use crate::modules::builder::program_swap_builder::ProgramSwapBuilder;
use crate::modules::builder::pump_fun::types::{ARGS_LEN, USER_VOLUME_ACCUMULATOR, WRITABLE_MASK};
use crate::modules::decoder::pump_fun::config::{
    PUMP_FUN_BUY, PUMP_FUN_PROGRAM_ID_PUBKEY, PUMP_FUN_SELL,
};
use crate::modules::decoder::types::ParsedInstruction;
use anyhow::{Context, Result};
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::instruction::Instruction;
use solana_sdk::pubkey::Pubkey;
use spl_associated_token_account;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;
use spl_token::instruction::{burn, close_account};

pub struct PumpFunBuilder;

impl ProgramSwapBuilder for PumpFunBuilder {
    fn build_buy(parsed_instruction: &ParsedInstruction) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::with_capacity(6);

        let token_mint_address = &parsed_instruction.accounts[2];
        let token_program_id = &parsed_instruction.accounts[8];

        let (base_amount, quote_amount) =
            Amounts::token_from_sol(parsed_instruction.price, parsed_instruction.token_decimals)?;

        let quote_ata = get_associated_token_address_with_program_id(
            SignerKeypair::pubkey(),
            token_mint_address,
            token_program_id,
        );

        instructions.push(create_associated_token_account_idempotent(
            SignerKeypair::pubkey(),
            SignerKeypair::pubkey(),
            token_mint_address,
            token_program_id,
        ));

        instructions.push(Instruction {
            program_id: PUMP_FUN_PROGRAM_ID_PUBKEY,
            accounts: Self::prepare_accounts(PUMP_FUN_BUY, parsed_instruction, quote_ata),
            data: Self::prepare_args(PUMP_FUN_BUY, base_amount, quote_amount),
        });

        Ok(instructions)
    }

    fn build_sell(parsed_instruction: &ParsedInstruction) -> Result<Vec<Instruction>> {
        let mut instructions = Vec::with_capacity(10);

        let token_mint_address = &parsed_instruction.accounts[2];
        let token_program_id = &parsed_instruction.accounts[9];

        let (base_amount, quote_amount) = Amounts::sol_from_token(
            &parsed_instruction.mint,
            &parsed_instruction.wallet,
            parsed_instruction.token_amount,
            parsed_instruction.token_decimals,
        )?;

        let quote_ata = get_associated_token_address_with_program_id(
            SignerKeypair::pubkey(),
            token_mint_address,
            token_program_id,
        );

        instructions.push(Instruction {
            program_id: PUMP_FUN_PROGRAM_ID_PUBKEY,
            accounts: Self::prepare_accounts(PUMP_FUN_SELL, parsed_instruction, quote_ata),
            data: Self::prepare_args(PUMP_FUN_SELL, base_amount, quote_amount),
        });

        let copy_position =
            PositionCache::get(&parsed_instruction.mint, &parsed_instruction.wallet, true)
                .with_context(|| {
                    format!(
                        "Copy position for close account not found: mint={}, key={:?}",
                        &parsed_instruction.mint,
                        PositionCache::get_key(&parsed_instruction.wallet, true),
                    )
                })?;

        let amount_left = copy_position.amount_left
            - Lamports::lamports_to_decimal(base_amount, parsed_instruction.token_decimals);

        let predict_status =
            PositionStatus::predict_from_amounts(amount_left, copy_position.amount_total);

        if predict_status == PositionStatus::Closed {
            if amount_left > Decimal::zero() {
                instructions.push(burn(
                    token_program_id,
                    &quote_ata,
                    token_mint_address,
                    SignerKeypair::pubkey(),
                    &[SignerKeypair::pubkey()],
                    Lamports::decimal_to_lamports(amount_left, parsed_instruction.token_decimals)?,
                )?);
            }

            instructions.push(close_account(
                token_program_id,
                &quote_ata,
                SignerKeypair::pubkey(),
                SignerKeypair::pubkey(),
                &[SignerKeypair::pubkey()],
            )?);
        }

        Ok(instructions)
    }

    fn prepare_accounts(
        discriminator: &[u8],
        parsed_instruction: &ParsedInstruction,
        quote_ata: Pubkey,
    ) -> Vec<AccountMeta> {
        let is_buy = discriminator == PUMP_FUN_BUY;
        let mask_dyn: u16 = if is_buy {
            (1 << 9) | (1 << 12) | (1 << 13)
        } else {
            1 << 8
        };
        let writable = WRITABLE_MASK | mask_dyn;
        let count = if is_buy { 16 } else { 14 };

        let mut accounts: Vec<AccountMeta> = Vec::with_capacity(count);

        for (i, pubkey) in parsed_instruction.accounts.iter().enumerate().take(count) {
            let pubkey = match i {
                5 => quote_ata,
                6 => *SignerKeypair::pubkey(),
                13 if is_buy => *USER_VOLUME_ACCUMULATOR,
                _ => *pubkey,
            };

            accounts.push(AccountMeta {
                pubkey,
                is_writable: (writable & (1_u16 << i)) != 0,
                is_signer: i == 6,
            });
        }

        accounts
    }

    fn prepare_args(discriminator: &[u8], base_amount: u64, quote_amount: u64) -> Vec<u8> {
        let mut args = Vec::with_capacity(ARGS_LEN);

        args.extend_from_slice(discriminator);
        args.extend_from_slice(&base_amount.to_le_bytes());
        args.extend_from_slice(&quote_amount.to_le_bytes());

        args
    }
}
