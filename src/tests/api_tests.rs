use actix_web::{test, App};
use crate::{index, search};

#[actix_web::test]
async fn test_index_health_check() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["is_healthy"], true);
    assert!(json["message"].as_str().unwrap().contains("Service is healthy"));
}

#[actix_web::test]
async fn test_search_valid_request() {
    let app = test::init_service(App::new().service(search)).await;

    let payload = r#"[
        {
            "length": 10,
            "quantity": 1
        }
    ]"#;

    let req = test::TestRequest::post()
        .uri("/search")
        .set_json(serde_json::from_str::<serde_json::Value>(payload).unwrap())
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let results: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(results.as_array().unwrap().len() > 0);
}

#[actix_web::test]
async fn test_search_invalid_request_empty_vehicles() {
    let app = test::init_service(App::new().service(search)).await;

    let payload = r#"[]"#;

    let req = test::TestRequest::post()
        .uri("/search")
        .set_json(serde_json::from_str::<serde_json::Value>(payload).unwrap())
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 400);

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["error"], "Validation failed");
}

#[actix_web::test]
async fn test_search_invalid_request_zero_length() {
    let app = test::init_service(App::new().service(search)).await;

    let payload = r#"[
        {
            "length": 0,
            "quantity": 1
        }
    ]"#;

    let req = test::TestRequest::post()
        .uri("/search")
        .set_json(serde_json::from_str::<serde_json::Value>(payload).unwrap())
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 400);

    let body = test::read_body(resp).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["error"], "Validation failed");
}

#[actix_web::test]
async fn test_search_invalid_request_too_many_vehicles() {
    let app = test::init_service(App::new().service(search)).await;

    let payload = r#"[
        {
            "length": 10,
            "quantity": 3
        },
        {
            "length": 20,
            "quantity": 3
        }
    ]"#;

    let req = test::TestRequest::post()
        .uri("/search")
        .set_json(serde_json::from_str::<serde_json::Value>(payload).unwrap())
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status().as_u16(), 400);
}
