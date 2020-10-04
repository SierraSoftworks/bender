mod stateview;
mod health;
mod quote;
mod trace_message;

use actix::prelude::*;

pub use quote::*;
pub use health::*;
pub use stateview::*;
pub use trace_message::*;

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