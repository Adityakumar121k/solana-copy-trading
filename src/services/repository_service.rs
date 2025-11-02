use crate::entities::position::entity::Position;
use crate::entities::trade::entity::Trade;
use crate::entities::trade::types::{TradeAction, TradeStatus};
use crate::kernel::cache::transaction_cache::{TransactionCache, TxCacheValue};
use crate::kernel::db::connection::Db;
use crate::kernel::db::repositories::position_repository::PositionRepository;
use crate::services::repositories::position_repository_service::PositionRepositoryService;
use crate::services::repositories::trade_repository_service::TradeRepositoryService;
use anyhow::{Error, Result};
use diesel::Connection;
use solana_sdk::pubkey::Pubkey;
use std::sync::Arc;

pub struct RepositoryService;

impl RepositoryService {
    pub async fn create_or_update_position(
        target_signature: &[u8],
        copy_signature: &[u8],
    ) -> Result<()> {
        let target_transaction_cache = &TransactionCache::get(target_signature, true)?;
        let copy_transaction_cache = &TransactionCache::get(copy_signature, true)?;

        let target_position_opened = PositionRepository::get_opened(
            &target_transaction_cache.mint.to_string(),
            &target_transaction_cache.wallet.to_string(),
        )?;

        let copy_position_opened = PositionRepository::get_opened(
            &copy_transaction_cache.mint.to_string(),
            &copy_transaction_cache.wallet.to_string(),
        )?;

        if target_position_opened.is_none() && copy_transaction_cache.status == TradeStatus::Failed
        {
            return Ok(());
        }

        if copy_position_opened.is_none() && copy_transaction_cache.action == TradeAction::Sell {
            tracing::info!(
                "Skip copy trade, position in cache not found\nsignature = {}\nmint = {}\nwallet = {}",
                bs58::encode(&copy_transaction_cache.signature).into_string(),
                copy_transaction_cache.mint.to_string(),
                copy_transaction_cache.wallet.to_string(),
            );

            return Ok(());
        }

        let (target_position, target_trade) = match target_position_opened.is_none() {
            true => Self::create_position_and_trade(
                &target_transaction_cache.wallet,
                false,
                target_transaction_cache,
                None,
                None,
            )?,
            false => Self::update_position_and_trade(
                &target_transaction_cache.wallet,
                false,
                target_transaction_cache,
                target_position_opened.unwrap(),
                None,
            )?,
        };

        if copy_position_opened.is_none() {
            if copy_transaction_cache.status == TradeStatus::Success {
                Self::create_position_and_trade(
                    &target_transaction_cache.wallet,
                    true,
                    copy_transaction_cache,
                    Some(target_position.id),
                    Some(target_trade.id),
                )?;
            }
        } else {
            Self::update_position_and_trade(
                &target_transaction_cache.wallet,
                true,
                copy_transaction_cache,
                copy_position_opened.unwrap(),
                Some(target_trade.id),
            )?;
        }

        Ok(())
    }

    fn create_position_and_trade(
        target_wallet: &Pubkey,
        is_copy: bool,
        data: &Arc<TxCacheValue>,
        target_position_id: Option<i64>,
        target_trade_id: Option<i64>,
    ) -> Result<(Position, Trade)> {
        Db::get_connection().transaction::<_, Error, _>(|_| {
            let position = PositionRepositoryService::create(
                target_wallet,
                is_copy,
                data,
                target_position_id,
            )?;

            let trade = TradeRepositoryService::create(data, position.id, target_trade_id)?;

            Ok((position, trade))
        })
    }

    fn update_position_and_trade(
        target_wallet: &Pubkey,
        is_copy: bool,
        data: &Arc<TxCacheValue>,
        position: Position,
        target_trade_id: Option<i64>,
    ) -> Result<(Position, Trade)> {
        Db::get_connection().transaction::<_, Error, _>(|_| {
            let trade = TradeRepositoryService::create(data, position.id, target_trade_id)?;

            let position =
                PositionRepositoryService::update(target_wallet, is_copy, data, position)?;

            Ok((position, trade))
        })
    }
}
