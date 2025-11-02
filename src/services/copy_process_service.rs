use crate::entities::trade::types::TradeAction;
use crate::kernel::cache::position_cache::PositionCache;
use crate::modules::builder::transaction_builder::TransactionBuilder;
use crate::modules::decoder::config::FILTER_WALLETS;
use crate::modules::decoder::types::ParsedTransaction;
use crate::modules::sender::transaction::TransactionSender;
use crate::services::repository_service::RepositoryService;
use crate::services::transaction_service::TransactionService;
use std::process::exit;

pub struct CopyProcessService;

impl CopyProcessService {
    pub async fn execute(target_transaction: &ParsedTransaction, is_simulate: bool) {
        if !is_simulate && FILTER_WALLETS.len() == 1 {
            tracing::error!(
                "FILTER_WALLETS len = 1, your must set is_simulate = true or add wallets for tracking"
            );
            exit(1);
        }

        let target_position_cache = PositionCache::get(
            &target_transaction.instruction.mint,
            &target_transaction.instruction.wallet,
            false,
        );

        if target_transaction.instruction.action == TradeAction::Sell
            && target_position_cache.is_none()
        {
            tracing::warn!("Target trade sell, but target position not found in cache");
            return;
        }

        let copy_position_cache = PositionCache::get(
            &target_transaction.instruction.mint,
            &target_transaction.instruction.wallet,
            true,
        );

        if target_transaction.instruction.action == TradeAction::Sell
            && copy_position_cache.is_none()
        {
            tracing::warn!("Target trade sell, but copy position not found in cache");
            return;
        }

        let built_transaction = &match TransactionBuilder::build_transaction(target_transaction) {
            Ok(transaction) => transaction,
            Err(error) => {
                tracing::error!("Failed to build swap: {:?}", error);
                return;
            }
        };

        if is_simulate {
            TransactionSender::simulate_transaction(built_transaction).await;
            return;
        }

        let target_signature = &target_transaction.signature;

        let copy_signature = &match TransactionSender::send_base64(built_transaction).await {
            Ok(signature) => bs58::decode(signature).into_vec().unwrap(),
            Err(error) => {
                tracing::error!(error = ?error, "Failed to send base64 transaction");
                return;
            }
        };

        if let Err(error) =
            TransactionService::check_confirmation(target_signature, copy_signature).await
        {
            tracing::error!(error = ?error, "Failed to check confirmation");
            return;
        }

        if let Err(error) =
            RepositoryService::create_or_update_position(target_signature, copy_signature).await
        {
            tracing::error!(error = ?error, "Failed to create or update position");
        };
    }
}
