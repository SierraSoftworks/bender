pub mod health;
pub mod quote;
pub mod error;

pub use quote::{Quote, QuotesState};
pub use error::APIError;

use actix_web::{web, App, dev, error::Error};
use actix_service::ServiceFactory;

pub trait StateView<T> {
    fn from_state(state: &T) -> Self;
    fn to_state(&self) -> T;
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    health::configure(cfg);
    quote::configure(cfg);
}

#[derive(Clone)]
pub struct GlobalStateManager {
    pub health_state: web::Data<health::HealthState>,
    pub quote_state: web::Data<quote::QuotesState>,
}

impl GlobalStateManager {
    pub fn new() -> Self {
        Self {
            health_state: web::Data::new(health::HealthState::new()),
            quote_state: web::Data::new(quote::QuotesState::new()),
        }
    }

    pub fn configure<T, B>(&self, app: App<T, B>) -> App<T, B>
    where
        B: dev::MessageBody,
        T: ServiceFactory<Config = (), Request = dev::ServiceRequest, Response = dev::ServiceResponse<B>, Error = Error, InitError = ()> {
        app
            .app_data(self.health_state.clone())
            .app_data(self.quote_state.clone())
    }
}