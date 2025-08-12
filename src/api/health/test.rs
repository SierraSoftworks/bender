use super::{configure, models};
use crate::models::*;

use actix_web::web::Data;
#[cfg(test)]
use actix_web::{test, App};

#[actix_rt::test]
async fn health_v1_status() {
    let app = test::init_service(
        App::new()
            .app_data(Data::new(GlobalState::new()))
            .configure(configure),
    )
    .await;

    let req = test::TestRequest::with_uri("/api/v1/health").to_request();
    let response = test::call_service(&app, req).await;

    assert!(response.status().is_success());
}

#[actix_rt::test]
async fn health_v1_content() {
    let state = GlobalState::new();

    let app = test::init_service(App::new().app_data(Data::new(state)).configure(configure)).await;

    let req = test::TestRequest::with_uri("/api/v1/health").to_request();
    let response: models::HealthV1 = test::call_and_read_body_json(&app, req).await;

    assert!(response.ok);
}

#[actix_rt::test]
async fn health_v2_status() {
    let app = test::init_service(
        App::new()
            .app_data(Data::new(GlobalState::new()))
            .configure(configure),
    )
    .await;

    let req = test::TestRequest::with_uri("/api/v2/health").to_request();
    let response = test::call_service(&app, req).await;

    assert!(response.status().is_success());
}

#[actix_rt::test]
async fn health_v2_content() {
    let state = GlobalState::new();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(state.clone()))
            .configure(configure),
    )
    .await;

    let req = test::TestRequest::with_uri("/api/v2/health").to_request();
    let response: models::HealthV2 = test::call_and_read_body_json(&app, req).await;

    assert!(response.ok);
    assert_eq!(
        response.started_at,
        state
            .store
            .send(GetHealth {})
            .await
            .unwrap()
            .unwrap()
            .started_at
    );
}
