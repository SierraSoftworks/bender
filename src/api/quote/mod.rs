mod models;
#[cfg(test)]
mod test;

use actix_web::{get, web, HttpRequest, Result};

use crate::models::*;
use super::{GlobalState, APIError, StateView};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(quote_v1)
        .service(quote_by_v1);
}

#[get("/api/v1/quote")]
pub async fn quote_v1(state: web::Data<GlobalState>) -> Result<models::QuoteV1, APIError> {
    state.store.send(GetQuote{
        who: "".to_string(),
    }).await?.map(|q| models::QuoteV1::from_state(&q))
}

#[get("/api/v1/quote/{person}")]
pub async fn quote_by_v1(state: web::Data<GlobalState>, request: HttpRequest) -> Result<models::QuoteV1, APIError> {
    state.store.send(GetQuote{
        who: request.match_info().get("person").unwrap().to_string()
    }).await?.map(|q| models::QuoteV1::from_state(&q))
}
