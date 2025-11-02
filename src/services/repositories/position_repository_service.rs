use crate::entities::position::entity::Position;
use crate::entities::position::entity::{NewPosition, UpdatePosition};
use crate::entities::position::types::PositionStatus;
use crate::entities::trade::types::TradeAction;
use crate::kernel::cache::position_cache::PositionCache;
use crate::kernel::cache::transaction_cache::TxCacheValue;
use crate::kernel::db::repositories::position_repository::PositionRepository;
use crate::kernel::db::repositories::trade_repository::TradeRepository;
use crate::services::repositories::trade_repository_service::TradeRepositoryService;
use anyhow::Result;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

pub struct PositionRepositoryService;

impl PositionRepositoryService {
    pub fn create(
        target_wallet: &Pubkey,
        is_copy: bool,
        data: &Arc<TxCacheValue>,
        target_position_id: Option<i64>,
    ) -> Result<Position> {
        let new_position = &NewPosition {
            target_position_id,
            wallet: data.wallet.to_string(),
            mint: data.mint.to_string(),
            amount_total: data.token_amount,
            amount_left: data.token_amount,
            avg_buy_price: data.price,
            avg_sell_price: None,
            total_fee: data.tx_fee + data.trade_fee,
            realized_pnl: None,
            amm: data.program.to_string(),
            status: PositionStatus::Opened,
        };

        let position = PositionRepository::create(new_position)?;

        PositionCache::set(&data.mint, target_wallet, &position, is_copy);

        Ok(position)
    }

    pub fn update(
        target_wallet: &Pubkey,
        is_copy: bool,
        data: &Arc<TxCacheValue>,
        position: Position,
    ) -> Result<Position> {
        let amount_total = match &data.action {
            TradeAction::Buy => position.amount_total + data.token_amount,
            TradeAction::Sell => position.amount_total,
        };

        let amount_left = match &data.action {
            TradeAction::Buy => position.amount_left + data.token_amount,
            TradeAction::Sell => Decimal::zero().max(position.amount_left - data.token_amount),
        };

        let status = PositionStatus::predict_from_amounts(amount_left, amount_total);
        let trades = &TradeRepository::get_all_by_position_id(position.id)?;
        let realized_pnl = (status == PositionStatus::Closed)
            .then(|| TradeRepositoryService::get_realized_pnl(trades));
        let (avg_buy_price, avg_sell_price) = TradeRepositoryService::get_avg_prices(trades);
        let total_fee = Some(position.total_fee + data.tx_fee + data.trade_fee);
        let closed_at = (status == PositionStatus::Closed).then(|| Utc::now().naive_utc());

        let position = PositionRepository::update(
            position.id,
            &UpdatePosition {
                amount_total: Some(amount_total),
                amount_left: Some(amount_left),
                avg_buy_price,
                avg_sell_price,
                total_fee,
                realized_pnl,
                status: Some(status),
                closed_at,
                ..Default::default()
            },
        )?;

        PositionCache::update(&data.mint, target_wallet, &position, is_copy);

        Ok(position)
    }
}
