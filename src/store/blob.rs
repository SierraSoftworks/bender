use actix::prelude::*;
use azure_core::HttpClient;
use tracing::*;
use tracing_futures::Instrument;
use super::{Loader, Store};
use crate::{api::APIError, models::*};
use crate::telemetry::*;
use std::{path::PathBuf, sync::Arc};

use azure_storage::core::prelude::*;
use azure_storage::blob::prelude::*;

pub struct BlobLoader {
    pub connection_string: String,
    pub container: String,
    pub path: PathBuf,
}

#[async_trait::async_trait]
impl Loader for BlobLoader {
    #[instrument(err, skip(self, state), fields(otel.kind = "internal"))]
    async fn load_quotes(&self, state: Addr<Store>) -> Result<(), APIError> {
        debug!("Initializing Azure Blob storage client");
        let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));

        let storage_client = StorageAccountClient::new_connection_string(http_client.clone(), self.connection_string.as_str())
            .map_err(|err| {
                error!({ exception.message = %err }, "Unable to create Azure Blob Storage client");
                APIError::new(503, "Service Unavailable", "We're sorry, but we can't seem to find any quotes around here right now. Please check back soon.")
            })?;

        let blob_client = storage_client
            .as_storage_client()
            .as_container_client(&self.container)
            .as_blob_client(&self.path.to_string_lossy().to_string());

        debug!("Fetching {}", self.path.display());
        let blob = blob_client
                    .get()
                    .execute()
                    .instrument(info_span!("get_blob", "otel.kind" = "client", db.system = "azure_storage", db.instance = self.container.as_str(), db.statement = format!("GET {}", self.path.display()).as_str()))
                    .await
                    .map_err(|err| {
                        error!({ exception.message = %err }, "Failed to fetch quote file from Azure Blob Storage.");
                        APIError::new(503, "Service Unavailable", "We're sorry, but we can't seem to find any quotes around here right now. Please check back soon.")
                    })?;

        let fc: Vec<BlobQuoteV1> = debug_span!("deserialize").in_scope(|| serde_json::from_slice(&blob.data) )
            .map_err(|err| {
                error!({ exception.message = %err }, "Unable to parse quote file.");
                APIError::new(503, "Service Unavailable", "We're sorry, but we can't seem to find any quotes around here right now. Please check back soon.")
            })?;

        let quote_count = fc.len();
        info!("Received {} quotes from Azure blob storage", quote_count);

        match state.send(AddQuotes {
            quotes: fc.iter().map(|q| q.clone().into()).collect()
        }.trace()).await? {
            Ok(_) => {
                event!(Level::INFO, "Loaded {} quotes into the state store.", quote_count);
                Ok(())
            },
            Err(err) => {
                error!("Failed to load quotes from {}: {}", self.path.display(), err);
                Err(err)
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlobQuoteV1 {
    pub quote: String,
    pub who: String,
}

impl Into<Quote> for BlobQuoteV1 {
    fn into(self) -> Quote {
        Quote {
            quote: self.quote.clone(),
            who: self.who.clone(),
        }
    }
}

impl From<Quote> for BlobQuoteV1 {
    fn from(state: Quote) -> Self {
        Self {
            quote: state.quote.clone(),
            who: state.who.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[actix_rt::test]
    #[ignore] // Test disabled until we have support for the storage emulator
    async fn load_quotes() {
        let loader = BlobLoader {
            connection_string: "UseDevelopmentStorage=true".to_string(),
            container: "quotes".to_string(),
            path: PathBuf::from("quotes.json"),
        };

        let state = Store::new().start();
        loader.load_quotes(state.clone()).await.unwrap();

        state.send(GetQuote{who:"".to_string()}).await.expect("the actor should respond").expect("we should get a quote");
        state.send(GetQuote{who:"Bender".to_string()}).await.expect("the actor should respond").expect("we should get a quote");
        state.send(GetQuote{who:"bEnDeR".to_string()}).await.expect("the actor should respond").expect("we should get a quote");
    }
}