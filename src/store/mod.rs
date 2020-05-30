pub mod file;

use super::api::{GlobalStateManager, Quote, QuotesState, StateView};
use std::io::Result;

#[async_trait::async_trait]
pub trait Loader {
    async fn load_quotes(&self, state: &QuotesState) -> Result<()>;
}

pub async fn load_global_state(loader: &dyn Loader, state: &GlobalStateManager) -> Result<()> {
    loader.load_quotes(&state.quote_state).await?;

    Ok(())
}