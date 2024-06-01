#[cfg(test)]
use super::*;
use actix_web::{test, web, App};
use dotenv::dotenv;

#[actix_web::test]
async fn test_start_time_endpoint() {
    dotenv().ok();
    let app = test::init_service(
        App::new().service(
            web::scope("/api/coffee")
                .service(coffee_controller::start_time)
                .service(coffee_controller::set_time)
                .service(coffee_controller::unset_time)
                .service(coffee_controller::toggle_on_off),
        ),
    )
    .await;

    let req = test::TestRequest::get()
        .uri("/api/coffee/start_time")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status() == 200 || resp.status() == 204);
}

#[actix_web::test]
async fn test_set_time_endpoint() {
    dotenv().ok();
    let app = test::init_service(
        App::new().service(
            web::scope("/api/coffee")
                .service(coffee_controller::start_time)
                .service(coffee_controller::set_time)
                .service(coffee_controller::unset_time)
                .service(coffee_controller::toggle_on_off),
        ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/coffee/set_time")
        .insert_header(("Content-Type", "application/json"))
        .set_json(model::coffee::SetTimerPayload {
            time: "10:00".to_string(),
        })
        .to_request();
    let resp = test::call_service(&app, req).await;

    let req2 = test::TestRequest::get()
        .uri("/api/coffee/start_time")
        .to_request();
    let resp2: model::coffee::SetTimerPayload = test::call_and_read_body_json(&app, req2).await;

    assert_eq!(resp.status(), 200);
    assert_eq!(resp2.time, "10:00");
}

#[actix_web::test]
async fn test_unset_time_endpoint() {
    dotenv().ok();
    let app = test::init_service(
        App::new().service(
            web::scope("/api/coffee")
                .service(coffee_controller::start_time)
                .service(coffee_controller::set_time)
                .service(coffee_controller::unset_time)
                .service(coffee_controller::toggle_on_off),
        ),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri("/api/coffee/unset_time")
        .to_request();
    let resp = test::call_service(&app, req).await;

    let req2 = test::TestRequest::get()
        .uri("/api/coffee/start_time")
        .to_request();
    let resp2 = test::call_service(&app, req2).await;

    assert_eq!(resp.status(), 200);
    assert_eq!(resp2.status(), 204);
}

#[actix_web::test]
async fn test_toggle_endpoint() {
    dotenv().ok();
    let app = test::init_service(
        App::new().service(
            web::scope("/api/coffee")
                .service(coffee_controller::start_time)
                .service(coffee_controller::set_time)
                .service(coffee_controller::unset_time)
                .service(coffee_controller::toggle_on_off),
        ),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/api/coffee/toggle_on_off")
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), 200);
}
