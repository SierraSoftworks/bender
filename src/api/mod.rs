pub mod error;
pub mod health;
mod macros;
pub mod quote;

pub use crate::models::GlobalState;
pub use error::APIError;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    health::configure(cfg);
    quote::configure(cfg);
}
