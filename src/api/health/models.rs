use crate::{json_responder, models::*};

#[derive(Serialize, Deserialize)]
pub struct HealthV1 {
    pub ok: bool,
}

impl From<Health> for HealthV1 {
    fn from(state: Health) -> Self {
        Self { ok: state.ok }
    }
}

impl Into<Health> for HealthV1 {
    fn into(self) -> Health {
        Health {
            ok: self.ok,
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

impl Into<Health> for HealthV2 {
    fn into(self) -> Health {
        Health {
            ok: self.ok,
            started_at: self.started_at,
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