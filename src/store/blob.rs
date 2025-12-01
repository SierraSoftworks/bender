use super::{Loader, Store};
use crate::telemetry::*;
use crate::{api::APIError, models::*};
use actix::prelude::*;
#[allow(unused_imports)]
use azure_identity::{DeveloperToolsCredential, ManagedIdentityCredential};
use azure_storage_blob::{BlobClient, BlobClientOptions};
use std::path::PathBuf;
use tracing_batteries::prelude::*;

#[allow(dead_code)]
pub struct BlobLoader {
    pub account_name: String,
    pub container: String,
    pub path: PathBuf,
}

#[async_trait::async_trait]
impl Loader for BlobLoader {
    #[tracing::instrument(err, skip(self, state), fields(otel.kind = "internal"))]
    async fn load_quotes(&self, state: Addr<Store>) -> Result<(), APIError> {
        debug!("Initializing Azure Blob storage client");
        #[cfg(debug_assertions)]
        let credential = ManagedIdentityCredential::new(None).map_err(|err| {
            error!({ exception.message = %err }, "Failed to create Managed Identity Credential for Azure Blob Storage.");
            APIError::new(503, "Service Unavailable", "We're sorry, but we can't seem to load quotes right now. Please try again later...")
        })?;
        #[cfg(not(debug_assertions))]
        let credential = DeveloperToolsCredential::new(None).map_err(|err| {
            error!({ exception.message = %err }, "Failed to create Developer Tools Credential for Azure Blob Storage.");
            APIError::new(503, "Service Unavailable", "We're sorry, but we can't seem to load quotes right now. Please try again later...")
        })?;

        let blob_client = BlobClient::new(
            &format!("https://{}.blob.core.windows.net", self.account_name),
            self.container.as_str(),
            self.path.to_string_lossy().as_ref(),
            Some(credential),
            Some(BlobClientOptions::default())
        ).map_err(|err| {
            error!({ exception.message = %err }, "Failed to create Azure Blob Client.");
            APIError::new(503, "Service Unavailable", "We're sorry, but we can't seem to load quotes right now. Please try again later...")
        })?;

        debug!("Fetching {}", self.path.display());
        let blob = blob_client.download(None)
                    .instrument(info_span!("get_blob", "otel.kind" = "client", db.system = "azure_storage", db.instance = self.container.as_str(), db.statement = format!("GET {}", self.path.display()).as_str()))
                    .await
                    .map_err(|err| {
                        error!({ exception.message = %err }, "Failed to fetch quote file from Azure Blob Storage.");
                        APIError::new(503, "Service Unavailable", "We're sorry, but we can't seem to find any quotes around here right now. Please check back soon.")
                    })?;

        let body = blob.into_body().collect().await.map_err(|err| {
            error!({ exception.message = %err }, "Failed to read quote file from Azure Blob Storage.");
            APIError::new(503, "Service Unavailable", "We're sorry, but we can't seem to find any quotes around here right now. Please check back soon.")
        })?;

        let fc: Vec<BlobQuoteV1> = debug_span!("deserialize").in_scope(|| serde_json::from_slice(&body))
            .map_err(|err| {
                error!({ exception.message = %err }, "Unable to parse quote file.");
                APIError::new(503, "Service Unavailable", "We're sorry, but we can't seem to find any quotes around here right now. Please check back soon.")
            })?;

        let quote_count = fc.len();
        info!("Received {} quotes from Azure blob storage", quote_count);

        match state
            .send(
                AddQuotes {
                    quotes: fc.iter().map(|q| q.clone().into()).collect(),
                }
                .trace(),
            )
            .await?
        {
            Ok(_) => {
                info!("Loaded {} quotes into the state store.", quote_count);
                Ok(())
            }
            Err(err) => {
                error!(
                    "Failed to load quotes from {}: {}",
                    self.path.display(),
                    err
                );
                Err(err)
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlobQuoteV1 {
    pub quote: String,
    pub who: String,
}

impl From<BlobQuoteV1> for Quote {
    fn from(val: BlobQuoteV1) -> Self {
        Quote {
            quote: val.quote.clone(),
            who: val.who,
        }
    }
}

impl From<Quote> for BlobQuoteV1 {
    fn from(state: Quote) -> Self {
        Self {
            quote: state.quote.clone(),
            who: state.who,
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
            account_name: "devstoreaccount1".to_string(),
            container: "quotes".to_string(),
            path: PathBuf::from("quotes.json"),
        };

        let state = Store::new().start();
        loader.load_quotes(state.clone()).await.unwrap();

        state
            .send(GetQuote {
                who: "".to_string(),
            })
            .await
            .expect("the actor should respond")
            .expect("we should get a quote");
        state
            .send(GetQuote {
                who: "Bender".to_string(),
            })
            .await
            .expect("the actor should respond")
            .expect("we should get a quote");
        state
            .send(GetQuote {
                who: "bEnDeR".to_string(),
            })
            .await
            .expect("the actor should respond")
            .expect("we should get a quote");
    }
}
