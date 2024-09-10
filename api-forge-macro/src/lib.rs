use darling::{FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};

#[derive(Debug, FromDeriveInput, Clone)]
#[darling(attributes(request))]
struct RequestArgs {
    data: darling::ast::Data<(), HeaderField>,

    endpoint: String,

    #[darling(default, rename = "response_type")]
    response_type: Option<Ident>,

    #[darling(default, rename = "method")]
    method: Option<Ident>,

    #[darling(default, rename = "transmission")]
    transmission: Option<Ident>,

    #[darling(default, rename = "authentication")]
    authentication: Option<Ident>,

    #[darling(default, rename = "path_parameters")]
    path_parameters: Option<Vec<LitStr>>,
}

#[derive(Debug, FromField, Clone)]
#[darling(attributes(request))]
struct HeaderField {
    ident: Option<Ident>,

    // This will capture the name of the header
    #[darling(default)]
    header_name: Option<LitStr>,
}

#[proc_macro_derive(Request, attributes(request))]
pub fn derive_request(input: TokenStream) -> TokenStream {
    // Parse the input into a DeriveInput struct using syn
    let input = parse_macro_input!(input as DeriveInput);

    // Use `darling` to parse the attributes from the input
    let args = RequestArgs::from_derive_input(&input).unwrap_or_else(|e| {
        let error = e.write_errors();

        panic!("{}", error);
    });

    let name = &input.ident;
    let data = args.data.clone();
    let mut header_inserts = vec![];

    data.map_struct_fields(|field| {
        if field.header_name.is_some() {
            // Add the #[serde(skip)] attribute to the header fields
            let header_field_ident = field.ident.as_ref().unwrap();
            let header_name = field.header_name.as_ref().unwrap().value();

            let header_insert = quote! {
                if let Some(value) = self.#header_field_ident.as_ref() {
                    builder = builder.header(#header_name, value);
                }
            };

            header_inserts.push(header_insert);
        }
    });

    let endpoint = args.endpoint;
    let response_type = args
        .response_type
        .unwrap_or_else(|| Ident::new("EmptyResponse", proc_macro2::Span::call_site()));
    let method = args
        .method
        .unwrap_or_else(|| Ident::new("GET", proc_macro2::Span::call_site()));
    let transmission_method = args
        .transmission
        .unwrap_or_else(|| Ident::new("QueryParams", proc_macro2::Span::call_site()));
    let authentication_method = args
        .authentication
        .unwrap_or_else(|| Ident::new("None", proc_macro2::Span::call_site()));
    let path_parameters = args.path_parameters.unwrap_or(Vec::new());
    let path_parameters = path_parameters
        .iter()
        .map(|p| p.value())
        .collect::<Vec<_>>();
    let path_parameters_idents = path_parameters
        .iter()
        .map(|p| Ident::new(p, proc_macro2::Span::call_site()))
        .collect::<Vec<_>>();

    let res_type = if response_type == Ident::new("EmptyResponse", proc_macro2::Span::call_site()) {
        quote!(())
    } else {
        quote!(#response_type)
    };

    // Generate the final code for the derive macro
    let expanded = quote! {
        impl api_forge::ApiRequest<#res_type> for #name {
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
                let mut url = format!("{}{}", base_url, Self::ENDPOINT);

                #(
                    url = url.replace(&format!("{{{}}}", #path_parameters), &self.#path_parameters_idents.to_string());
                )*

                let client = reqwest::Client::new();

                let mut builder = match Self::METHOD {
                    reqwest::Method::GET => client.get(&url),
                    reqwest::Method::POST => client.post(&url),
                    reqwest::Method::PUT => client.put(&url),
                    reqwest::Method::DELETE => client.delete(&url),
                    reqwest::Method::PATCH => client.patch(&url),
                    reqwest::Method::HEAD => client.head(&url),
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

                #(#header_inserts)*

                if let Some(headers) = headers {
                    all_headers.extend(headers);
                }

                builder.headers(all_headers)
            }
        }
    };

    TokenStream::from(expanded)
}
