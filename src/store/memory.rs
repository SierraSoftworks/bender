use std::sync::{RwLock, Arc};
use crate::{models::*, trace_handler};
use crate::telemetry::*;
use actix::prelude::*;
use tracing::{info_span, instrument};
use crate::api::APIError;
use rand::seq::{SliceRandom, IteratorRandom};
use prometheus::{self, IntGauge, IntCounterVec};

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

    static ref UP_GAUGE: IntGauge =
        register_int_gauge!("process_start_time_seconds", "The time at which the application was first started.").unwrap();
}

pub struct MemoryStore {
    quotes: Arc<RwLock<Vec<Quote>>>,
    started_at: chrono::DateTime<chrono::Utc>,
}

impl MemoryStore {
    pub fn new() -> Self {
        UP_GAUGE.set(chrono::Utc::now().timestamp());

        Self {
            quotes: Arc::new(RwLock::new(Vec::new())),
            started_at: chrono::Utc::now(),
        }
    }
}

impl Actor for MemoryStore {
    type Context = Context<Self>;
}

trace_handler!(MemoryStore, AddQuote, Result<(), APIError>);

impl Handler<AddQuote> for MemoryStore {
    type Result = Result<(), APIError>;

    #[instrument(err, skip(self), name="add_quote")]
    fn handle(&mut self, msg: AddQuote, _: &mut Self::Context) -> Self::Result {
        QUOTES_LOADED_COUNTER.with_label_values(&[msg.who.as_str()]).inc();

        let mut qs = info_span!("lock.acquire", db.instance="quotes", db.statement="WRITE").in_scope(|| {
            self.quotes.write().map_err(|exception| {
                error!("Unable to acquire write lock on quotes collection: {:?}", exception);
                APIError::new(500, "Internal Server Error", "The service is currently unavailable, please try again later.")
            })
        })?;

        qs.push(Quote {
            who: msg.who,
            quote: msg.quote,
        });

        Ok(())
    }
}

trace_handler!(MemoryStore, GetQuote, Result<Quote, APIError>);

impl Handler<GetQuote> for MemoryStore {
    type Result = Result<Quote, APIError>;

    #[instrument(err, skip(self), name="get_quote")]
    fn handle(&mut self, msg: GetQuote, _: &mut Self::Context) -> Self::Result {

        let qs = info_span!("lock.acquire", db.instance="quotes", db.statement="WRITE").in_scope(|| {
            self.quotes.read().map_err(|exception| {
                error!("Unable to acquire read lock on quotes collection: {:?}", exception);
                APIError::new(500, "Internal Server Error", "The service is currently unavailable, please try again later.")
            })
        })?;

        let quote = info_span!("quotes.choose").in_scope(|| {
            if msg.who.is_empty() {
                qs.choose(&mut rand::thread_rng())
            } else {
                qs.iter().filter(|q| q.who.to_lowercase() == msg.who.to_lowercase()).choose(&mut rand::thread_rng())
            }
        });

        quote
            .ok_or({
                APIError::new(404, "Not Found", "There are no quotes available right now, please add one and try again.")
            })
            .map(|q| {
                QUOTES_VIEWED_COUNTER.with_label_values(&[q.who.as_str()]).inc();
                q.clone()
            })
    }
}

trace_handler!(MemoryStore, GetHealth, Result<Health, APIError>);

impl Handler<GetHealth> for MemoryStore {
    type Result = Result<Health, APIError>;

    #[instrument(err, skip(self), name="get_health")]
    fn handle(&mut self, _: GetHealth, _: &mut Self::Context) -> Self::Result {
        Ok(Health {
            ok: true,
            started_at: self.started_at.clone(),
        })
    }
}