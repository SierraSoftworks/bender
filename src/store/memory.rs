use crate::api::APIError;
use crate::telemetry::*;
use crate::{models::*, trace_handler};
use actix::prelude::*;
use rand::seq::{IteratorRandom, SliceRandom};
use std::sync::{Arc, RwLock};
use tracing_batteries::prelude::*;

pub struct MemoryStore {
    quotes: Arc<RwLock<Vec<Quote>>>,
    started_at: chrono::DateTime<chrono::Utc>,
}

impl MemoryStore {
    pub fn new() -> Self {
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

    #[tracing::instrument(err, skip(self), name="add_quote", fields(otel.kind = "internal"))]
    fn handle(&mut self, msg: AddQuote, _: &mut Self::Context) -> Self::Result {
        let mut qs = info_span!(
            "lock.acquire",
            "otel.kind" = "internal",
            db.instance = "quotes",
            db.statement = "WRITE"
        )
        .in_scope(|| {
            self.quotes.write().map_err(|exception| {
                error!(
                    "Unable to acquire write lock on quotes collection: {:?}",
                    exception
                );
                APIError::new(
                    500,
                    "Internal Server Error",
                    "The service is currently unavailable, please try again later.",
                )
            })
        })?;

        qs.push(Quote {
            who: msg.who,
            quote: msg.quote,
        });

        Ok(())
    }
}

trace_handler!(MemoryStore, AddQuotes, Result<(), APIError>);

impl Handler<AddQuotes> for MemoryStore {
    type Result = Result<(), APIError>;

    #[tracing::instrument(err, skip(self), name="add_quotes", fields(otel.kind = "internal"))]
    fn handle(&mut self, msg: AddQuotes, _: &mut Self::Context) -> Self::Result {
        let mut qs = info_span!(
            "lock.acquire",
            "otel.kind" = "internal",
            db.instance = "quotes",
            db.statement = "WRITE"
        )
        .in_scope(|| {
            self.quotes.write().map_err(|exception| {
                error!(
                    "Unable to acquire write lock on quotes collection: {:?}",
                    exception
                );
                APIError::new(
                    500,
                    "Internal Server Error",
                    "The service is currently unavailable, please try again later.",
                )
            })
        })?;

        for quote in msg.quotes {
            qs.push(quote);
        }

        Ok(())
    }
}

trace_handler!(MemoryStore, GetQuote, Result<Quote, APIError>);

impl Handler<GetQuote> for MemoryStore {
    type Result = Result<Quote, APIError>;

    #[tracing::instrument(err, skip(self), name="get_quote", fields(otel.kind = "internal"))]
    fn handle(&mut self, msg: GetQuote, _: &mut Self::Context) -> Self::Result {
        let qs = info_span!(
            "lock.acquire",
            "otel.kind" = "internal",
            db.instance = "quotes",
            db.statement = "WRITE"
        )
        .in_scope(|| {
            self.quotes.read().map_err(|exception| {
                error!(
                    "Unable to acquire read lock on quotes collection: {:?}",
                    exception
                );
                APIError::new(
                    500,
                    "Internal Server Error",
                    "The service is currently unavailable, please try again later.",
                )
            })
        })?;

        let quote = info_span!("quotes.choose").in_scope(|| {
            if msg.who.is_empty() {
                qs.choose(&mut rand::thread_rng())
            } else {
                qs.iter()
                    .filter(|q| q.who.to_lowercase() == msg.who.to_lowercase())
                    .choose(&mut rand::thread_rng())
            }
        });

        quote
            .ok_or_else(|| {
                APIError::new(
                    404,
                    "Not Found",
                    "There are no quotes available right now, please add one and try again.",
                )
            })
            .cloned()
    }
}

trace_handler!(MemoryStore, GetHealth, Result<Health, APIError>);

impl Handler<GetHealth> for MemoryStore {
    type Result = Result<Health, APIError>;

    #[tracing::instrument(err, skip(self), name="get_health", fields(otel.kind = "internal"))]
    fn handle(&mut self, _: GetHealth, _: &mut Self::Context) -> Self::Result {
        Ok(Health {
            ok: true,
            started_at: self.started_at,
        })
    }
}
