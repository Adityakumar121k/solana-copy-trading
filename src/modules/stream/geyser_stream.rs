use crate::kernel::cache::block_hash_cache::BlockHashCache;
use crate::kernel::cache::transaction_cache::TransactionCache;
use crate::kernel::wallet::signer::SignerKeypair;
use crate::modules::decoder::pump_fun::config::PUMP_FUN_PROGRAM_ID_PUBKEY;
use crate::modules::decoder::transaction_decoder::TransactionDecoder;
use crate::modules::stream::config::{FOLLOW_WALLETS, GRPC_ENDPOINT, SIMULATE_MODE};
use crate::services::copy_process_service::CopyProcessService;
use anyhow::Result;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio::time::{Duration, sleep};
use yellowstone_grpc_client::{
    ClientTlsConfig, GeyserGrpcBuilderResult, GeyserGrpcClient, Interceptor,
};
use yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof;
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterBlocksMeta,
    SubscribeRequestFilterTransactions,
};

pub struct GeyserStream;

impl GeyserStream {
    pub async fn transaction_self() -> Result<()> {
        loop {
            sleep(Duration::from_secs(5)).await;

            tracing::info!("Stream transaction SELF started");

            let filter = HashMap::from([
                (
                    "failed".into(),
                    SubscribeRequestFilterTransactions {
                        vote: Some(false),
                        failed: Some(true),
                        account_required: vec![SignerKeypair::pubkey().to_string()],
                        ..Default::default()
                    },
                ),
                (
                    "success".into(),
                    SubscribeRequestFilterTransactions {
                        vote: Some(false),
                        failed: Some(false),
                        account_required: vec![SignerKeypair::pubkey().to_string()],
                        ..Default::default()
                    },
                ),
            ]);

            match Self::transaction(filter, CommitmentLevel::Processed, false).await {
                Ok(()) => {
                    tracing::warn!("Transaction SELF stream closed, reconnecting...");
                }
                Err(error) => {
                    tracing::error!(error =? error, "Transaction SELF stream closed, reconnecting...");
                }
            }
        }
    }

    pub async fn transaction_target() -> Result<()> {
        loop {
            sleep(Duration::from_secs(6)).await;

            tracing::info!("Stream transaction TARGET started");

            let filter = HashMap::from([(
                "filter".into(),
                SubscribeRequestFilterTransactions {
                    vote: Some(false),
                    failed: Some(false),
                    account_include: FOLLOW_WALLETS.clone(),
                    account_required: vec![PUMP_FUN_PROGRAM_ID_PUBKEY.to_string()],
                    ..Default::default()
                },
            )]);

            match Self::transaction(filter, CommitmentLevel::Processed, true).await {
                Ok(()) => {
                    tracing::warn!("Transaction TARGET stream closed, reconnecting...");
                }
                Err(error) => {
                    tracing::error!(error =? error, "Transaction TARGET stream closed, reconnecting...");
                }
            }
        }
    }

    async fn transaction(
        filter: HashMap<String, SubscribeRequestFilterTransactions>,
        commitment: CommitmentLevel,
        is_exec: bool,
    ) -> Result<()> {
        let mut client = Self::init_client().await?;
        let (mut subscribe, mut stream) = client.subscribe().await?;

        subscribe
            .send(SubscribeRequest {
                slots: HashMap::new(),
                accounts: HashMap::new(),
                transactions: filter,
                transactions_status: HashMap::new(),
                entry: HashMap::new(),
                blocks: HashMap::new(),
                blocks_meta: HashMap::new(),
                commitment: Some(commitment as i32),
                accounts_data_slice: vec![],
                ping: None,
                from_slot: None,
            })
            .await?;

        while let Some(message) = stream.next().await {
            match message {
                Ok(message) => {
                    if let Some(UpdateOneof::Transaction(transaction)) = message.update_oneof {
                        let parsed_transaction = match TransactionDecoder::decode(&transaction) {
                            Ok(parsed_transaction) => parsed_transaction,
                            Err(error) => {
                                tracing::error!(error = ?error, "Parse transaction:");
                                continue;
                            }
                        };

                        if let Some(parsed_transaction) = &parsed_transaction {
                            TransactionCache::set(
                                parsed_transaction.signature.clone(),
                                parsed_transaction,
                            );

                            if is_exec {
                                CopyProcessService::execute(parsed_transaction, *SIMULATE_MODE)
                                    .await;
                            }
                        }
                    }
                }
                Err(error) => {
                    tracing::error!(error = ?error, "Transaction stream");
                }
            }
        }

        Ok(())
    }

    pub async fn blocks_meta() -> Result<()> {
        loop {
            tracing::info!("Stream block meta started");

            let mut client = Self::init_client().await?;
            let (mut subscribe, mut stream) = client.subscribe().await?;

            let block_meta_filters =
                HashMap::from([("all_blocks".into(), SubscribeRequestFilterBlocksMeta {})]);

            subscribe
                .send(SubscribeRequest {
                    slots: HashMap::new(),
                    accounts: HashMap::new(),
                    transactions: HashMap::new(),
                    transactions_status: HashMap::new(),
                    entry: HashMap::new(),
                    blocks: HashMap::new(),
                    blocks_meta: block_meta_filters,
                    commitment: Some(CommitmentLevel::Finalized as i32),
                    accounts_data_slice: vec![],
                    ping: None,
                    from_slot: None,
                })
                .await?;

            while let Some(message) = stream.next().await {
                match message {
                    Ok(msg) => {
                        if let Some(UpdateOneof::BlockMeta(block)) = msg.update_oneof {
                            BlockHashCache::set(&block.blockhash);
                        }
                    }
                    Err(error) => {
                        tracing::error!(error = ?error, "Block meta stream");
                        break;
                    }
                }
            }

            tracing::warn!("Blick meta stream closed, reconnecting...");
        }
    }

    async fn init_client() -> GeyserGrpcBuilderResult<GeyserGrpcClient<impl Interceptor>> {
        GeyserGrpcClient::build_from_shared(GRPC_ENDPOINT.clone())?
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .tcp_keepalive(Some(Duration::from_secs(30)))
            .http2_keep_alive_interval(Duration::from_secs(20))
            .keep_alive_while_idle(true)
            .keep_alive_timeout(Duration::from_secs(5))
            .connect()
            .await
    }
}
