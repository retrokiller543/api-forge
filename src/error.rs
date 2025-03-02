use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error type for API Forge operations.
#[derive(Debug, Error)]
pub enum ApiForgeError {
    /// Error from the reqwest HTTP client.
    #[error("HTTP request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// Error response from the server with status code.
    #[error("Server returned error: {status} - {message}")]
    ResponseError {
        status: StatusCode,
        message: String,
    },

    /// Error parsing the response body (e.g., JSON parsing).
    #[error("Failed to parse response: {0}")]
    ParseError(reqwest::Error),

    /// Error parsing XML content.
    #[error("Failed to parse XML response: {0}")]
    XmlParseError(#[from] serde_xml_rust::Error),

    /// Unsupported or unknown content type.
    #[error("Unsupported content type: {0}")]
    UnsupportedContentType(String),

    /// Required configuration missing (e.g., authentication).
    #[error("Missing required configuration: {0}")]
    ConfigError(String),

    /// Request validation error.
    #[error("Request validation failed: {0}")]
    ValidationError(String),
}

/// Implementation of common methods for ApiForgeError.
impl ApiForgeError {
    /// Creates a new `ResponseError` from a status code and message.
    pub fn response_error(status: StatusCode, message: impl Into<String>) -> Self {
        Self::ResponseError {
            status,
            message: message.into(),
        }
    }

    /// Creates a new `ValidationError` with the given message.
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::ValidationError(msg.into())
    }

    /// Creates a new `ConfigError` with the given message.
    pub fn config(msg: impl Into<String>) -> Self {
        Self::ConfigError(msg.into())
    }

    /// Returns the status code if this error is a response error.
    pub fn status_code(&self) -> Option<StatusCode> {
        match self {
            Self::ResponseError { status, .. } => Some(*status),
            Self::ReqwestError(err) => err.status(),
            _ => None,
        }
    }

    /// Returns true if this error represents a 4xx client error.
    pub fn is_client_error(&self) -> bool {
        self.status_code()
            .map(|status| status.is_client_error())
            .unwrap_or(false)
    }

    /// Returns true if this error represents a 5xx server error.
    pub fn is_server_error(&self) -> bool {
        self.status_code()
            .map(|status| status.is_server_error())
            .unwrap_or(false)
    }
}

/// A structure for standardized error responses from APIs.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    /// The error message.
    pub message: Option<String>,

    /// The error code, if any.
    pub code: Option<String>,

    /// Additional error details.
    pub details: Option<serde_json::Value>,
}

impl From<ErrorResponse> for ApiForgeError {
    fn from(err: ErrorResponse) -> Self {
        ApiForgeError::ResponseError {
            status: StatusCode::BAD_REQUEST,
            message: err.message.unwrap_or_else(|| "Unknown error".to_string()),
        }
    }
}