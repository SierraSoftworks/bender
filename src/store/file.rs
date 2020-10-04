use actix::prelude::*;
use super::{Loader, StateView, Store};
use crate::models::*;
use std::{fs::File, path::PathBuf, error::Error};
use opentelemetry::api::{KeyValue, Span, StatusCode, Tracer};
use opentelemetry::global;

pub struct FileLoader {
    pub path: PathBuf,
}

#[async_trait::async_trait]
impl Loader for FileLoader {
    async fn load_quotes(&self, state: Addr<Store>) -> Result<(), Box<dyn Error>> {
        println!("Loading quotes from {}", self.path.display());
        let span = global::tracer("file-storage").start("load-quotes");
        span.set_attribute(KeyValue::new("db.type", "file"));
        span.set_attribute(KeyValue::new("db.statement", format!("GET {}", self.path.display())));

        span.add_event("file.open.start".into(), vec![]);
        let f = File::open(self.path.clone())?;
        span.add_event("file.open.end".into(), vec![]);

        span.add_event("file.read.start".into(), vec![]);
        let fc: Vec<FileQuoteV1> = serde_json::from_reader(f)?;
        span.add_event("file.read.end".into(), vec![]);

        span.add_event("store.update.start".into(), vec![]);
        let quote_count = fc.len();
        for q in fc {
            match state.send(AddQuote{
                quote: q.quote,
                who: q.who,
            }).await? {
                Ok(_) => {},
                Err(err) => {
                    error!("Failed to load quotes from {}: {}", self.path.display(), err);
                    span.set_status(StatusCode::NotFound, format!{"{:?}", err});
                    Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", err))))?;
                },
            }
        }
        span.add_event("store.update.end".into(), vec![]);

        span.set_status(StatusCode::OK, format!("Loaded {} quotes into the state store.", quote_count));
        span.end();

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct FileQuoteV1 {
    pub quote: String,
    pub who: String,
}

impl StateView<Quote> for FileQuoteV1 {
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