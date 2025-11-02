use crate::kernel::clients::http::Http;
use crate::kernel::clients::rpc_helius::RpcHelius;
use crate::kernel::utils::encoder::Encoder;
use crate::modules::sender::rpc_landing::RpcLanding;
use anyhow::{Result, bail};
use serde_json::{Value, json};
use solana_sdk::transaction::VersionedTransaction;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct TransactionSender;

impl TransactionSender {
    pub async fn send_base64(transaction: &VersionedTransaction) -> Result<String> {
        let start = Instant::now();

        let base64_transaction = Encoder::base64_encode(transaction);

        let params = json!({
            "encoding": "base64",
            "skipPreflight": true,
            "maxRetries": 0,
        });

        let response = RpcLanding::json_rpc::<String>(
            "",
            "sendTransaction",
            Vec::from([Value::String(base64_transaction), params]),
        )
        .await;

        println!(
            "landing transaction {} µs",
            start.elapsed().as_nanos() as f64 / 1_000.0
        );

        match response {
            Ok(response) => Ok(response.result),
            Err(error) => bail!(error),
        }
    }

    pub async fn simulate_transaction(transaction: &VersionedTransaction) {
        let start = Instant::now();

        match RpcHelius::get_rpc_client()
            .simulate_transaction(transaction)
            .await
        {
            Ok(response) => {
                let simulate = response.value;

                if simulate.err.is_none() {
                    tracing::info!("Simulate success");
                }

                if simulate.err.is_some() {
                    tracing::error!("Simulate error");
                    tracing::error!("{:#?}", simulate);
                }
            }

            Err(error) => {
                tracing::error!("Failed to simulate transaction: {:#?}", error);
            }
        }

        println!(
            "simulate transaction {} µs",
            start.elapsed().as_nanos() as f64 / 1_000.0
        );
    }

    pub fn heartbeat() {
        tokio::spawn(async move {
            loop {
                if RpcLanding::json_rpc::<String>("", "getHealth", vec![])
                    .await
                    .is_err()
                {
                    sleep(Duration::from_secs(3)).await;
                    continue;
                }

                sleep(Duration::from_secs(60)).await;
            }
        });
    }
}
