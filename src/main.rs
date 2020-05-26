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
use actix_web::{middleware, App, HttpServer};

mod api;
mod store;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let state = api::GlobalStateManager::new();

    println!("Loading quotes from ./quotes.json");
    store::load_global_state(&store::file::FileLoader {
        path: std::path::PathBuf::from("./quotes.json"),
    }, &state).await?;

    println!("Starting server on :8000");
    HttpServer::new(move || {
        state.configure(App::new())
            .wrap(middleware::Logger::default())
            .wrap(Cors::new().send_wildcard().allowed_origin("All").finish())
            .configure(api::configure)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}