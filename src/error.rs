use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiForgeError {
    #[error("Request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Response error: {0}")]
    ResponseError(reqwest::StatusCode),
}
