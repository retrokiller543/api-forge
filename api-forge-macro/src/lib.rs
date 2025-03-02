use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr, Error as SynError};

#[derive(Debug, FromDeriveInput, Clone)]
#[darling(attributes(request))]
struct RequestArgs {
    data: darling::ast::Data<(), HeaderField>,
    endpoint: String,
    #[darling(default, rename = "response_type")]
    response_type: Option<String>,
    #[darling(default, rename = "method")]
    method: Option<syn::Path>,
    #[darling(default, rename = "transmission")]
    transmission: Option<syn::Path>,
    #[darling(default, rename = "authentication")]
    authentication: Option<syn::Path>,
    #[darling(default, rename = "path_parameters")]
    path_parameters: Option<Vec<LitStr>>,
    #[darling(default)]
    accept: Option<LitStr>,
    #[darling(default)]
    content_type: Option<LitStr>,
}

#[derive(Debug, FromField, Clone)]
#[darling(attributes(request))]
struct HeaderField {
    ident: Option<Ident>,
    #[darling(default)]
    header_name: Option<LitStr>,
}

/// # Request Derive Macro
///
/// Automatically implements the `ApiRequest` trait for structs, making it easy to define
/// type-safe API requests with minimal boilerplate.
///
/// This macro analyzes your struct and its attributes to generate an implementation of 
/// `api_forge::ApiRequest<T>`, which handles creating HTTP requests with appropriate 
/// configuration for endpoints, methods, headers, and authentication.
///
/// ## Basic Usage
///
/// ```no_compile
/// #[derive(Debug, Serialize, Request)]
/// #[request(endpoint = "/api/users")]
/// struct GetUsersRequest {
///     page: Option<u32>,
///     limit: Option<u32>,
/// }
/// ```
///
/// ## Struct-Level Attributes
///
/// ### Required Attributes
///
/// - `endpoint` (required): The API endpoint path to call (e.g., `/api/users`).
///   Path parameters can be specified with curly braces (e.g., `/api/users/{id}`).
///
/// ### Optional Attributes
///
/// - `response_type` (optional): The type to deserialize the API response into.
///   Defaults to `EmptyResponse` which is interpreted as `()`.
///   Example: `#[request(endpoint = "/api/users", response_type = "Vec<User>")]`
///
/// - `method` (optional): The HTTP method to use. Defaults to `GET`.
///   Supported values: `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, `HEAD`.
///   Example: `#[request(endpoint = "/api/users", method = "POST")]`
///
/// - `transmission` (optional): How to transmit the request data. Defaults to `QueryParams`.
///   Supported values:
///   - `QueryParams`: Sends data as URL query parameters
///   - `Json`: Sends data as a JSON body
///   - `FormData`: Sends data as application/x-www-form-urlencoded
///   - `Multipart`: Sends data as multipart/form-data (requires implementing `multipart_form_data()`)
///   Example: `#[request(endpoint = "/api/users", transmission = "Json")]`
///
/// - `authentication` (optional): The authentication method to use. Defaults to `None`.
///   Supported values:
///   - `None`: No authentication
///   - `Basic`: HTTP Basic authentication
///   - `Bearer`: Bearer token authentication
///   Example: `#[request(endpoint = "/api/users", authentication = "Bearer")]`
///
/// - `path_parameters` (optional): A list of field names that should be used to replace
///   placeholders in the endpoint path.
///   Example: `#[request(endpoint = "/api/users/{id}/posts/{post_id}", path_parameters = ["id", "post_id"])]`
///
/// - `accept` (optional): The value for the `Accept` header. Defaults to `application/json`.
///   Example: `#[request(endpoint = "/api/users", accept = "application/xml")]`
///
/// - `content_type` (optional): The value for the `Content-Type` header.
///   If not specified, no Content-Type header is explicitly added (frameworks like reqwest
///   may add one based on the transmission method).
///   Example: `#[request(endpoint = "/api/users", content_type = "application/vnd.api+json")]`
///
/// ## Field-Level Attributes
///
/// - `header_name`: Marks a field to be sent as an HTTP header instead of as part of the request data.
///   Example: `#[request(header_name = "X-Api-Key")]`
///
/// ## Complete Examples
///
/// ### Simple GET Request
///
/// ```no_compile
/// #[derive(Debug, Serialize, Request)]
/// #[request(
///     endpoint = "/api/users",
///     response_type = "Vec<User>"
/// )]
/// struct ListUsersRequest {
///     page: Option<u32>,
///     per_page: Option<u32>,
/// }
/// ```
///
/// ### POST Request with JSON Body
///
/// ```no_compile
/// #[derive(Debug, Serialize, Request)]
/// #[request(
///     endpoint = "/api/users",
///     method = "POST",
///     transmission = "Json",
///     response_type = "User"
/// )]
/// struct CreateUserRequest {
///     name: String,
///     email: String,
///     role: String,
/// }
/// ```
///
/// ### Request with Path Parameters
///
/// ```no_compile
/// #[derive(Debug, Serialize, Request)]
/// #[request(
///     endpoint = "/api/users/{user_id}/posts/{post_id}",
///     path_parameters = ["user_id", "post_id"],
///     response_type = "Post"
/// )]
/// struct GetUserPostRequest {
///     user_id: String,
///     post_id: String,
///     include_comments: Option<bool>,
/// }
/// ```
///
/// ### Request with Custom Headers
///
/// ```no_compile
/// #[derive(Debug, Serialize, Request)]
/// #[request(endpoint = "/api/protected/resources")]
/// struct ProtectedRequest {
///     // This field becomes a header, not a query parameter
///     #[request(header_name = "X-Api-Key")]
///     api_key: String,
///     
///     // This is an optional header that will only be included if Some
///     #[request(header_name = "X-Request-ID")]
///     request_id: Option<String>,
///     
///     // Regular field - will be sent as a query parameter
///     filter: Option<String>,
/// }
/// ```
///
/// ### Multipart File Upload
///
/// ```no_compile
/// #[derive(Debug, Serialize, Request)]
/// #[request(
///     endpoint = "/api/upload",
///     method = "POST",
///     transmission = "Multipart"
/// )]
/// struct UploadFileRequest {
///     name: String,
///     description: Option<String>,
/// }
///
/// // When using Multipart, you must implement this method
/// impl UploadFileRequest {
///     fn multipart_form_data(&self) -> reqwest::multipart::Form {
///         let file_bytes = std::fs::read("path/to/file.jpg").unwrap();
///         let part = reqwest::multipart::Part::bytes(file_bytes)
///             .file_name("file.jpg");
///         
///         reqwest::multipart::Form::new()
///             .text("name", self.name.clone())
///             .part("file", part)
///     }
/// }
/// ```
///
/// ## Generated Implementation
///
/// The macro generates an implementation of the `api_forge::ApiRequest<T>` trait, where T is the
/// specified response type. This implementation:
///
/// 1. Builds the complete URL by combining the base URL with the endpoint and replacing any path parameters
/// 2. Creates an HTTP request with the specified method
/// 3. Applies the data transmission method (query params, JSON, form data, or multipart)
/// 4. Adds authentication if provided
/// 5. Sets all specified headers
/// 6. Logs request details for debugging
///
/// ## How Path Parameters Work
///
/// If your endpoint contains parameters in curly braces like `/users/{id}`, you must:
/// 1. List the parameter names in the `path_parameters` attribute
/// 2. Have fields in your struct with matching names
/// 3. The values of these fields will replace the placeholders in the URL
///
/// ## Type Requirements
///
/// - Your struct must be serializable (implement `Serialize` from serde)
/// - For `Multipart` transmission, your struct must implement a `multipart_form_data()` method
/// - Response types must be deserializable (implement `Deserialize` from serde)
#[proc_macro_derive(Request, attributes(request))]
pub fn derive_request(input: TokenStream) -> TokenStream {
    // Parse the input into a DeriveInput struct using syn
    let input = parse_macro_input!(input as DeriveInput);

    // Use `darling` to parse the attributes from the input
    let args = match RequestArgs::from_derive_input(&input) {
        Ok(args) => args,
        Err(e) => {
            let error = e.write_errors();
            return SynError::new(Span::call_site(), format!("{}", error))
                .to_compile_error()
                .into();
        }
    };

    let name = &input.ident;
    let data = args.data.clone();
    let mut header_inserts = vec![];

    data.map_struct_fields(|field| {
        if let Some(header_name) = &field.header_name {
            if let Some(header_field_ident) = &field.ident {
                let header_name_value = header_name.value();

                let header_insert = quote! {
                    if let Some(value) = self.#header_field_ident.as_ref() {
                        all_headers.insert(#header_name_value, reqwest::header::HeaderValue::from_str(&value.to_string()).unwrap_or_default());
                    }
                };

                header_inserts.push(header_insert);
            }
        }
    });

    let endpoint = args.endpoint;
    // Create a path for EmptyResponse
    let response_type = match &args.response_type {
        Some(type_str) => {
            // Parse the type string into a syn::Type
            match syn::parse_str::<syn::Type>(type_str) {
                Ok(parsed_type) => parsed_type,
                Err(err) => {
                    return SynError::new(
                        Span::call_site(),
                        format!("Failed to parse response_type '{}': {}", type_str, err)
                    )
                        .to_compile_error()
                        .into();
                }
            }
        }
        None => {
            syn::parse_str::<syn::Type>("EmptyResponse").unwrap()
        }
    };
    let method = args.method.clone().unwrap_or_else(|| syn::parse_quote!(GET));
    let transmission_method = args
        .transmission
        .clone()
        .unwrap_or_else(|| syn::parse_quote!(QueryParams));
    let authentication_method = args
        .authentication
        .clone()
        .unwrap_or_else(|| syn::parse_quote!(None));

    // Handle path parameters more robustly
    let path_parameters = args.path_parameters.unwrap_or_default();
    let path_parameters_str = path_parameters
        .iter()
        .map(|p| p.value())
        .collect::<Vec<_>>();
    let path_parameters_idents = path_parameters_str
        .iter()
        .map(|p| Ident::new(p, Span::call_site()))
        .collect::<Vec<_>>();

    // Content type headers
    let accept = args.accept.unwrap_or_else(||
        LitStr::new("application/json", Span::call_site())
    );

    let content_type = args.content_type.map(|ct| {
        quote! {
            builder = builder.header(reqwest::header::CONTENT_TYPE, #ct);
        }
    });

    // Define empty_type as a path to EmptyResponse for comparison
    let empty_type = "EmptyResponse".to_string();

    // If response_type is EmptyResponse, use (), otherwise use the specified type
    let res_type = if std::format!("{}", quote!(#response_type)) == std::format!("{}", quote!(#empty_type)) {
        quote!(())
    } else {
        quote!(#response_type)
    };

    // Generate a more efficient method match
    let method_match = quote! {
        let builder = match Self::METHOD {
            reqwest::Method::GET => client.get(&url),
            reqwest::Method::POST => client.post(&url),
            reqwest::Method::PUT => client.put(&url),
            reqwest::Method::DELETE => client.delete(&url),
            reqwest::Method::PATCH => client.patch(&url),
            reqwest::Method::HEAD => client.head(&url),
            other => {
                tracing::warn!("Using non-standard HTTP method: {:?}", other);
                client.request(other, &url)
            },
        };
    };

    // More efficient path parameter replacement
    let path_param_replacements = if !path_parameters_str.is_empty() {
        quote! {
            let mut url = format!("{}{}", base_url, Self::ENDPOINT);
            #(
                url = url.replace(&format!("{{{}}}", #path_parameters_str), &self.#path_parameters_idents.to_string());
            )*
        }
    } else {
        quote! {
            let url = format!("{}{}", base_url, Self::ENDPOINT);
        }
    };

    // Extract the input's generics to reuse them in the impl
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Create a new where clause that includes our trait bounds
    let where_clause = where_clause.cloned().unwrap_or_else(|| syn::WhereClause {
        where_token: syn::token::Where::default(),
        predicates: syn::punctuated::Punctuated::new(),
    });

    // Generate the final code for the derive macro
    let expanded = quote! {
        impl #impl_generics api_forge::ApiRequest<#res_type> for #name #ty_generics #where_clause {
            const ENDPOINT: &'static str = #endpoint;
            const METHOD: reqwest::Method = reqwest::Method::#method;
            const DATA_TRANSMISSION_METHOD: api_forge::DataTransmissionMethod = api_forge::DataTransmissionMethod::#transmission_method;
            const AUTHENTICATION_METHOD: api_forge::AuthenticationMethod = api_forge::AuthenticationMethod::#authentication_method;

            fn generate_request(
                &self,
                base_url: &str,
                headers: Option<reqwest::header::HeaderMap>,
                token: Option<(String, Option<String>)>,
            ) -> reqwest::RequestBuilder {
                #path_param_replacements

                let client = reqwest::Client::new();
                #method_match

                // Apply data transmission method
                let mut builder = match Self::DATA_TRANSMISSION_METHOD {
                    api_forge::DataTransmissionMethod::QueryParams => builder.query(self),
                    api_forge::DataTransmissionMethod::Json => builder.json(self),
                    api_forge::DataTransmissionMethod::FormData => builder.form(self),
                    api_forge::DataTransmissionMethod::Multipart => builder.multipart(self.multipart_form_data()),
                };

                // Apply authentication if provided
                if let Some((token, password)) = token {
                    builder = match Self::AUTHENTICATION_METHOD {
                        api_forge::AuthenticationMethod::Basic => builder.basic_auth(token, password),
                        api_forge::AuthenticationMethod::Bearer => builder.bearer_auth(token),
                        api_forge::AuthenticationMethod::None => builder,
                    };
                }

                // Create and populate headers
                let mut all_headers = reqwest::header::HeaderMap::new();

                // Add custom headers from fields
                #(#header_inserts)*

                // Apply provided headers
                if let Some(headers) = headers {
                    all_headers.extend(headers);
                }

                // Set default headers if not already set
                if !all_headers.contains_key(reqwest::header::ACCEPT) {
                    all_headers.insert(
                        reqwest::header::ACCEPT,
                        reqwest::header::HeaderValue::from_static(#accept)
                    );
                }

                #content_type

                builder = builder.headers(all_headers);

                tracing::debug!("Generated request: {:?}", builder);
                builder
            }
        }
    };

    TokenStream::from(expanded)
}