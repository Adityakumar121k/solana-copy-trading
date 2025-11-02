use crate::modules::sender::types::{ErrorResponse, RpcResponse, SuccessResponse};
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap};
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use std::time::Duration;

pub struct HttpCfg {
    pub client: Client,
    pub provider_url: String,
    pub api_key: String,
}

pub trait Http {
    #[allow(clippy::new_ret_no_self)]
    fn new(api_key: String, provider_url: String) -> HttpCfg {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .tcp_keepalive(Some(Duration::from_secs(30)))
            .pool_idle_timeout(Duration::from_secs(90))
            .build()
            .expect("Failed to build reqwest client");

        HttpCfg { client, provider_url, api_key }
    }

    fn init();

    fn get_cfg() -> &'static HttpCfg;

    // pub async fn get<T>(
    //     path: &str,
    //     query: Option<HashMap<String, String>>,
    // ) -> Result<SuccessResponse<T>, ErrorResponse>
    // where
    //     T: DeserializeOwned + std::fmt::Debug,
    // {
    //     let url = format!("{}{}", Self::get_self().provider_url, path);
    //
    //     let mut req = Self::get_self()
    //         .client
    //         .get(&url)
    //         .query(&[("clients-key", Self::get_self().api_key.as_str())]);
    //
    //     if let Some(query) = query {
    //         req = req.query(&query);
    //     }
    //
    //     let resp = req.send().await
    //         .map_err(|e| ErrorResponse::from_other(format!("send failed: {e}")))?;
    //
    //     Self::parse_response(resp).await
    // }

    #[allow(async_fn_in_trait)]
    async fn json_rpc<T>(
        path: &str,
        method: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<SuccessResponse<T>, ErrorResponse>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let url = format!("{}{}", Self::get_cfg().provider_url, path);

        let req_body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": method,
            "params": params,
        });

        let resp = Self::get_cfg()
            .client
            .post(&url)
            .query(&[("api-key", Self::get_cfg().api_key.as_str())])
            .json(&req_body)
            .send()
            .await
            .map_err(|e| ErrorResponse::from_other(format!("send failed: {e}")))?;

        Self::parse_response(resp).await
    }

    // pub async fn get_text(
    //     path: &str,
    //     query: Option<HashMap<String, String>>,
    // ) -> Result<String, ErrorResponse> {
    //     let url = format!("{}{}", Self::get_cfg().provider_url, path);
    //
    //     let mut req = Self::get_cfg()
    //         .client
    //         .get(&url)
    //         .query(&[("clients-key", Self::get_cfg().api_key.as_str())]);
    //
    //     if let Some(query) = query {
    //         req = req.query(&query);
    //     }
    //
    //     let response = req.send().await
    //         .map_err(|e| ErrorResponse::from_other(format!("send failed: {e}")))?;
    //
    //     let status = response.status();
    //     let text = response.text().await
    //         .map_err(|e| ErrorResponse::from_other(format!("read body failed: {e}")))?;
    //
    //     if !status.is_success() {
    //         return Err(ErrorResponse::from_status(status, text));
    //     }
    //
    //     Ok(text)
    // }

    #[allow(async_fn_in_trait)]
    async fn parse_response<T>(response: Response) -> Result<SuccessResponse<T>, ErrorResponse>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let status = response.status();

        let ct: String = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_owned())
            .unwrap_or_default();

        let body_text = response
            .text()
            .await
            .map_err(|e| ErrorResponse::from_other(format!("read body failed: {e}")))?;

        if !status.is_success() {
            return Err(ErrorResponse::from_status(status, body_text));
        }

        if body_text.trim().is_empty() {
            return Err(ErrorResponse::from_status(
                status,
                "<empty body>".to_string(),
            ));
        }

        let json_str = if ct.contains("application/json")
            || body_text.trim_start().starts_with(['{', '[', '"'])
        {
            body_text.clone()
        } else {
            serde_json::to_string(&body_text).unwrap()
        };

        if let Ok(response) = serde_json::from_str::<RpcResponse<T>>(&json_str) {
            return match response {
                RpcResponse::Success(s) => Ok(s),
                RpcResponse::Error(e) => Err(ErrorResponse {
                    code: e.error.code,
                    message: e.error.message,
                    data: e.error.data,
                }),
            };
        }

        match serde_json::from_str::<T>(&json_str) {
            Ok(val) => Ok(SuccessResponse { result: val }),
            Err(e) => Err(ErrorResponse::from_other(format!(
                "deserialize failed: {e};\nbody={body_text}"
            ))),
        }
    }
}
