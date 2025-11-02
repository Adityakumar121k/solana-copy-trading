use crate::kernel::cache::priority_fee_cache::PriorityFeeCache;
use crate::kernel::clients::http::Http;
use crate::kernel::clients::rpc_helius::RpcHelius;
use serde::Deserialize;
use serde_json::json;
use tokio::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PriorityFeeResponse {
    priority_fee_estimate: f64,
}

pub struct PriorityFeeStream;

impl PriorityFeeStream {
    pub fn stream() {
        tokio::spawn(async move {
            loop {
                if let Ok(response) = RpcHelius::json_rpc::<PriorityFeeResponse>(
                    "",
                    "getPriorityFeeEstimate",
                    Vec::from([json!({
                        "options": {
                            "priorityLevel": "High"
                        }
                    })]),
                )
                .await
                {
                    PriorityFeeCache::set(response.result.priority_fee_estimate as u64);
                };

                sleep(Duration::from_secs(1)).await;
            }
        });
    }
}
