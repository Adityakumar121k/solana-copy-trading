use crate::entities::position::entity::Position;
use crate::entities::position::types::PositionStatus;
use crate::kernel::db::repositories::position_repository::PositionRepository;
use crate::kernel::wallet::signer::SignerKeypair;
use anyhow::{Context, Result};
use dashmap::DashMap;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};

static CACHE: LazyLock<PositionCache> = LazyLock::new(|| PositionCache { store: DashMap::new() });

#[derive(Debug)]
pub struct PositionCache {
    store: DashMap<Pubkey, DashMap<(Pubkey, Pubkey), Arc<Position>>>,
}

impl PositionCache {
    pub fn init() -> Result<()> {
        let rows = PositionRepository::get_all_opened().context("PositionCache::init")?;

        let cache = &*CACHE;

        let map_by_id_wallet: HashMap<i64, &str> = rows
            .iter()
            .map(|row| (row.id, row.wallet.as_str()))
            .collect();

        for row in &rows {
            let key = match row
                .target_position_id
                .and_then(|id| map_by_id_wallet.get(&id).copied())
            {
                Some(target_wallet) => Self::get_key(&Pubkey::from_str_const(target_wallet), true),
                None => Self::get_key(&Pubkey::from_str_const(&row.wallet), false),
            };

            cache
                .store
                .entry(Pubkey::from_str_const(&row.mint))
                .or_default()
                .insert(key, Arc::new(row.clone()));
        }

        tracing::info!("Position cache init, count = {}", rows.len());

        Ok(())
    }

    pub fn get(mint: &Pubkey, target_wallet: &Pubkey, is_copy: bool) -> Option<Arc<Position>> {
        CACHE.store.get(mint).and_then(|mint| {
            mint.value()
                .get(&Self::get_key(target_wallet, is_copy))
                .map(|p| Arc::clone(&p))
        })
    }

    pub fn set(mint: &Pubkey, target_wallet: &Pubkey, position: &Position, is_copy: bool) {
        Self::insert(
            mint,
            Self::get_key(target_wallet, is_copy),
            Arc::new(position.clone()),
        );
    }

    pub fn update(mint: &Pubkey, target_wallet: &Pubkey, position: &Position, is_copy: bool) {
        let key = Self::get_key(target_wallet, is_copy);

        if position.status == PositionStatus::Closed {
            if let Some(mut outer) = CACHE.store.get_mut(mint) {
                let is_empty = {
                    let inner = outer.value_mut();
                    inner.remove(&key);
                    inner.is_empty()
                };

                drop(outer);

                if is_empty {
                    CACHE.store.remove(mint);
                }
            }
        } else {
            Self::insert(mint, key, Arc::new(position.clone()));
        }
    }

    #[inline]
    fn insert(mint: &Pubkey, key: (Pubkey, Pubkey), position: Arc<Position>) {
        CACHE.store.entry(*mint).or_default().insert(key, position);
    }

    #[inline]
    pub fn get_key(target_wallet: &Pubkey, is_copy: bool) -> (Pubkey, Pubkey) {
        match is_copy {
            true => (*target_wallet, *SignerKeypair::pubkey()),
            false => (*target_wallet, *target_wallet),
        }
    }
}
