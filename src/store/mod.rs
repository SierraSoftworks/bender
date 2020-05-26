pub mod file;

use super::api::{Quote, QuotesState};
use std::io::Result;

#[async_trait::async_trait]
pub trait Loader {
    async fn load_quotes(&self, state: &QuotesState) -> Result<()>;
}