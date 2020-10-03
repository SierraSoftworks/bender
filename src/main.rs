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
use actix_web::{middleware, App, HttpServer};
use actix_web_opentelemetry::{RequestTracing};
use actix_web_prom::PrometheusMetrics;
use prometheus::default_registry;

mod api;
mod models;
mod store;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let _raven = sentry::init((
        "https://950ba56ab61a4abcb3679b1117158c33@o219072.ingest.sentry.io/1362607",
        sentry::ClientOptions {
            release: release_name!(),
            ..Default::default()
        },
    ));

    // TODO: Update this to use a valid trace provider
    opentelemetry::global::set_provider(opentelemetry::api::NoopProvider{});

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
            .wrap(RequestTracing::default())
            .wrap(middleware::Logger::default())
            .wrap(Cors::new()
                .send_wildcard()
                .finish())
            .configure(api::configure)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}