use crate::entities::trade::types::TradeStatus;
use crate::kernel::cache::transaction_cache::{TransactionCache, TxCacheValue};
use anyhow::Result;
use rust_decimal::Decimal;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

const DURATION: Duration = Duration::from_secs(60);

pub struct TransactionService;

impl TransactionService {
    pub async fn check_confirmation(
        target_signature: &Vec<u8>,
        copy_signature: &Vec<u8>,
    ) -> Result<()> {
        let start = Instant::now();

        while start.elapsed() < DURATION {
            if TransactionCache::get(copy_signature, false).is_ok() {
                break;
            }

            sleep(Duration::from_millis(50)).await;
        }

        let target = TransactionCache::get(target_signature, false)?;
        let copy = TransactionCache::get(copy_signature, false)?;

        Self::log_trade_pair(&target, &copy, target_signature, copy_signature);

        Ok(())
    }

    pub fn log_trade_pair(
        target: &Arc<TxCacheValue>,
        copy: &Arc<TxCacheValue>,
        target_signature: &Vec<u8>,
        copy_signature: &Vec<u8>,
    ) {
        let target_signature_bs58 = bs58::encode(target_signature).into_string();
        let copy_signature_bs58 = bs58::encode(copy_signature).into_string();
        let diff_time_s = copy.time.duration_since(target.time).as_secs_f64();
        let diff_price: Decimal = ((target.price - copy.price) / target.price) * Decimal::from(100);
        let diff_slot = copy.slot.saturating_sub(target.slot);
        let copy_fee = copy.tx_fee + copy.trade_fee;

        let details = format!(
            "\n\
            program       = {}\n\
            action        = {:?}\n\
            mint          = {}\n\
            target_user   = {}\n\
            copy_user     = {}\n\
            target_sig    = {}\n\
            copy_sig      = {}\n\
            target_status = {:?}\n\
            copy_status   = {:?}\n\
            target_token  = {}\n\
            copy_token    = {}\n\
            target_sol    = {}\n\
            copy_sol      = {}\n\
            target_fee    = {}\n\
            copy_fee      = {}\n\
            diff_time_s   = {}s\n\
            diff_price    = {:.4}\n\
            diff_slot     = {}",
            target.program,
            target.action,
            target.mint,
            target.wallet,
            copy.wallet,
            target_signature_bs58,
            copy_signature_bs58,
            target.status,
            copy.status,
            target.token_amount,
            copy.token_amount,
            target.sol_amount,
            copy.sol_amount,
            "not calculated",
            copy_fee,
            diff_time_s,
            diff_price,
            diff_slot,
        );

        if copy.status == TradeStatus::Success {
            tracing::info!("{details}");
        } else {
            tracing::error!("{details}");
        }
    }
}
