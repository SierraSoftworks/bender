use super::{Loader, Quote, QuotesState, StateView, Error};
use std::{path::PathBuf};

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
    async fn load_quotes(&self, state: &QuotesState) -> Result<(), Box<dyn Error>> {
        let client = Client::from_connection_string(self.connection_string.as_str())?;

        let blob = client
                    .get_blob()
                    .with_container_name(self.container.as_str())
                    .with_blob_name(self.path.to_string_lossy().to_string().as_str())
                    .finalize()
                    .await?;


        let fc: Vec<BlobQuoteV1> = serde_json::from_slice(&blob.data)?;

        for q in fc {
            state.add_quote(&q).await;
        }
        
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

        let state = QuotesState::new();
        loader.load_quotes(&state).await.unwrap();

        let qs = state.quotes.read().await;
        assert!(qs.len() > 0);

        for q in qs.iter() {
            assert_ne!(q.who, "");
            assert_ne!(q.quote, "");
        }
    }
}