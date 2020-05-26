#[macro_use]
extern crate serde;

extern crate actix_web;
extern crate chrono;
extern crate serde_json;
extern crate uuid;
extern crate mime;
extern crate tokio;
extern crate rand;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};

mod api;
mod store;

use store::Loader;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let health_state = web::Data::new(api::health::HealthState::new());
    let quote_state = web::Data::new(api::quote::QuotesState::new());

    println!("Loading quotes from ./quotes.json");
    let loader = store::file::FileLoader{
        path: std::path::PathBuf::from("./quotes.json"),
    };
    loader.load_quotes(&quote_state).await?;

    println!("Starting server on :8000");
    HttpServer::new(move || {
        App::new()
            .app_data(health_state.clone())
            .app_data(quote_state.clone())
            .wrap(middleware::Logger::default())
            .wrap(Cors::new().send_wildcard().allowed_origin("All").finish())
            .configure(api::health::configure)
            .configure(api::quote::configure)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}