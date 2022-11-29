use assert_json_diff::assert_json_include;
use serde_json::json;

mod common;
use common::organisation::OrganisationFixture;
use common::user::{get_create_user_query, CreateUserResponse};
use common::{execute_query, get_test_app};

#[actix_web::test]
async fn test_create_user() {
    let app = get_test_app().await;
    let org = OrganisationFixture::new("Test Org", "admin@example.com")
        .execute(&app)
        .await;

    let query_string = get_create_user_query("Test User", "test.user@example.com", &org.id);
    let response_string = execute_query(&app, query_string).await;

    let actual: CreateUserResponse = serde_json::from_str(&response_string).unwrap();
    let expected = json!({
        "data": {
            "createUser": {
                "email": "test.user@example.com",
                "name": "Test User",
            }
        }
    });
    assert_json_include!(actual: actual, expected: expected);
}

#[actix_web::test]
async fn test_create_user_unique_email() {
    let app = get_test_app().await;
    let org = OrganisationFixture::new("Test Org", "admin@example.com")
        .execute(&app)
        .await;

    let query_string = get_create_user_query("Test User", "test.user@example.com", &org.id);
    let _response_1 = execute_query(&app, query_string.clone()).await;
    let response_string = execute_query(&app, query_string).await;

    assert!(String::from(response_string).contains("UNIQUE constraint failed: users.email"));
}
