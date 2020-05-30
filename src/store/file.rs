use super::{Loader, Quote, QuotesState, StateView};
use std::{fs::File, path::PathBuf, error::Error};

pub struct FileLoader {
    pub path: PathBuf,
}

#[async_trait::async_trait]
impl Loader for FileLoader {
    async fn load_quotes(&self, state: &QuotesState) -> Result<(), Box<dyn Error>> {
        println!("Loading quotes from {}", self.path.display());
        let f = File::open(self.path.clone())?;
        let fc: Vec<FileQuoteV1> = serde_json::from_reader(f)?;

        for q in fc {
            state.add_quote(&q).await;
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

        let state = QuotesState::new();
        loader.load_quotes(&state).await.unwrap();

        let qs = state.quotes.read().await;
        assert!(qs.len() > 0);

        for q in qs.iter() {
            assert_ne!(q.who, "");
            assert_ne!(q.quote, "");
        }
    }

    fn get_dev_dir() -> PathBuf {
        let file = PathBuf::from(file!()).canonicalize().unwrap();
        file.parent().unwrap()
            .parent().unwrap()
            .parent().unwrap()
            .to_path_buf()
    }
}