use crate::entities::trade::entity::NewTrade;
use crate::entities::trade::entity::Trade;
use crate::entities::trade::types::TradeAction;
use crate::kernel::cache::transaction_cache::TxCacheValue;
use crate::kernel::db::repositories::trade_repository::TradeRepository;
use anyhow::Result;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use std::sync::Arc;

pub struct TradeRepositoryService;

impl TradeRepositoryService {
    pub fn create(
        data: &Arc<TxCacheValue>,
        position_id: i64,
        target_trade_id: Option<i64>,
    ) -> Result<Trade> {
        let new_trade = &NewTrade {
            position_id,
            target_trade_id,
            wallet: data.wallet.to_string(),
            signature: bs58::encode(&data.signature).into_string(),
            action: data.action,
            status: data.status,
            mint: data.mint.to_string(),
            amount: data.token_amount,
            price: data.price,
            trade_fee: data.trade_fee,
            tx_fee: data.tx_fee,
            amm: data.program.to_string(),
            slot: data.slot,
        };

        TradeRepository::create(new_trade)
    }

    pub fn get_avg_prices(trades: &[Trade]) -> (Option<Decimal>, Option<Decimal>) {
        let mut sum_buy = Decimal::zero();
        let mut sum_buy_amount = Decimal::zero();
        let mut sum_sell = Decimal::zero();
        let mut sum_sell_amount = Decimal::zero();

        for trade in trades {
            if trade.action == TradeAction::Buy {
                sum_buy += trade.price * trade.amount;
                sum_buy_amount += trade.amount;
            }

            if trade.action == TradeAction::Sell {
                sum_sell += trade.price * trade.amount;
                sum_sell_amount += trade.amount;
            }
        }

        (
            (sum_buy_amount > Decimal::zero()).then(|| sum_buy / sum_buy_amount),
            (sum_sell_amount > Decimal::zero()).then(|| sum_sell / sum_sell_amount),
        )
    }

    pub fn get_realized_pnl(trades: &[Trade]) -> Decimal {
        let mut buy_sum = Decimal::zero();
        let mut sell_sum = Decimal::zero();
        let mut total_fee = Decimal::zero();

        for trade in trades {
            match trade.action {
                TradeAction::Buy => {
                    buy_sum += trade.price * trade.amount;
                }
                TradeAction::Sell => {
                    sell_sum += trade.price * trade.amount;
                }
            }

            total_fee += trade.trade_fee + trade.tx_fee;
        }

        sell_sum - buy_sum - total_fee
    }
}
