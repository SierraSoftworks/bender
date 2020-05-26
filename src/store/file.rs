use super::{Loader, Quote, QuotesState};
use std::{fs::File, path::PathBuf, io::Result};

pub struct FileLoader {
    pub path: PathBuf,
}

#[async_trait::async_trait]
impl Loader for FileLoader {
    async fn load_quotes(&self, state: &QuotesState) -> Result<()> {
        let f = File::open(self.path.clone())?;
        let mut fc: Vec<Quote> = serde_json::from_reader(f)?;
        
        let mut qs = state.quotes.write().await;
        qs.append(&mut fc);

        Ok(())
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