use anyhow::{Context, Result};
use rust_decimal::Decimal;
use rust_decimal::prelude::{ToPrimitive, Zero};

pub struct Lamports;

impl Lamports {
    pub fn decimal_to_lamports(token_amount: Decimal, token_decimals: u32) -> Result<u64> {
        (token_amount * Decimal::from(10_u64.pow(token_decimals)))
            .to_u64()
            .context("Cao not converted decimal to lamports")
    }

    pub fn lamports_to_decimal(token_amount: u64, token_decimals: u32) -> Decimal {
        if token_amount == 0 {
            return Decimal::zero();
        }

        Decimal::from(token_amount) / Decimal::from(10_u64.pow(token_decimals))
    }
}
