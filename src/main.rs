use anyhow::Result;
use copy_trade::kernel::cache::position_cache::PositionCache;
use copy_trade::kernel::clients::http::Http;
use copy_trade::kernel::clients::rpc_helius::RpcHelius;
use copy_trade::kernel::config::Config;
use copy_trade::kernel::logger::tracing::Tracing;
use copy_trade::kernel::wallet::signer::SignerKeypair;
use copy_trade::modules::sender::rpc_landing::RpcLanding;
use copy_trade::modules::sender::transaction::TransactionSender;
use copy_trade::modules::stream::geyser_stream::GeyserStream;
use copy_trade::modules::stream::priority_fee_stream::PriorityFeeStream;
use dotenvy::dotenv;
use tokio::try_join;

#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
async fn main() -> Result<()> {
    dotenv().ok();

    Tracing::init();
    Config::init_configs();
    SignerKeypair::init();
    RpcLanding::init();
    RpcHelius::init();
    PositionCache::init()?;

    PriorityFeeStream::stream();

    TransactionSender::heartbeat();

    try_join!(
        GeyserStream::blocks_meta(),
        GeyserStream::transaction_target(),
        GeyserStream::transaction_self(),
    )?;

    Ok(())
}
