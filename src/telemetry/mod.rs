mod actix_message;
mod actix_web_tracing;

pub use actix_web_tracing::TracingLogger;
pub use actix_message::*;

use tracing_batteries::{Session, Sentry, OpenTelemetry};

pub fn setup() -> Session {
    Session::new("bender", version!())
        .with_battery(Sentry::new("https://950ba56ab61a4abcb3679b1117158c33@o219072.ingest.sentry.io/1362607"))
        .with_battery(OpenTelemetry::new("https://api.honeycomb.io:443")
            .with_header("x-honeycomb-team", std::env::var("HONEYCOMB_KEY").unwrap_or_default()))
}