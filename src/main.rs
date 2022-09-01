#[macro_use] extern crate serde;
extern crate actix_web;
extern crate chrono;
extern crate serde_json;
extern crate uuid;
extern crate mime;
extern crate tokio;
extern crate rand;
#[macro_use] extern crate tracing;
extern crate sentry;

use actix_web::{App, HttpServer, web::Data};
use telemetry::Session;
use tracing::{Instrument, info_span};

mod api;
#[macro_use] mod macros;
mod models;
mod store;
mod telemetry;

fn get_listening_port() -> u16 {
    std::env::var("FUNCTIONS_CUSTOMHANDLER_PORT").map(|v| v.parse().unwrap_or(8000)).unwrap_or(8000)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let _session = Session::new();

    let state = api::GlobalState::new();

    info!("Preparing service to start");
    store::load_global_state(&store::file::FileLoader {
        path: std::path::PathBuf::from("./quotes.json"),
    }, &state).instrument(info_span!("Loading global state information", "otel.kind" = "internal")).await.map_err(|err| {
        error!({ exception.message = %err }, "Unable to load global state information");
    }).unwrap_or_default();

    let listen_on = get_listening_port();

    info!("Starting server on :{}", listen_on);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .wrap(telemetry::TracingLogger)
            .configure(api::configure)
    })
    .bind(format!("0.0.0.0:{}", listen_on))?
    .run()
    .await
}