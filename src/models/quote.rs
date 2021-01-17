use actix::prelude::*;
use crate::api::APIError;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Quote {
    pub quote: String,
    pub who: String,
}

#[derive(Debug)]
pub struct AddQuote {
    pub quote: String,
    pub who: String,
}

impl Message for AddQuote {
    type Result = Result<(), APIError>;
}


#[derive(Debug)]
pub struct AddQuotes {
    pub quotes: Vec<Quote>,
}

impl Message for AddQuotes {
    type Result = Result<(), APIError>;
}

#[derive(Debug)]
pub struct GetQuote {
    pub who: String,
}

impl Message for GetQuote {
    type Result = Result<Quote, APIError>;
}