use crate::kernel::cache::position_cache::PositionCache;
use crate::kernel::utils::lamports::Lamports;
use crate::modules::builder::config::{ORDER_SOL_LAMPORTS, SLIPPAGE_MULTIPLIER};
use anyhow::{Context, Result};
use rust_decimal::Decimal;
use solana_sdk::pubkey::Pubkey;

pub struct Amounts;

impl Amounts {
    pub fn token_from_sol(price: Decimal, token_decimals: u32) -> Result<(u64, u64)> {
        let base_amount =
            Lamports::decimal_to_lamports(*ORDER_SOL_LAMPORTS / price, token_decimals)?;

        let quote_amount =
            Lamports::decimal_to_lamports(*ORDER_SOL_LAMPORTS * *SLIPPAGE_MULTIPLIER, 9)?;

        Ok((base_amount, quote_amount))
    }

    pub fn sol_from_token(
        mint: &Pubkey,
        target_wallet: &Pubkey,
        amount: Decimal,
        token_decimals: u32,
    ) -> Result<(u64, u64)> {
        let target_position =
            PositionCache::get(mint, target_wallet, false).with_context(|| {
                format!(
                    "Target position not found: mint={mint}, key={:?}",
                    PositionCache::get_key(target_wallet, false),
                )
            })?;

        let copy_position = PositionCache::get(mint, target_wallet, true).with_context(|| {
            format!(
                "Copy position not found: mint={mint}, key={:?}",
                PositionCache::get_key(target_wallet, true),
            )
        })?;

        let percent = Decimal::from(1).min(amount / target_position.amount_left);
        let amount = copy_position.amount_left * percent;

        Ok((
            Lamports::decimal_to_lamports(amount, token_decimals)?,
            0_u64,
        ))
    }
}
