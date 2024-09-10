#![feature(associated_type_defaults)]

pub use crate::error::*;
pub use crate::traits::*;
pub use api_forge_macro::Request;

pub mod error;
pub mod traits;
