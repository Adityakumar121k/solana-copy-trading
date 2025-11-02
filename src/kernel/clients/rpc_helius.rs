use crate::kernel::clients::http::{Http, HttpCfg};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::env;
use std::sync::LazyLock;

pub struct RpcHelius;

static HELIUS_API_KEY: LazyLock<String> =
    LazyLock::new(|| env::var("HELIUS_API_KEY").expect("HELIUS_API_KEY not set"));

static HELIUS_URL: LazyLock<String> =
    LazyLock::new(|| env::var("HELIUS_URL").expect("HELIUS_URL not set"));

static CLIENT: LazyLock<HttpCfg> =
    LazyLock::new(|| RpcHelius::new(HELIUS_API_KEY.clone(), HELIUS_URL.clone()));

static RPC_CLIENT: LazyLock<RpcClient> = LazyLock::new(|| {
    RpcClient::new_with_commitment(
        format!("{}?api-key={}", &*HELIUS_URL, &*HELIUS_API_KEY),
        CommitmentConfig::confirmed(),
    )
});

impl RpcHelius {
    pub fn get_rpc_client() -> &'static RpcClient {
        &RPC_CLIENT
    }
}

impl Http for RpcHelius {
    fn init() {
        let _ = &CLIENT;
        tracing::info!("Connected Helius RPC");
    }

    fn get_cfg() -> &'static HttpCfg {
        &CLIENT
    }
}
