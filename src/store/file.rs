use actix::prelude::*;
use super::{Loader, StateView, Store};
use crate::models::*;
use std::{fs::File, path::PathBuf, error::Error};
use opentelemetry::api::Tracer;
use opentelemetry::global;

pub struct FileLoader {
    pub path: PathBuf,
}

#[async_trait::async_trait]
impl Loader for FileLoader {
    async fn load_quotes(&self, state: Addr<Store>) -> Result<(), Box<dyn Error>> {
        println!("Loading quotes from {}", self.path.display());
        let _span = global::tracer("file-storage").start("open-file");
        let f = File::open(self.path.clone())?;
        let _span = global::tracer("file-storage").start("deserialize");
        let fc: Vec<FileQuoteV1> = serde_json::from_reader(f)?;

        let _span = global::tracer("file-storage").start("update-state");
        for q in fc {
            match state.send(AddQuote{
                quote: q.quote,
                who: q.who,
            }).await? {
                Ok(_) => {},
                Err(err) => {
                    error!("Failed to load quotes from {}: {}", self.path.display(), err);
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, format!("{}", err))))
                },
            }
        }

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