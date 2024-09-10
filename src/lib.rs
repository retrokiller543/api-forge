#![feature(associated_type_defaults)]

pub use crate::error::*;
pub use crate::traits::*;
pub mod error;
pub mod traits;
/*
pub struct ApiForge {
    base_url: String,
}

impl ApiForge {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }

    pub fn send_request<Req: Request>(&self, request: &Req, headers: Option<HeaderMap>) -> reqwest::Result<Response> {
        request.send_request(&self.base_url, headers)
    }
}*/
