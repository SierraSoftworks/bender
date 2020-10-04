mod models;
#[cfg(test)]
mod test;

use actix_web::{get, web};
use tracing::{instrument};
use crate::telemetry::*;

use crate::models::*;
use super::{StateView, APIError};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(health_v1)
        .service(health_v2);
}

#[instrument(err, skip(state))]
#[get("/api/v1/health")]
pub async fn health_v1(state: web::Data<GlobalState>) -> Result<models::HealthV1, APIError> {
    state.store.send(GetHealth {}.trace()).await?.map(|h| models::HealthV1::from_state(&h))
}

#[instrument(err, skip(state))]
#[get("/api/v2/health")]
pub async fn health_v2(state: web::Data<GlobalState>) -> Result<models::HealthV2, APIError> {
    state.store.send(GetHealth {}.trace()).await?.map(|h| models::HealthV2::from_state(&h))
}