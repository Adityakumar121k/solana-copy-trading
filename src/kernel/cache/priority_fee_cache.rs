use std::sync::LazyLock;
use tokio::sync::watch;

static CACHE: LazyLock<(watch::Sender<u64>, watch::Receiver<u64>)> =
    LazyLock::new(|| watch::channel(u64::default()));

pub struct PriorityFeeCache;

impl PriorityFeeCache {
    pub fn set(value: u64) {
        let _ = CACHE.0.send(value);
    }

    pub fn get() -> u64 {
        *CACHE.1.borrow()
    }
}
