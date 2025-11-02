use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RpcResponse<T> {
    Success(SuccessResponse<T>),
    Error(ErrorEnvelope),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuccessResponse<T> {
    pub result: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorEnvelope {
    pub error: ErrorData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorData {
    pub code: i32,
    pub message: String,
    pub data: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Error)]
#[error("code: {code}\nmessage: {message}\ndata: {data:?}")]
pub struct ErrorResponse {
    pub code: i32,
    pub message: String,
    pub data: Option<String>,
}

impl ErrorResponse {
    pub fn from_status(status: StatusCode, body: String) -> Self {
        Self {
            code: status.as_u16() as i32,
            message: format!(
                "HTTP {} {}",
                status.as_u16(),
                status.canonical_reason().unwrap_or("")
            ),
            data: if body.is_empty() { None } else { Some(body) },
        }
    }

    pub fn from_other<M: Into<String>>(msg: M) -> Self {
        Self { code: -1, message: msg.into(), data: None }
    }
}
