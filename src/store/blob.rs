use actix::prelude::*;
use tracing::{Level, event, instrument, span};
use tracing_futures::Instrument;
use super::{Loader, StateView, Store};
use crate::models::*;
use std::{path::PathBuf, error::Error};

use azure_sdk_core::prelude::*;
use azure_sdk_storage_blob::prelude::*;
use azure_sdk_storage_core::prelude::*;

pub struct BlobLoader {
    pub connection_string: String,
    pub container: String,
    pub path: PathBuf,
}

#[async_trait::async_trait]
impl Loader for BlobLoader {
    #[instrument(err, skip(self, state))]
    async fn load_quotes(&self, state: Addr<Store>) -> Result<(), Box<dyn Error>> {
        debug!("Initializing Azure Blob storage client");
        let blob_client = client::from_connection_string(self.connection_string.as_str())?;

        debug!("Fetching {}", self.path.display());
        let blob = blob_client
                    .get_blob()
                    .with_container_name(self.container.as_str())
                    .with_blob_name(self.path.to_string_lossy().to_string().as_str())
                    .finalize()
                    .instrument(span!(Level::INFO, "get_blob", db.instance = self.container.as_str(), db.statement = format!("GET {}", self.path.display()).as_str()))
                    .await?;

        let fc: Vec<BlobQuoteV1> = span!(Level::DEBUG, "deserialize").in_scope(|| serde_json::from_slice(&blob.data) )?;
        let quote_count = fc.len();
        info!("Received {} quotes from Azure blob storage", quote_count);

        for q in fc {
            match state.send(AddQuote{
                quote: q.quote,
                who: q.who,
            }.trace())
            .await? {
                Ok(_) => {},
                Err(err) => {
                    error!("Failed to load quotes from {}: {}", self.path.display(), err);
                    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", err))))?;
                },
            }
        }

        event!(Level::INFO, "Loaded {} quotes into the state store.", quote_count);

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct BlobQuoteV1 {
    pub quote: String,
    pub who: String,
}

impl StateView<Quote> for BlobQuoteV1 {
    fn to_state(&self) -> Quote {
        Quote {
            quote: self.quote.clone(),
            who: self.who.clone(),
        }
    }

    fn from_state(state: &Quote) -> Self {
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