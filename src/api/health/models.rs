use crate::{json_responder, models::*};
use tracing_batteries::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct HealthV1 {
    pub ok: bool,
}

impl From<Health> for HealthV1 {
    fn from(state: Health) -> Self {
        Self { ok: state.ok }
    }
}

impl From<HealthV1> for Health {
    fn from(val: HealthV1) -> Self {
        Health {
            ok: val.ok,
            started_at: chrono::Utc::now(),
        }
    }
}

json_responder!(HealthV1);

#[derive(Serialize, Deserialize)]
pub struct HealthV2 {
    pub ok: bool,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl From<HealthV2> for Health {
    fn from(val: HealthV2) -> Self {
        Health {
            ok: val.ok,
            started_at: val.started_at,
        }
    }
}

impl From<Health> for HealthV2 {
    fn from(state: Health) -> Self {
        Self {
            ok: state.ok,
            started_at: state.started_at,
        }
    }
}

json_responder!(HealthV2);