#![allow(non_snake_case)]

use assert_json_diff::assert_json_include;
use serde_json::json;

mod common;
use common::organisation::{
    get_create_organisation_query, get_delete_organisation_query, CreateOrganisationResponse,
    DeleteOrganisationResponse, OrganisationFixture,
};
use common::{execute_query, get_test_app};

#[actix_web::test]
async fn test_create_organisation() {
    let app = get_test_app().await;

    let query_string =
        get_create_organisation_query("Example Inc", "admin@example.com", "Admin User");
    let response_string = execute_query(&app, query_string).await;

    let actual: CreateOrganisationResponse = serde_json::from_str(&response_string).unwrap();
    let expected = json!({
        "data": {
            "createOrganisation": {
                "name": "Example Inc",
                "admin": {
                    "email": "admin@example.com",
                    "name": "Admin User",
                }

            }
        }
    });
    assert_json_include!(actual: actual, expected: expected);
}

#[actix_web::test]
async fn test_delete_organisation() {
    let app = get_test_app().await;
    let org = OrganisationFixture::new("ACME", "acme@example.com")
        .execute(&app)
        .await;

    let query = get_delete_organisation_query(&org.id);
    let response_string = execute_query(&app, query).await;

    let actual: DeleteOrganisationResponse = serde_json::from_str(&response_string).unwrap();

    let expected = json!({
        "data": {
            "deleteOrganisation": {
                "name": "ACME"
            }
        }
    });
    assert_json_include!(actual: actual, expected: expected);
}
