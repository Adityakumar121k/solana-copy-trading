use std::env;
use std::sync::LazyLock;

pub static GRPC_ENDPOINT: LazyLock<String> =
    LazyLock::new(|| env::var("GRPC_ENDPOINT").expect("GRPC_ENDPOINT env not set"));

pub static SIMULATE_MODE: LazyLock<bool> = LazyLock::new(|| {
    env::var("SIMULATE_MODE")
        .expect("SIMULATE_MODE env not set")
        .parse::<bool>()
        .expect("SIMULATE_MODE should be true/false")
});

pub static FOLLOW_WALLETS: LazyLock<Vec<String>> = LazyLock::new(|| {
    let wallets = env::var("FOLLOW_WALLETS").expect("FOLLOW_WALLETS env not set");

    wallets
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
});
