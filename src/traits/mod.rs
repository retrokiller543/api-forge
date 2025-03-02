use crate::error::ApiForgeError;
use reqwest::header::{HeaderMap};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use tracing::{debug, error, info};
use crate::ApiResult;

/// Enum representing different methods for transmitting data in an HTTP request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataTransmissionMethod {
    /// Data sent as query parameters in the URL.
    QueryParams,
    /// Data sent as a JSON body.
    Json,
    /// Data sent as URL-encoded form data.
    FormData,
    /// Data sent as multipart form data (for file uploads).
    Multipart,
}

/// Enum representing different methods for authentication in an HTTP request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthenticationMethod {
    /// Bearer token authentication.
    Bearer,
    /// Basic authentication (username and password).
    Basic,
    /// No authentication.
    None,
}

/// `ApiRequest` trait.
///
/// This trait defines a structure for making HTTP requests with custom serialization and response handling.
/// It is intended to be implemented by request types that are serializable and can generate HTTP requests.
///
/// # Type Parameters
///
/// - `Res`: The response type that will be deserialized from the API response. Defaults to `()`.
///
/// # Requirements
///
/// Implementing types must:
/// - Implement the `Serialize` trait from `serde` for serializing the request data.
/// - Implement the `Debug` trait for debugging purposes.
/// - Define a constant `ENDPOINT` representing the API endpoint.
///
/// # Associated Constants
///
/// - `ENDPOINT`: A static string representing the endpoint for the request.
/// - `METHOD`: The HTTP method (default is `GET`).
/// - `DATA_TRANSMISSION_METHOD`: Specifies how the request data is sent (default is `QueryParams`).
/// - `AUTHENTICATION_METHOD`: Specifies the authentication method (default is `None`).
///
/// # Methods
///
/// - `generate_request`: Generates a `reqwest::RequestBuilder` based on the request type.
/// - `send_request`: Sends the request asynchronously and returns the response.
/// - `send_and_parse`: Sends the request and parses the response, returning a result or an error.
///
/// # Example
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use reqwest::header::HeaderMap;
/// use reqwest::Method;
/// use api_forge::{ApiRequest, Request};
///
/// #[derive(Serialize, Debug, Request)]
/// #[request(endpoint = "/my_endpoint", method = "POST", transmission = Json, response_type = "MyResponse")]
/// struct MyRequest {
///     field1: String,
///     field2: i32,
///
///     #[request(header_name = "X-Custom-Header")]
///     custom_header: Option<String>,
/// }
///
/// #[derive(Deserialize, Debug, Default)]
/// struct MyResponse {
///     result: String,
/// }
///
/// // Example usage
/// async fn example() -> Result<(), Box<dyn std::error::Error>> {
///     let request = MyRequest {
///         field1: "value".to_string(),
///         field2: 42,
///         custom_header: Some("header-value".to_string()),
///     };
///
///     let response: MyResponse = request
///         .send_and_parse("https://api.example.com", None, None)
///         .await?;
///
///     println!("Response: {:?}", response);
///     Ok(())
/// }
/// ```
#[cfg_attr(feature = "native", allow(async_fn_in_trait))]
pub trait ApiRequest<Res = ()>
where
    Self: Serialize + Debug,
    Res: Default + DeserializeOwned,
{
    /// A static string representing the endpoint for the request.
    const ENDPOINT: &'static str;

    /// Determines the HTTP method for the request. Defaults to `GET`.
    const METHOD: reqwest::Method = reqwest::Method::GET;

    /// Specifies how the data will be transmitted in the request.
    /// The default is `DataTransmissionMethod::QueryParams`.
    const DATA_TRANSMISSION_METHOD: DataTransmissionMethod = DataTransmissionMethod::QueryParams;

    /// Specifies the method of authentication for the request.
    /// The default is `AuthenticationMethod::None`.
    const AUTHENTICATION_METHOD: AuthenticationMethod = AuthenticationMethod::None;

    /// Parses a response into the expected type.
    /// This method handles different content types and formats.
    async fn from_response(resp: reqwest::Response) -> ApiResult<Res> {
        debug!("Received response: {:?}", resp);
        let status = resp.status();

        // Check if the response is successful
        if !status.is_success() {
            return Err(ApiForgeError::ResponseError {
                status,
                message: resp.text().await.unwrap_or_default(),
            });
        }
        
        if status == reqwest::StatusCode::NO_CONTENT {
            debug!("Status is 204 No Content, returning default value.");
            return Ok(Res::default());
        }

        // Check for empty body or 204 No Content status
        if resp.content_length().unwrap_or(0) == 0 {
            debug!("Response is empty, returning default value.");
            return Ok(Res::default());
        }

        // Get content type to determine parsing strategy
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .map(|v| v.to_str().unwrap_or("").to_lowercase())
            .unwrap_or_default();

        debug!("Content-Type: {}", content_type);

        // Parse based on content type
        if content_type.contains("application/json") {
            debug!("Parsing response as JSON");
            resp.json::<Res>().await.map_err(|e| {
                error!("JSON parsing error: {}", e);
                ApiForgeError::ParseError(e)
            })
        } else if content_type.contains("application/xml") || content_type.contains("text/xml") {
            debug!("Parsing response as XML");
            let text = resp.text().await.map_err(|e| {
                error!("Failed to get text from response: {}", e);
                ApiForgeError::ParseError(e)
            })?;

            serde_xml_rust::from_str::<Res>(&text).map_err(|e| {
                error!("XML parsing error: {}", e);
                ApiForgeError::XmlParseError(e)
            })
        } else if content_type.contains("text/plain") {
            // For text/plain, we just try JSON as a fallback
            debug!("Content-Type is text/plain, attempting JSON parsing as fallback");
            resp.json::<Res>().await.map_err(|e| {
                error!("Failed to parse text/plain as JSON: {}", e);
                ApiForgeError::ParseError(e)
            })
        } else {
            // Try json as a last resort
            debug!("Unknown content type: {}, trying JSON as fallback", content_type);
            resp.json::<Res>().await.map_err(|e| {
                error!("Failed to parse unknown content type: {}", e);
                ApiForgeError::UnsupportedContentType(content_type)
            })
        }
    }

    /// Optional: Provides multipart form data for file uploads.
    /// Override this method when using `DataTransmissionMethod::Multipart`.
    fn multipart_form_data(&self) -> reqwest::multipart::Form {
        debug!("Using default empty multipart form. Override multipart_form_data() if needed.");
        reqwest::multipart::Form::new()
    }

    /// Generates a `reqwest::RequestBuilder` based on the request's parameters.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the API.
    /// * `headers` - Optional headers to include in the request.
    /// * `token` - Optional authentication token (and password for Basic auth).
    ///
    /// # Returns
    ///
    /// A configured `reqwest::RequestBuilder` ready to be sent.
    fn generate_request(
        &self,
        base_url: &str,
        headers: Option<HeaderMap>,
        token: Option<(String, Option<String>)>,
    ) -> reqwest::RequestBuilder;

    /// Sends the request asynchronously and returns the raw response.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the API.
    /// * `headers` - Optional headers to include in the request.
    /// * `token` - Optional authentication token (and password for Basic auth).
    ///
    /// # Returns
    ///
    /// A `Result` containing the raw response or a reqwest error.
    async fn send_request(
        &self,
        base_url: &str,
        headers: Option<HeaderMap>,
        token: Option<(String, Option<String>)>,
    ) -> reqwest::Result<reqwest::Response> {
        info!("Sending request to {}{}...", base_url, Self::ENDPOINT);
        debug!("Request: {:?}", self);
        self.generate_request(base_url, headers, token).send().await
    }

    /// Sends the request and parses the response into the expected type.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the API.
    /// * `headers` - Optional headers to include in the request.
    /// * `token` - Optional authentication token (and password for Basic auth).
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed response or an error.
    async fn send_and_parse(
        &self,
        base_url: &str,
        headers: Option<HeaderMap>,
        token: Option<(String, Option<String>)>,
    ) -> ApiResult<Res> {
        let response = self.send_request(base_url, headers, token).await?;
        debug!("Response status: {}", response.status());
        Self::from_response(response).await
    }
}