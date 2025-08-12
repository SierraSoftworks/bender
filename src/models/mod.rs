mod health;
mod quote;

use actix::prelude::*;

pub use health::*;
pub use quote::*;

#[derive(Clone)]
pub struct GlobalState {
    pub store: Addr<crate::store::Store>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            store: crate::store::Store::new().start(),
        }
    }
}
