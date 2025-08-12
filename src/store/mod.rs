pub mod blob;
pub mod file;
pub mod memory;

use crate::api::APIError;
use actix::prelude::*;
use tracing_batteries::prelude::*;

use super::api::GlobalState;

pub type Store = memory::MemoryStore;

#[async_trait::async_trait]
pub trait Loader {
    async fn load_quotes(&self, state: Addr<Store>) -> Result<(), APIError>;
}

#[tracing::instrument(err, skip(loader, state))]
pub async fn load_global_state(loader: &dyn Loader, state: &GlobalState) -> Result<(), APIError> {
    loader.load_quotes(state.store.clone()).await
}
