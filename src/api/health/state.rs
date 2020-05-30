use super::super::StateView;
use prometheus::{self, IntGauge};

lazy_static! {
    static ref UP_GAUGE: IntGauge =
        register_int_gauge!("up", "The time at which the application was first started.").unwrap();
}

#[derive(Clone, Copy)]
pub struct HealthState {
    pub ok: bool,
    pub started_at: chrono::DateTime<chrono::Utc>,
}

impl HealthState {
    pub fn new() -> Self {
        let now = chrono::Utc::now();

        UP_GAUGE.set(now.timestamp());

        Self {
            ok: true,
            started_at: now.clone(),
        }
    }

    pub fn health<T: StateView<HealthState>>(&self) -> T {
        T::from_state(self)
    }
}