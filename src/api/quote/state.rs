use super::super::StateView;
use std::sync::Arc;
use tokio::sync::RwLock;
use rand::seq::{SliceRandom, IteratorRandom};
use prometheus::{self, IntCounterVec};

lazy_static! {
    static ref QUOTES_LOADED_COUNTER: IntCounterVec =
        register_int_counter_vec!(
            "bender_quotes_loaded_total",
            "The number of quotes which have been loaded into the Bender instance by each author..",
            &["author"]
        ).unwrap();

    static ref QUOTES_VIEWED_COUNTER: IntCounterVec =
        register_int_counter_vec!(
            "bender_quotes_viewed_total",
            "The number of times that quotes by each author have been viewed.",
            &["author"]
        ).unwrap();
}

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

    pub async fn add_quote<T: StateView<Quote>>(&self, quote: &T) {
        let mut qs = self.quotes.write().await;
        let q = T::to_state(quote);
        QUOTES_LOADED_COUNTER.with_label_values(&[q.who.as_str()]).inc();
        qs.push(q);
    }

    pub async fn quote<T: StateView<Quote>>(&self) -> Option<T> {
        let qs = self.quotes.read().await;
        
        qs.choose(&mut rand::thread_rng()).map(|q| {
            QUOTES_VIEWED_COUNTER.with_label_values(&[q.who.as_str()]).inc();
            q
        }).map(T::from_state)
    }

    pub async fn quote_by<T: StateView<Quote>>(&self, person: &str) -> Option<T> {
        let qs = self.quotes.read().await;

        qs.iter().filter(|x| x.who.to_lowercase() == person.to_lowercase()).choose(&mut rand::thread_rng()).map(|q| {
            QUOTES_VIEWED_COUNTER.with_label_values(&[q.who.as_str()]).inc();
            q
        }).map(T::from_state)
    }
}