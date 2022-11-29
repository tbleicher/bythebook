#![allow(dead_code)]
#![allow(non_snake_case)]

use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    Error,
};
use serde::{Deserialize, Serialize};

use super::execute_query;

#[derive(Deserialize, Debug, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateUserData {
    pub createUser: User,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateUserResponse {
    pub data: CreateUserData,
}

pub async fn create_user(
    app: impl Service<Request, Response = ServiceResponse, Error = Error>,
    email: &str,
    organisation_id: &str,
) -> String {
    let query = get_create_user_query("Test User", email, organisation_id);

    execute_query(&app, query).await
}

pub fn get_create_user_query(name: &str, email: &str, organisation_id: &str) -> String {
    format!(
        r#"mutation {{ createUser(input: {{ email: {:?}, name: {:?}, organisationId: {:?} }}) {{ id email name }} }}"#,
        email, name, organisation_id
    )
}
