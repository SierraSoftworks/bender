pub mod blob;
pub mod file;
pub mod memory;

use actix::prelude::*;
use opentelemetry::api::{KeyValue, Span, StatusCode, Tracer};
use super::api::{GlobalState, StateView};
use std::error::Error;

pub type Store = memory::MemoryStore;

#[async_trait::async_trait]
pub trait Loader {
    async fn load_quotes(&self, state: Addr<Store>) -> Result<(), Box<dyn Error>>;
}

pub async fn load_global_state(loader: &dyn Loader, state: &GlobalState) -> std::io::Result<()> {
    let span = opentelemetry::global::tracer("store").start("load-global-state");
    
    match loader.load_quotes(state.store.clone()).await {
        Ok(_) => {
            span.add_event("quotes.loaded".into(), vec![]);
        },
        Err(e) => {
            span.set_status(StatusCode::Internal, format!("{:?}", e));
            Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?;
        }
    }

    span.set_status(StatusCode::OK, "Loaded global state".into());
    Ok(())
}