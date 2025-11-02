use anyhow::Result;
use copy_trade::modules::decoder::pump_fun::config::PUMP_FUN_PROGRAM_ID_PUBKEY;
use copy_trade::modules::decoder::transaction_decoder::TransactionDecoder;
use copy_trade::modules::stream::config::GRPC_ENDPOINT;
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use dotenvy::dotenv;
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio::time::Duration;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use yellowstone_grpc_proto::geyser::subscribe_update::UpdateOneof;
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterTransactions,
    SubscribeUpdateTransaction,
};
use yellowstone_grpc_proto::tonic::codec::CompressionEncoding;

async fn collect_real_transactions(count: usize) -> Result<Vec<SubscribeUpdateTransaction>> {
    dotenv().ok();

    let mut client = GeyserGrpcClient::build_from_shared(GRPC_ENDPOINT.clone())?
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .tcp_keepalive(Some(Duration::from_secs(30)))
        .http2_keep_alive_interval(Duration::from_secs(20))
        .keep_alive_while_idle(true)
        .keep_alive_timeout(Duration::from_secs(5))
        .send_compressed(CompressionEncoding::Gzip)
        .connect()
        .await?;

    let (mut subscribe, mut stream) = client.subscribe().await?;

    let filter = HashMap::from([(
        "filter".into(),
        SubscribeRequestFilterTransactions {
            vote: Some(false),
            failed: Some(false),
            account_required: vec![PUMP_FUN_PROGRAM_ID_PUBKEY.to_string()],
            ..Default::default()
        },
    )]);

    subscribe
        .send(SubscribeRequest {
            slots: HashMap::new(),
            accounts: HashMap::new(),
            transactions: filter,
            transactions_status: HashMap::new(),
            entry: HashMap::new(),
            blocks: HashMap::new(),
            blocks_meta: HashMap::new(),
            commitment: Some(CommitmentLevel::Processed as i32),
            accounts_data_slice: vec![],
            ping: None,
            from_slot: None,
        })
        .await?;

    let mut transactions = Vec::new();

    let timeout = tokio::time::timeout(Duration::from_secs(60), async {
        while let Some(message) = stream.next().await {
            match message {
                Ok(message) => {
                    if let Some(UpdateOneof::Transaction(transaction)) = message.update_oneof {
                        transactions.push(transaction);

                        if transactions.len() >= count {
                            break;
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Transaction stream error: {:?}", error);
                    break;
                }
            }
        }
    })
    .await;

    if timeout.is_err() {
        eprintln!(
            "Timeout collecting transactions, got {} instead of {}",
            transactions.len(),
            count
        );
    }

    Ok(transactions)
}

fn decode_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let transactions = rt
        .block_on(collect_real_transactions(1000))
        .unwrap_or_else(|e| {
            eprintln!("Failed to collect real transactions: {:?}", e);
            Vec::new()
        });

    if transactions.is_empty() {
        eprintln!("No real transactions collected, skipping benchmark");
        return;
    }

    let mut group = c.benchmark_group("transaction_decode");
    group.throughput(Throughput::Elements(transactions.len() as u64));

    group.bench_function(BenchmarkId::new("decode_all", transactions.len()), |b| {
        b.iter(|| {
            for tx in &transactions {
                let _ = TransactionDecoder::decode(black_box(tx));
            }
        })
    });

    group.finish();
}

criterion_group!(benches, decode_benchmark);

criterion_main!(benches);
