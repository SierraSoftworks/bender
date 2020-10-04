use std::sync::{RwLock, Arc};
use crate::models::*;
use actix::prelude::*;
use opentelemetry::{api::KeyValue, api::Span, api::StatusCode, api::Tracer, global};
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

impl Handler<AddQuote> for MemoryStore {
    type Result = Result<(), APIError>;

    fn handle(&mut self, msg: AddQuote, _: &mut Self::Context) -> Self::Result {
        let span = global::tracer("memory-store").start("add-quote");
        
        QUOTES_LOADED_COUNTER.with_label_values(&[msg.who.as_str()]).inc();

        span.add_event("lock.acquire.start".into(), vec![
            KeyValue::new("db.type", "memory"),
            KeyValue::new("db.instance", "quotes"),
            KeyValue::new("db.statement", "WRITE")
        ]);

        let mut qs = self.quotes.write().map_err(|err| {
            span.record_exception(&err);
            span.set_status(StatusCode::Aborted, "Could not acquire lock".into());

            APIError::new(500, "Internal Server Error", "The service is currently unavailable, please try again later.")
        })?;

        span.add_event("lock.acquire.end".into(), vec![
            KeyValue::new("db.type", "memory"),
            KeyValue::new("db.instance", "quotes"),
            KeyValue::new("db.statement", "WRITE")
        ]);

        qs.push(Quote {
            who: msg.who,
            quote: msg.quote,
        });

        span.set_status(StatusCode::OK, "Added quote to the in-memory store.".into());
        span.end();
        Ok(())
    }
}

impl Handler<GetQuote> for MemoryStore {
    type Result = Result<Quote, APIError>;

    fn handle(&mut self, msg: GetQuote, _: &mut Self::Context) -> Self::Result {
        let span = global::tracer("memory-store").start("get-quote");

        span.add_event("lock.acquire.start".into(), vec![
            KeyValue::new("db.type", "memory"),
            KeyValue::new("db.instance", "quotes"),
            KeyValue::new("db.statement", "READ")
        ]);

        let qs = self.quotes.read().map_err(|err| {
            span.record_exception(&err);
            span.set_status(StatusCode::Aborted, "Could not acquire lock".into());

            APIError::new(500, "Internal Server Error", "The service is currently unavailable, please try again later.")
        })?;


        span.add_event("lock.acquire.end".into(), vec![
            KeyValue::new("db.type", "memory"),
            KeyValue::new("db.instance", "quotes"),
            KeyValue::new("db.statement", "READ")
        ]);

        span.add_event("quotes.choose.start".into(), vec![]);
        let quote = if msg.who.is_empty() {
            qs.choose(&mut rand::thread_rng())
        } else {
            qs.iter().filter(|q| q.who.to_lowercase() == msg.who.to_lowercase()).choose(&mut rand::thread_rng())
        };
        span.add_event("quotes.choose.end".into(), vec![]);

        quote
            .ok_or({
                span.set_status(StatusCode::NotFound, "No quotes available matching the precondition.".into());
                span.end();

                APIError::new(404, "Not Found", "There are no quotes available right now, please add one and try again.")
            })
            .map(|q| {
                QUOTES_VIEWED_COUNTER.with_label_values(&[q.who.as_str()]).inc();
                span.set_status(StatusCode::OK, "Added quote to the in-memory store.".into());
                span.end();
                q.clone()
            })
    }
}

impl Handler<GetHealth> for MemoryStore {
    type Result = Result<Health, APIError>;

    fn handle(&mut self, _: GetHealth, _: &mut Self::Context) -> Self::Result {
        Ok(Health {
            ok: true,
            started_at: self.started_at.clone(),
        })
    }
}