#![allow(non_snake_case)]

use assert_json_diff::assert_json_include;
use serde_json::json;

mod common;
use common::organisation::OrganisationFixture;
use common::project::{
    get_create_project_query, get_delete_project_query, get_get_project_query,
    get_list_projects_query, CreateProjectResponse, DeleteProjectResponse, GetProjectResponse,
    ListProjectsResponse, ProjectFixture,
};
use common::{execute_query, get_test_app_graphql};

#[actix_web::test]
async fn test_create_project() {
    let app = get_test_app_graphql().await;
    let org = OrganisationFixture::new("Test Org", "admin@example.com")
        .execute(&app)
        .await;
    let query = get_create_project_query("Project Title", "the description", &org.id);
    let body_as_string = execute_query(app, query).await;

    let response: CreateProjectResponse = serde_json::from_str(&body_as_string).unwrap();

    let expected = json!({
        "data": {
            "createProject": {
                "title": "Project Title",
                "description": "the description",
                "organisationId": org.id
            }
        }
    });

    assert_json_include!(actual: response, expected: expected);
}

#[actix_web::test]
async fn test_delete_project() {
    let app = get_test_app_graphql().await;
    let org = OrganisationFixture::new("Test Org", "admin@example.com")
        .execute(&app)
        .await;
    let project = ProjectFixture::new(&org.id).execute(&app).await;

    let query = get_delete_project_query(&project.id);
    let response = execute_query(&app, query).await;

    let actual: DeleteProjectResponse = serde_json::from_str(&response).unwrap();
    let expected = json!({
        "data": {
            "deleteProject": {
                "title": "Title",
                "description": "description",
                "organisationId": org.id
            }
        }
    });
    assert_json_include!(actual: actual, expected: expected);
}

#[actix_web::test]
async fn test_get_project_existing() {
    let app = get_test_app_graphql().await;
    let org = OrganisationFixture::new("Test Org", "admin@example.com")
        .execute(&app)
        .await;
    let project = ProjectFixture::new(&org.id).execute(&app).await;

    let query = get_get_project_query(&project.id);
    let body_as_string = execute_query(&app, query).await;
    let get_project_response: GetProjectResponse = serde_json::from_str(&body_as_string).unwrap();

    let expected = json!({
        "data": {
            "project": {
                "title": "Title",
                "description": "description",
                "organisationId": org.id
            }
        }
    });

    assert_json_include!(actual: get_project_response, expected: expected);
}

#[actix_web::test]
async fn test_get_project_not_existing() {
    let app = get_test_app_graphql().await;

    let query = get_get_project_query("abc");
    let body_as_string = execute_query(&app, query).await;
    let get_project_response: GetProjectResponse = serde_json::from_str(&body_as_string).unwrap();

    let expected = json!({
        "data": {
            "project": null
        }
    });

    assert_json_include!(actual: get_project_response, expected: expected);
}

#[actix_web::test]
async fn test_list_projects_empty() {
    let app = get_test_app_graphql().await;

    let query = get_list_projects_query();
    let body_as_string = execute_query(&app, query).await;
    let list_projects_response: ListProjectsResponse =
        serde_json::from_str(&body_as_string).unwrap();

    let expected = json!({
        "data": {
            "projects": []
        }
    });
    assert_json_include!(actual: list_projects_response, expected: expected);
}

#[actix_web::test]
async fn test_list_projects_multiple() {
    let app = get_test_app_graphql().await;
    let org = OrganisationFixture::new("Test Org", "admin@example.com")
        .execute(&app)
        .await;
    let _p1 = ProjectFixture::new(&org.id)
        .set_title("Title 1")
        .execute(&app)
        .await;
    let _p2 = ProjectFixture::new(&org.id)
        .set_title("Title 2")
        .execute(&app)
        .await;

    let query = get_list_projects_query();
    let body_as_string = execute_query(&app, query).await;
    let list_projects_response: ListProjectsResponse =
        serde_json::from_str(&body_as_string).unwrap();

    let expected = json!({
        "data": {
            "projects": [
                {
                    "title": "Title 1",
                    "description": "description",
                    "organisationId": org.id
                },
                {
                    "title": "Title 2",
                    "description": "description",
                    "organisationId": org.id
                }
            ]
        }
    });

    assert_json_include!(actual: list_projects_response, expected: expected);
}
