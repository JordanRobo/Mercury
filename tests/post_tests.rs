use actix_web::{test, web, App};
use mercury::{config::admin_api, db, posts::models::Post};

#[actix_web::test]
async fn test_get_posts() {
    // Set up the database connection
    let db = db::establish_connection();
    let app_data = web::Data::new(db);

    // Create the app with the same configuration as in main.rs
    let app = test::init_service(App::new().app_data(app_data.clone()).configure(admin_api)).await;

    // Send a request to the /posts endpoint
    let req = test::TestRequest::get().uri("/api/posts").to_request();
    let resp = test::call_service(&app, req).await;

    // Assert that the response is successful
    assert!(resp.status().is_success());

    // Read and parse the response body
    let posts: Vec<Post> = test::read_body_json(resp).await;
    assert!(!posts.is_empty());
}
