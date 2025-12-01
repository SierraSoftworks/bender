mod models;
#[cfg(test)]
mod test;

use crate::telemetry::*;
use actix_web::{HttpRequest, get, web};
use tracing_batteries::prelude::*;

use super::APIError;
use crate::models::*;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health_v1)
        .service(health_v2)
        .service(tracing_v1);
}

#[tracing::instrument(err, skip(state), fields(otel.kind = "internal"))]
#[get("/api/v1/health")]
pub async fn health_v1(state: web::Data<GlobalState>) -> Result<models::HealthV1, APIError> {
    state
        .store
        .send(GetHealth {}.trace())
        .await?
        .map(|h| h.into())
}

#[tracing::instrument(err, skip(state), fields(otel.kind = "internal"))]
#[get("/api/v2/health")]
pub async fn health_v2(state: web::Data<GlobalState>) -> Result<models::HealthV2, APIError> {
    state
        .store
        .send(GetHealth {}.trace())
        .await?
        .map(|h| h.into())
}

#[tracing::instrument(err, skip(req), fields(otel.kind = "internal"))]
#[get("/api/v1/tracing")]
pub async fn tracing_v1(req: HttpRequest) -> Result<String, APIError> {
    let headers = req
        .headers()
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or("invalid utf8")))
        .collect::<Vec<String>>()
        .join("\n");

    let traceparent = req
        .headers()
        .get("traceparent")
        .map(|v| v.to_str().unwrap_or("invalid utf8"))
        .unwrap_or("<not set>");

    let context = Span::current().context();
    let trace_span = context.span();
    let traceid = trace_span.span_context().trace_id().to_string();
    let sampling = if trace_span.span_context().trace_flags().is_sampled() {
        "enabled"
    } else {
        "disabled"
    };

    let reply = format!(
        "OpenTelemetry TraceParent is: {traceparent} (current trace ID is: {traceid}, sampling is {sampling})\n\n{headers}"
    );

    Ok(reply)
}
