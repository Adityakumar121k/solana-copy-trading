use crate::kernel::clients::http::{Http, HttpCfg};
use std::env;
use std::sync::LazyLock;

pub struct RpcLanding;

static CLIENT: LazyLock<HttpCfg> = LazyLock::new(|| {
    RpcLanding::new(
        env::var("ZEROSLOT_API_KEY").expect("ZEROSLOT_API_KEY not set"),
        env::var("ZEROSLOT_URL").expect("ZEROSLOT_URL not set"),
    )
});

impl Http for RpcLanding {
    fn init() {
        let _ = &CLIENT;
        tracing::info!("Connected landing RPC");
    }

    fn get_cfg() -> &'static HttpCfg {
        &CLIENT
    }
}
