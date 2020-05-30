pub mod blob;
pub mod file;

use super::api::{GlobalStateManager, Quote, QuotesState, StateView};
use std::error::Error;

#[async_trait::async_trait]
pub trait Loader {
    async fn load_quotes(&self, state: &QuotesState) -> Result<(), Box<dyn Error>>;
}

pub async fn load_global_state(loader: &dyn Loader, state: &GlobalStateManager) -> std::io::Result<()> {
    match loader.load_quotes(&state.quote_state).await {
        Ok(_) => Ok(()),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e)))
    }
}