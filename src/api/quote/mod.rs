mod models;
mod state;
#[cfg(test)]
mod test;

use actix_web::{get, web, HttpRequest, Result};

pub use self::state::{Quote, QuotesState};
use super::APIError;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(quote_v1)
        .service(quote_by_v1);
}

#[get("/api/v1/quote")]
pub async fn quote_v1(state: web::Data<state::QuotesState>) -> Result<models::QuoteV1, APIError> {
    state.quote().await.ok_or(APIError::new(404, "Not Found", "No quotes were present in the collection."))
}

#[get("/api/v1/quote/{person}")]
pub async fn quote_by_v1(state: web::Data<state::QuotesState>, request: HttpRequest) -> Result<models::QuoteV1, APIError> {
    state.quote_by(request.match_info().get("person").unwrap()).await.ok_or(APIError::new(404, "Not Found", "No quotes were present for the provided author."))
}
