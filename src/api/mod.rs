pub mod health;
pub mod quote;

pub use quote::{Quote, QuotesState};

pub trait StateView<T> {
    fn from_state(state: &T) -> Self;
    fn to_state(&self) -> T;
}