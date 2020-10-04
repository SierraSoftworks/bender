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
use opentelemetry::api::Key;
use prometheus::default_registry;
use tracing_log::LogTracer;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use crate::telemetry::OpenTelemetryB3;

mod api;
mod models;
mod store;
mod telemetry;

fn init_opentelemetry() {
    match std::env::var("JAEGER_COLLECTOR_ENDPOINT") {
        Ok(endpoint) if !endpoint.is_empty() => {
            let exporter = opentelemetry_jaeger::Exporter::builder()
                .with_collector_endpoint(endpoint)
                .with_process(opentelemetry_jaeger::Process {
                    service_name: "bender".to_string(),
                    tags: vec![
                        Key::new("service.version").string((release_name!()).unwrap_or("dev".into()))
                    ]
                })
                .init()
                .unwrap();
            
            let provider = opentelemetry::sdk::Provider::builder()
                .with_simple_exporter(exporter)
                .with_config(opentelemetry::sdk::Config {
                    default_sampler: Box::new(opentelemetry::sdk::Sampler::AlwaysOn),
                    ..Default::default()
                })
                .build();

            opentelemetry::global::set_provider(provider);
                    
            let tracer = opentelemetry::global::tracer("bender");
            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
            let subscriber = Registry::default()
                .with(telemetry);
            tracing::subscriber::set_global_default(subscriber).unwrap_or_default();
        },
        _ => {
            opentelemetry::global::set_provider(opentelemetry::api::NoopProvider{});
            tracing_subscriber::fmt()
                .with_max_level(tracing::Level::INFO)
                .init();
        }
    };

}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    init_opentelemetry();
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

    println!("Starting server on :8000");
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(metrics.clone())
            .wrap(TracingLogger)
            .wrap(OpenTelemetryB3)
            .wrap(Cors::new()
                .send_wildcard()
                .finish())
            .configure(api::configure)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}