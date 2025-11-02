use crate::entities::trade::types::{TradeAction, TradeStatus};
use crate::modules::decoder::types::ParsedTransaction;
use anyhow::{Context, Result};
use dashmap::DashMap;
use rust_decimal::Decimal;
use solana_sdk::pubkey::Pubkey;
use std::sync::{Arc, LazyLock};
use std::time::Instant;

static CACHE: LazyLock<TransactionCache> =
    LazyLock::new(|| TransactionCache { store: DashMap::new() });

#[derive(Debug)]
pub struct TxCacheValue {
    pub time: Instant,
    pub program: Pubkey,
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub sol_amount: Decimal,
    pub token_amount: Decimal,
    pub price: Decimal,
    pub trade_fee: Decimal,
    pub tx_fee: Decimal,
    pub status: TradeStatus,
    pub action: TradeAction,
    pub signature: Vec<u8>,
    pub slot: i32,
}

pub struct TransactionCache {
    store: DashMap<Vec<u8>, Arc<TxCacheValue>>,
}

impl TransactionCache {
    pub fn set(key: Vec<u8>, parsed_transaction: &ParsedTransaction) {
        let data = Self::prepare_cache(parsed_transaction);
        CACHE.store.insert(key, Arc::new(data));
    }

    pub fn get(key: &[u8], is_drop: bool) -> Result<Arc<TxCacheValue>> {
        if is_drop {
            CACHE.store.remove(key).map(|(_, v)| v)
        } else {
            CACHE.store.get(key).map(|v| Arc::clone(&v))
        }
        .with_context(|| {
            format!(
                "Failed to retrieve cache\nsignature = {}",
                bs58::encode(key).into_string()
            )
        })
    }

    pub fn prepare_cache(parsed_transaction: &ParsedTransaction) -> TxCacheValue {
        TxCacheValue {
            time: Instant::now(),
            program: parsed_transaction.instruction.program_id,
            wallet: parsed_transaction.instruction.wallet,
            mint: parsed_transaction.instruction.mint,
            sol_amount: parsed_transaction.instruction.sol_amount,
            token_amount: parsed_transaction.instruction.token_amount,
            price: parsed_transaction.instruction.price,
            trade_fee: parsed_transaction.instruction.trade_fee,
            tx_fee: parsed_transaction.tx_fee,
            status: parsed_transaction.status,
            action: parsed_transaction.instruction.action,
            signature: parsed_transaction.signature.clone(),
            slot: parsed_transaction.slot,
        }
    }
}
