#[macro_use] extern crate serde;
extern crate actix_web;
extern crate chrono;
extern crate serde_json;
extern crate uuid;
extern crate mime;
extern crate tokio;
extern crate rand;
#[macro_use] extern crate log;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate prometheus;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use actix_web_prom::PrometheusMetrics;
use prometheus::default_registry;

mod api;
mod store;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let state = api::GlobalStateManager::new();
    let metrics = PrometheusMetrics::new_with_registry(default_registry().clone(), "bender", Some("/api/v1/metrics"), None).unwrap();

    store::load_global_state(&store::file::FileLoader {
        path: std::path::PathBuf::from("./quotes.json"),
    }, &state).await?;

    println!("Starting server on :8000");
    HttpServer::new(move || {
        state.configure(App::new())
            .wrap(metrics.clone())
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