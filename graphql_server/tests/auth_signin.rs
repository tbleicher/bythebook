use graphql_server::auth::get_token::GetTokenRequest;
use serde::{Deserialize, Serialize};
mod common;
use common::get_test_app_rest;

use crate::common::execute_post_request;

#[derive(Deserialize, Serialize)]
struct TokenResponse {
    token: String,
}

#[actix_web::test]
async fn test_get_user_token() {
    let app = get_test_app_rest().await;
    let request = GetTokenRequest {
        email: "user2@example.com".to_string(),
        password: "password".to_string(),
    };
    let response_string = execute_post_request(&app, "/get_token", request).await;

    let actual: TokenResponse = serde_json::from_str(&response_string).unwrap();
    assert!(actual.token.starts_with("ey"));
}
