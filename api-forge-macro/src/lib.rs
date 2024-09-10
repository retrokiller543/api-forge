use darling::FromDeriveInput;
use proc_macro2::Ident;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(request))]
struct RequestArgs {
    endpoint: String,

    #[darling(rename = "response_type")]
    response_type: Ident,

    #[darling(default, rename = "method")]
    method: Option<Ident>,

    #[darling(default, rename = "transmission")]
    transmission: Option<Ident>,

    #[darling(default, rename = "authentication")]
    authentication: Option<Ident>,

    /*#[darling(default, rename = "headers")]
    headers: Option<Vec<(String, String)>>,*/
}

#[proc_macro_derive(Request, attributes(request))]
pub fn derive_request(input: TokenStream) -> TokenStream {
    // Parse the input into a DeriveInput struct using syn
    let input = parse_macro_input!(input as DeriveInput);

    // Use `darling` to parse the attributes from the input
    let args = RequestArgs::from_derive_input(&input).unwrap_or_else(|e| {
        panic!("Error parsing attributes: {:?}", e);
    });

    let name = &input.ident;

    // `endpoint` is now required, so no need to check if it exists
    let endpoint = args.endpoint;
    let response_type = args.response_type;
    let method = args.method.unwrap_or_else(|| Ident::new("GET", proc_macro2::Span::call_site()));
    let transmission_method = args.transmission.unwrap_or_else(|| Ident::new("QueryParams", proc_macro2::Span::call_site()));
    let authentication_method = args.authentication.unwrap_or_else(|| Ident::new("None", proc_macro2::Span::call_site()));
    /*let default_headers = args.headers.unwrap_or_else(|| vec![]);*/

    // The macro will generate the `Request` trait implementation for the struct
    let expanded = quote! {
        impl api_forge::ApiRequest<#response_type> for #name {
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
                let url = format!("{}{}", base_url, Self::ENDPOINT);
                let client = reqwest::Client::new();

                let mut builder = match Self::METHOD {
                    reqwest::Method::GET => client.get(&url),
                    reqwest::Method::POST => client.post(&url),
                    reqwest::Method::PUT => client.put(&url),
                    _ => client.get(&url),
                };

                builder = match Self::DATA_TRANSMISSION_METHOD {
                    api_forge::DataTransmissionMethod::QueryParams => builder.query(self),
                    api_forge::DataTransmissionMethod::Json => builder.json(self),
                    _ => builder.form(self),
                };

                if let Some((token, password)) = token {
                    builder = match Self::AUTHENTICATION_METHOD {
                        api_forge::AuthenticationMethod::Basic => builder.basic_auth(token, password),
                        api_forge::AuthenticationMethod::Bearer => builder.bearer_auth(token),
                        api_forge::AuthenticationMethod::None => builder,
                    };
                }

                let mut all_headers = reqwest::header::HeaderMap::new();
                // Add default headers from attributes
                /*#(
                    all_headers.insert(
                        reqwest::header::HeaderName::from_static(#default_headers),
                        reqwest::header::HeaderValue::from_static("default_value")  // You can expand this for real values
                    );
                )**/

                if let Some(headers) = headers {
                    all_headers.extend(headers);
                }

                builder.headers(all_headers)
            }
        }
    };

    TokenStream::from(expanded)
}