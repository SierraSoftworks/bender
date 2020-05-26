use super::super::StateView;
use std::sync::Arc;
use tokio::sync::RwLock;
use rand::seq::{SliceRandom, IteratorRandom};

#[derive(Clone)]
pub struct QuotesState {
    pub quotes: Arc<RwLock<Vec<Quote>>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Quote {
    pub quote: String,
    pub who: String,
}

impl<'a> QuotesState {
    pub fn new() -> Self {
        Self {
            quotes: Arc::new(RwLock::new(vec![])),
        }
    }

    pub async fn quote<T: StateView<Quote>>(&self) -> Option<T> {
        let qs = self.quotes.read().await;
        let q = qs.choose(&mut rand::thread_rng());

        q.map(T::from_state)
    }

    pub async fn quote_by<T: StateView<Quote>>(&self, person: &str) -> Option<T> {
        let qs = self.quotes.read().await;

        let q = qs.iter().filter(|x| x.who == person).choose(&mut rand::thread_rng());

        q.map(T::from_state)
    }
}