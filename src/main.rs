#[macro_use] extern crate serde;
extern crate actix_web;
extern crate chrono;
extern crate serde_json;
extern crate uuid;
extern crate mime;
extern crate tokio;
extern crate rand;
#[macro_use] extern crate log;
#[macro_use] extern crate sentry;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prometheus;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use prometheus::default_registry;
use tracing_log::LogTracer;
use tracing_actix_web::TracingLogger;

mod api;
mod models;
mod store;
mod telemetry;

fn get_listening_port() -> u16 {
    std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT").map(|v| v.parse().unwrap_or(8000)).unwrap_or(8000)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let (_tracer, _uninstall) = opentelemetry_application_insights::new_pipeline(
        std::env::var("APPINSIGHTS_INSTRUMENTATIONKEY").unwrap_or_default()
    )
        .with_client(reqwest::Client::new())
        .install();


    LogTracer::init().unwrap_or_default();

    let _raven = sentry::init((
        "https://950ba56ab61a4abcb3679b1117158c33@o219072.ingest.sentry.io/1362607",
        sentry::ClientOptions {
            release: release_name!(),
            ..Default::default()
        },
    ));

    let state = api::GlobalState::new();
    let metrics = PrometheusMetrics::new_with_registry(default_registry().clone(), "bender", Some("/api/v1/metrics"), None).unwrap();

    store::load_global_state(&store::file::FileLoader {
        path: std::path::PathBuf::from("./quotes.json"),
    }, &state).await?;

    let listen_on = get_listening_port();

    info!("Starting server on :{}", listen_on);
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(metrics.clone())
            .wrap(TracingLogger)
            .wrap(Cors::new()
                .send_wildcard()
                .finish())
            .configure(api::configure)
    })
    .bind(format!("0.0.0.0:{}", listen_on))?
    .run()
    .await
}