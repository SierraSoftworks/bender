use actix::prelude::*;
use tracing::*;
use super::{Loader, Store};
use crate::{api::APIError, models::*};
use crate::telemetry::*;
use std::{fs::File, path::PathBuf};

pub struct FileLoader {
    pub path: PathBuf,
}

#[async_trait::async_trait]
impl Loader for FileLoader {
    #[instrument(err, skip(self, state), fields(otel.kind = "internal"))]
    async fn load_quotes(&self, state: Addr<Store>) -> Result<(), APIError> {
        info!("Loading quotes from {}", self.path.display());

        debug!("Opening file");
        let f = File::open(self.path.clone())
                .map_err(|err| {
                    error!({ exception.message = ?err }, "Unable to open quotes file.");
                    APIError::new(503, "Service Unavailable", "We could not load the quotes needed to give you your daily dose of fun, we're so sorry.")
                })?;

        
        let fc: Vec<FileQuoteV1> = info_span!("read_file").in_scope(|| serde_json::from_reader(f) ).map_err(|err| {
            error!({ exception.message = ?err }, "Unable to parse quotes file.");
            APIError::new(503, "Service Unavailable", "We could not load the quotes needed to give you your daily dose of fun, we're so sorry.")
        })?;
        let quote_count = fc.len();
        info!("Read {} quotes from file", quote_count);

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
pub struct FileQuoteV1 {
    pub quote: String,
    pub who: String,
}

impl Into<Quote> for FileQuoteV1 {
    fn into(self) -> Quote {
        Quote {
            quote: self.quote.clone(),
            who: self.who.clone(),
        }
    }
}

impl From<Quote> for FileQuoteV1 {
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
    async fn load_quotes() {
        let loader = FileLoader{
            path: get_dev_dir().join("quotes.json"),
        };

        let state = Store::new().start();
        loader.load_quotes(state.clone()).await.unwrap();

        state.send(GetQuote{who:"".to_string()}).await.expect("the actor should respond").expect("we should get a quote");
        state.send(GetQuote{who:"Bender".to_string()}).await.expect("the actor should respond").expect("we should get a quote");
        state.send(GetQuote{who:"bEnDeR".to_string()}).await.expect("the actor should respond").expect("we should get a quote");
    }

    fn get_dev_dir() -> PathBuf {
        let file = PathBuf::from(file!()).canonicalize().unwrap();
        file.parent().unwrap()
            .parent().unwrap()
            .parent().unwrap()
            .to_path_buf()
    }
}