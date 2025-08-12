#[macro_use]
extern crate serde;
extern crate actix_web;
extern crate chrono;
extern crate mime;
extern crate rand;
extern crate serde_json;
extern crate tokio;
extern crate uuid;

use actix_web::{web::Data, App, HttpServer};
use tracing_batteries::prelude::*;

mod api;
#[macro_use]
mod macros;
mod models;
mod store;
mod telemetry;

fn get_listening_port() -> u16 {
    std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT")
        .map(|v| v.parse().unwrap_or(8000))
        .unwrap_or(8000)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let session = telemetry::setup();

    let state = api::GlobalState::new();

    info!("Preparing service to start");
    store::load_global_state(
        &store::file::FileLoader {
            path: std::path::PathBuf::from("./quotes.json"),
        },
        &state,
    )
    .instrument(info_span!(
        "Loading global state information",
        "otel.kind" = "internal"
    ))
    .await
    .map_err(|err| {
        error!({ exception.message = %err }, "Unable to load global state information");
    })
    .unwrap_or_default();

    let listen_on = get_listening_port();

    info!("Starting server on :{}", listen_on);
    let result = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(telemetry::TracingLogger)
            .configure(api::configure)
    })
    .bind(format!("0.0.0.0:{listen_on}"))?
    .run()
    .await;

    if let Err(error) = &result {
        eprintln!("Error starting server: {error}");
        session.record_error(error);
    }

    session.shutdown();
    result
}
