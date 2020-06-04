use actix::prelude::*;
use crate::api::APIError;

#[derive(Clone, Serialize, Deserialize)]
pub struct Quote {
    pub quote: String,
    pub who: String,
}

pub struct AddQuote {
    pub quote: String,
    pub who: String,
}

impl Message for AddQuote {
    type Result = Result<(), APIError>;
}

pub struct GetQuote {
    pub who: String,
}

impl Message for GetQuote {
    type Result = Result<Quote, APIError>;
}