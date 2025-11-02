use anyhow::{Result, bail};
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;

pub struct Price;

impl Price {
    pub fn get_price(token_amount: Decimal, sol_amount: Decimal) -> Result<Decimal> {
        if token_amount <= Decimal::zero() {
            bail!("Token amount must be greater than zero");
        }

        Ok(sol_amount / token_amount)
    }
}
