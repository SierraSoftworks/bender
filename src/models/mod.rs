mod stateview;
mod health;
mod quote;

use actix::prelude::*;

pub use quote::*;
pub use health::*;
pub use stateview::*;

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