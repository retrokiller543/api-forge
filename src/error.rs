use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiForgeError {
    #[error("Request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Response error: {0}")]
    ResponseError(reqwest::StatusCode),
    #[error("Parse error: {0}")]
    ParseError(reqwest::Error),
    #[error("XML parse error: {0}")]
    XmlParseError(#[from] serde_xml_rust::Error),
    #[error("Unsupported content type: {0}")]
    UnsupportedContentType(String),
}
