pub mod blob;
pub mod file;
pub mod memory;

use actix::prelude::*;
use tracing::instrument;
use super::api::{GlobalState, StateView};
use std::error::Error;

pub type Store = memory::MemoryStore;

#[async_trait::async_trait]
pub trait Loader {
    async fn load_quotes(&self, state: Addr<Store>) -> Result<(), Box<dyn Error>>;
}

#[instrument(err, skip(loader, state))]
pub async fn load_global_state(loader: &dyn Loader, state: &GlobalState) -> std::io::Result<()> {
    match loader.load_quotes(state.store.clone()).await {
        Ok(_) => Ok(()),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))?
    }
}