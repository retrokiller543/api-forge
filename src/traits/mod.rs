use crate::error::ApiForgeError;
use reqwest::header::HeaderMap;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use tracing::{debug, info, warn};

/// Enum representing different methods for transmitting data in an HTTP request.
pub enum DataTransmissionMethod {
    QueryParams, // Data sent as query parameters.
    Json,        // Data sent as a JSON body.
    FormData,    // Data sent as URL-encoded form data.
    Multipart,   // Data sent as multipart form data.
}

/// Enum representing different methods for authentication in an HTTP request.
pub enum AuthenticationMethod {
    Bearer, // Bearer token authentication.
    Basic,  // Basic authentication (username and password).
    None,   // No authentication.
}

/// `Request` trait.
///
/// This trait defines a structure for making HTTP requests with custom serialization and response handling.
/// It is intended to be implemented by request types that are serializable and can generate HTTP requests.
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
/// use api_forge::{Request, DataTransmissionMethod, AuthenticationMethod, ApiForgeError};
///
/// #[derive(Serialize, Debug)]
/// struct MyRequest {
///     field1: String,
///     field2: i32,
/// }
///
/// #[derive(Deserialize, Debug, Default)]
/// struct MyResponse {
///     result: String,
/// }
///
/// impl From<reqwest::Response> for MyResponse {
///     fn from(resp: reqwest::Response) -> Self {
///         // Convert the response into your response structure
///         resp.json().unwrap_or_else(|_| MyResponse {
///             result: "Error parsing response".into(),
///         })
///     }
/// }
///
/// impl Request<MyResponse> for MyRequest {
///     const ENDPOINT: &'static str = "/api/my_endpoint";
///     const METHOD: Method = Method::POST; // Override HTTP method if necessary
///     const DATA_TRANSMISSION_METHOD: DataTransmissionMethod = DataTransmissionMethod::Json; // Send data as JSON
///     const AUTHENTICATION_METHOD: AuthenticationMethod = AuthenticationMethod::Bearer; // Use Bearer authentication
///     async fn from_response(resp: reqwest::Response) -> Result<Self::Response, ApiForgeError> where <Self as Request<MyResponse>>::Response: From<reqwest::Response> {
///         resp.json().await
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let request = MyRequest {
///         field1: "Test".to_string(),
///         field2: 42,
///     };
///
///     let headers = HeaderMap::new();
///     let token = Some(("my_token".to_string(), None));
///
///     match request.send_and_parse("https://api.example.com", Some(headers), token).await {
///         Ok(response) => println!("Success: {:?}", response),
///         Err(e) => eprintln!("Request failed: {:?}", e),
///     }
/// }
/// ```
#[allow(async_fn_in_trait)]
pub trait Request<Res>
where
    Self: Serialize + Debug,
{
    /// The type of the response, which must implement `From<reqwest::Response>`.
    type Response = Res;

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

    async fn from_response(resp: reqwest::Response) -> Result<Self::Response, ApiForgeError>;

    /// Optional: Provides multipart form data for file uploads.
    fn multipart_form_data(&self) -> reqwest::multipart::Form {
        debug!("Implement multipart_form_data() if needed, or leave empty.");
        reqwest::multipart::Form::new()
    }

    /// Generates a `reqwest::RequestBuilder` based on the request's parameters, including optional headers and authentication.
    fn generate_request(
        &self,
        base_url: &str,
        headers: Option<HeaderMap>,
        token: Option<(String, Option<String>)>,
    ) -> reqwest::RequestBuilder {
        let url = format!("{}{}", base_url, Self::ENDPOINT);
        let client = reqwest::Client::new();

        // Match the HTTP method
        let builder = match Self::METHOD {
            reqwest::Method::GET => client.get(&url),
            reqwest::Method::POST => client.post(&url),
            reqwest::Method::PUT => client.put(&url),
            reqwest::Method::DELETE => client.delete(&url),
            reqwest::Method::PATCH => client.patch(&url),
            reqwest::Method::HEAD => client.head(&url),
            _ => client.get(&url),
        };

        // Add data based on the transmission method
        let mut request = match Self::DATA_TRANSMISSION_METHOD {
            DataTransmissionMethod::QueryParams => builder.query(self),
            DataTransmissionMethod::Json => builder.json(self),
            DataTransmissionMethod::FormData => builder.form(self),
            DataTransmissionMethod::Multipart => builder.multipart(self.multipart_form_data()),
        };

        // Add authentication if applicable
        if let Some((token, password)) = token {
            match Self::AUTHENTICATION_METHOD {
                AuthenticationMethod::Basic => request = request.basic_auth(token, password),
                AuthenticationMethod::Bearer => request = request.bearer_auth(token),
                AuthenticationMethod::None => warn!("No authentication required for this request."),
            }
        }

        // Add headers if provided
        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        debug!("Generated request: {:#?}", request);
        request
    }

    /// Sends the request asynchronously and returns the result.
    async fn send_request(
        &self,
        base_url: &str,
        headers: Option<HeaderMap>,
        token: Option<(String, Option<String>)>,
    ) -> reqwest::Result<reqwest::Response> {
        info!("Sending request to {}{}...", base_url, Self::ENDPOINT);
        debug!("Request body: {:#?}", self);
        self.generate_request(base_url, headers, token).send().await
    }

    /// Sends the request and attempts to parse the response.
    /// Returns a `Result` containing the parsed response or an error.
    async fn send_and_parse(
        &self,
        base_url: &str,
        headers: Option<HeaderMap>,
        token: Option<(String, Option<String>)>,
    ) -> Result<Self::Response, ApiForgeError> {
        let response = self.send_request(base_url, headers, token).await?;

        if response.error_for_status_ref().is_err() {
            Err(ApiForgeError::ResponseError(response.status()))
        } else {
            Ok(Self::from_response(response).await?)
        }
    }
}