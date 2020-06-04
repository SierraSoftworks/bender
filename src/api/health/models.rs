use super::super::StateView;
use crate::models::*;

use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};

#[derive(Serialize, Deserialize)]
pub struct HealthV1 {
    pub ok: bool,
}

impl StateView<Health> for HealthV1 {
    fn to_state(&self) -> Health {
        Health {
            ok: self.ok,
            started_at: chrono::Utc::now(),
        }
    }

    fn from_state(state: &Health) -> Self {
        Self { ok: state.ok }
    }
}

impl Responder for HealthV1 {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(&self)))
    }
}

#[derive(Serialize, Deserialize)]
pub struct HealthV2 {
    pub ok: bool,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl StateView<Health> for HealthV2 {
    fn to_state(&self) -> Health {
        Health {
            ok: self.ok,
            started_at: self.started_at,
        }
    }

    fn from_state(state: &Health) -> Self {
        Self {
            ok: state.ok,
            started_at: state.started_at,
        }
    }
}

impl Responder for HealthV2 {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(&self)))
    }
}