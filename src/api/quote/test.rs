use super::{configure, models, GlobalState};
use crate::models::*;

#[cfg(test)]
use actix_web::{test, App};

#[actix_rt::test]
async fn quote_v1_not_found_status() {
    let mut app = test::init_service(App::new().data(GlobalState::new()).configure(configure)).await;

    let req = test::TestRequest::with_uri("/api/v1/quote").to_request();
    let response = test::call_service(&mut app, req).await;

    assert!(response.status() == 404);
}

#[actix_rt::test]
async fn quote_v1_found_status() {
    let state = GlobalState::new();
    
    state.store.send(AddQuote{
        who: "Bender".to_string(),
        quote: "Bite my shiny metal ass!".to_string(),
    }).await.expect("The actor should respond").expect("The quote should have been added to the store");

    let mut app = test::init_service(App::new().data(state).configure(configure)).await;

    let req = test::TestRequest::with_uri("/api/v1/quote").to_request();
    let response = test::call_service(&mut app, req).await;

    assert!(response.status().is_success());
}

#[actix_rt::test]
async fn quote_v1_content() {
    let state = GlobalState::new();
    
    state.store.send(AddQuote{
        who: "Bender".to_string(),
        quote: "Bite my shiny metal ass!".to_string(),
    }).await.expect("The actor should respond").expect("The quote should have been added to the store");

    let mut app = test::init_service(App::new().data(state).configure(configure)).await;

    let req = test::TestRequest::with_uri("/api/v1/quote").to_request();
    let response: models::QuoteV1 = test::read_response_json(&mut app, req).await;

    assert_eq!(response.who, "Bender");
    assert_eq!(response.quote, "Bite my shiny metal ass!");
}