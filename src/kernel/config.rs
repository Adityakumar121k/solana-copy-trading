use crate::modules::builder::config::{
    ORDER_SOL_LAMPORTS, POSITION_CLOSE_PERCENT, SLIPPAGE_MULTIPLIER, TIP_RECEIVERS,
};
use crate::modules::builder::pump_fun::types::USER_VOLUME_ACCUMULATOR;
use crate::modules::decoder::config::{DISCRIMINATORS, FILTER_WALLETS};
use crate::modules::stream::config::{FOLLOW_WALLETS, GRPC_ENDPOINT, SIMULATE_MODE};
use std::env;
use std::sync::LazyLock;

pub struct Config;

impl Config {
    pub fn init_configs() {
        let _ = &*DATABASE_URL;
        let _ = &*GRPC_ENDPOINT;
        let _ = &*SIMULATE_MODE;

        // Instruction config
        let _ = &*ORDER_SOL_LAMPORTS;
        let _ = &*SLIPPAGE_MULTIPLIER;
        let _ = &*POSITION_CLOSE_PERCENT;
        let _ = &*DISCRIMINATORS;
        let _ = &*FOLLOW_WALLETS;
        let _ = &*FILTER_WALLETS;

        // Pump Fun
        let _ = &*USER_VOLUME_ACCUMULATOR;

        // Lending
        let _ = &*TIP_RECEIVERS;

        tracing::info!("Init config");
    }
}

pub static DATABASE_URL: LazyLock<String> =
    LazyLock::new(|| env::var("DATABASE_URL").expect("DATABASE_URL env not set"));
