use solana_sdk::hash::Hash;
use std::str::FromStr;
use std::sync::LazyLock;
use tokio::sync::watch;

static CACHE: LazyLock<(watch::Sender<Hash>, watch::Receiver<Hash>)> =
    LazyLock::new(|| watch::channel(Hash::default()));

pub struct BlockHashCache;

impl BlockHashCache {
    pub fn set(value: &str) {
        let value = Hash::from_str(value).unwrap();

        let _ = CACHE.0.send(value);
    }

    pub fn get() -> Hash {
        *CACHE.1.borrow()
    }
}
