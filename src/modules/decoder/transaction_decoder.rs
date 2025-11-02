use crate::entities::trade::types::TradeStatus;
use crate::kernel::cache::priority_fee_cache::PriorityFeeCache;
use crate::kernel::utils::lamports::Lamports;
use crate::kernel::wallet::signer::SignerKeypair;
use crate::modules::builder::config::TIP_FEE_LAMPORTS;
use crate::modules::decoder::config::PROGRAM_ID_PUBKEYS;
use crate::modules::decoder::program_decoder::ProgramDecoder;
use crate::modules::decoder::pump_fun::config::PUMP_FUN_PROGRAM_ID_PUBKEY;
use crate::modules::decoder::pump_fun::pump_fun_decoder::PumpFunDecoder;
use crate::modules::decoder::types::{FilteredInstruction, ParsedInstruction, ParsedTransaction};
use anyhow::{Result, bail};
use solana_sdk::pubkey::Pubkey;
use std::time::Instant;
use yellowstone_grpc_proto::geyser::{SubscribeUpdateTransaction, SubscribeUpdateTransactionInfo};
use yellowstone_grpc_proto::prelude::TokenBalance;

pub struct TransactionDecoder;

impl TransactionDecoder {
    pub fn decode(transaction: &SubscribeUpdateTransaction) -> Result<Option<ParsedTransaction>> {
        let start = Instant::now();

        let slot = transaction.slot as i32;

        let Some(transaction) = transaction.transaction.as_ref() else {
            return Ok(None);
        };

        let Some(meta) = transaction.meta.as_ref() else {
            return Ok(None);
        };

        let Some(account_keys) = &Self::get_account_keys(transaction)? else {
            return Ok(None);
        };

        let Some(program_id) = Self::filter_programs(account_keys) else {
            return Ok(None);
        };

        let Some(instruction) = &Self::filter_instructions(program_id, transaction, account_keys)
        else {
            return Ok(None);
        };

        let priority_fee = PriorityFeeCache::get();

        let transaction_status = match meta.err.is_none() {
            true => TradeStatus::Success,
            false => TradeStatus::Failed,
        };

        let instruction = Self::parse_instruction(
            &transaction_status,
            program_id,
            instruction,
            account_keys,
            &meta.pre_token_balances,
            &meta.post_token_balances,
        )?;

        let mut total_tx_fee = meta.fee;

        if &instruction.wallet == SignerKeypair::pubkey() {
            total_tx_fee += TIP_FEE_LAMPORTS;
        }

        let tx_fee = Lamports::lamports_to_decimal(total_tx_fee, 9);

        let parsed_transaction = ParsedTransaction {
            status: transaction_status,
            signature: transaction.signature.clone(),
            instruction,
            tx_fee,
            slot,
            priority_fee,
        };

        println!(
            "parsed transaction {} µs",
            start.elapsed().as_nanos() as f64 / 1_000.0
        );

        Ok(Some(parsed_transaction))
    }

    fn get_account_keys(
        transaction: &SubscribeUpdateTransactionInfo,
    ) -> Result<Option<Vec<Pubkey>>> {
        let Some(tx) = &transaction.transaction else { return Ok(None) };

        let Some(msg) = &tx.message else { return Ok(None) };

        let extra = transaction
            .meta
            .as_ref()
            .map(|m| m.loaded_writable_addresses.len() + m.loaded_readonly_addresses.len())
            .unwrap_or(0);

        let mut accounts = Vec::with_capacity(msg.account_keys.len() + extra);

        for k in &msg.account_keys {
            accounts.push(Pubkey::try_from(k.as_slice())?);
        }

        if let Some(meta) = &transaction.meta {
            for k in &meta.loaded_writable_addresses {
                accounts.push(Pubkey::try_from(k.as_slice())?);
            }
            for k in &meta.loaded_readonly_addresses {
                accounts.push(Pubkey::try_from(k.as_slice())?);
            }
        }

        Ok(Some(accounts))
    }

    fn filter_programs(account_keys: &[Pubkey]) -> Option<Pubkey> {
        let mut program_id: Option<Pubkey> = None;
        let mut count = 0;

        for &account in account_keys {
            if !Self::check_program_id(account) {
                continue;
            }

            count += 1;

            if count > 1 {
                return None;
            }

            program_id = Some(account);
        }

        program_id
    }

    fn filter_instructions<'a>(
        program_id: Pubkey,
        transaction: &'a SubscribeUpdateTransactionInfo,
        account_keys: &[Pubkey],
    ) -> Option<FilteredInstruction<'a>> {
        match program_id {
            PUMP_FUN_PROGRAM_ID_PUBKEY => {
                PumpFunDecoder::filter_instructions(transaction, account_keys)
            }
            _ => None,
        }
    }

    fn parse_instruction(
        status: &TradeStatus,
        program_id: Pubkey,
        instruction: &FilteredInstruction,
        account_keys: &[Pubkey],
        pre_tb: &[TokenBalance],
        post_tb: &[TokenBalance],
    ) -> Result<ParsedInstruction> {
        match program_id {
            PUMP_FUN_PROGRAM_ID_PUBKEY => PumpFunDecoder::parse_instruction(
                status,
                instruction,
                account_keys,
                pre_tb,
                post_tb,
            ),
            _ => bail!("unknown program in parse instruction"),
        }
    }

    fn check_program_id(program_id: Pubkey) -> bool {
        PROGRAM_ID_PUBKEYS
            .iter()
            .any(|pubkey| &program_id == pubkey)
    }
}
