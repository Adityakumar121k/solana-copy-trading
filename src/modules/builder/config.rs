use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use solana_sdk::pubkey::Pubkey;
use std::env;
use std::sync::LazyLock;

pub static JITO_DONT_FRONT_ADDRESS: Pubkey = Pubkey::new_from_array([
    10, 241, 195, 67, 33, 136, 202, 58, 99, 81, 53, 161, 58, 24, 149, 26, 206, 189, 41, 230, 172,
    45, 174, 103, 255, 219, 6, 215, 64, 0, 0, 0,
]);

pub const TIP_FEE_LAMPORTS: u64 = 100_000_u64; // 0.0001 SOL

pub const COMPUTE_UNIT_LIMIT: u32 = 140_000_u32;

pub static TIP_RECEIVERS: LazyLock<[Pubkey; 5]> = LazyLock::new(|| {
    [
        // 0slot
        "Eb2KpSC8uMt9GmzyAEm5Eb1AAAgTjRaXWFjKyFXHZxF3"
            .parse()
            .unwrap(),
        "FCjUJZ1qozm1e8romw216qyfQMaaWKxWsuySnumVCCNe"
            .parse()
            .unwrap(),
        "ENxTEjSQ1YabmUpXAdCgevnHQ9MHdLv8tzFiuiYJqa13"
            .parse()
            .unwrap(),
        "6rYLG55Q9RpsPGvqdPNJs4z5WTxJVatMB8zV3WJhs5EK"
            .parse()
            .unwrap(),
        "Cix2bHfqPcKcM233mzxbLk14kSggUUiz2A87fJtGivXr"
            .parse()
            .unwrap(),
    ]
});

// Astraline
// "astrazznxsGUhWShqgNtAdfrzP2G83DzcWVJDxwV9bF".parse().unwrap(),
// "astra4uejePWneqNaJKuFFA8oonqCE1sqF6b45kDMZm".parse().unwrap(),
// "astra9xWY93QyfG6yM8zwsKsRodscjQ2uU2HKNL5prk".parse().unwrap(),
// "astraRVUuTHjpwEVvNBeQEgwYx9w9CFyfxjYoobCZhL".parse().unwrap(),

pub static ORDER_SOL_LAMPORTS: LazyLock<Decimal> = LazyLock::new(|| {
    let value: f64 = env::var("ORDER_SOL_AMOUNT")
        .expect("ORDER_SOL_AMOUNT env not set")
        .parse()
        .expect("ORDER_SOL_AMOUNT must be a number");

    Decimal::from_f64(value).unwrap()
});

pub static SLIPPAGE_MULTIPLIER: LazyLock<Decimal> = LazyLock::new(|| {
    let value: f64 = env::var("SLIPPAGE_PERCENT")
        .expect("SLIPPAGE_PERCENT env not set")
        .parse()
        .expect("SLIPPAGE_PERCENT must be a number");

    Decimal::from(1) + Decimal::from_f64(value / 100_f64).unwrap()
});

pub static POSITION_CLOSE_PERCENT: LazyLock<Decimal> = LazyLock::new(|| {
    let value: f64 = env::var("POSITION_CLOSE_PERCENT")
        .expect("POSITION_CLOSE_PERCENT env not set")
        .parse()
        .expect("POSITION_CLOSE_PERCENT must be a number");

    Decimal::from_f64(value).unwrap()
});
