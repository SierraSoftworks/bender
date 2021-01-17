pub mod health;
pub mod quote;
pub mod error;

pub use error::APIError;
pub use crate::models::{GlobalState};

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    health::configure(cfg);
    quote::configure(cfg);
}