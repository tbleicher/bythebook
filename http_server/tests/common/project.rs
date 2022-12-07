#![allow(non_snake_case)]
#![allow(dead_code)]

use actix_http::Request;
use actix_web::{
    dev::{Service, ServiceResponse},
    Error,
};

use serde::{Deserialize, Serialize};

use super::execute_query;

#[derive(Deserialize, Debug, Serialize)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub organisationId: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateProjectData {
    pub createProject: Project,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateProjectResponse {
    pub data: CreateProjectData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DeleteProjectData {
    pub deleteProject: Project,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DeleteProjectResponse {
    pub data: DeleteProjectData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GetProjectData {
    pub project: Option<Project>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GetProjectResponse {
    pub data: GetProjectData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ListProjectsResponse {
    pub data: ListProjectsData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ListProjectsData {
    pub projects: Vec<Project>,
}

pub struct ProjectFixture {
    title: String,
    description: String,
    organisation_id: String,
}

impl ProjectFixture {
    pub fn new(organisation_id: &str) -> Self {
        Self {
            title: "Title".to_string(),
            description: "description".to_string(),
            organisation_id: organisation_id.to_string(),
        }
    }

    pub async fn execute(
        &self,
        app: &impl Service<Request, Response = ServiceResponse, Error = Error>,
    ) -> Project {
        let query = get_create_project_query(&self.title, &self.description, &self.organisation_id);
        let body_as_string = execute_query(app, query).await;

        let response: CreateProjectResponse = serde_json::from_str(&body_as_string).unwrap();
        let project = response.data.createProject;

        project
    }

    pub fn set_title(&self, title: &str) -> Self {
        Self {
            title: title.to_string(),
            description: self.description.clone(),
            organisation_id: self.organisation_id.clone(),
        }
    }
}

pub fn get_create_project_query(title: &str, description: &str, organisation_id: &str) -> String {
    format!(
        r#"mutation {{ createProject(input: {{ title: {:?}, description: {:?}, organisationId: {:?} }}) {{ id title description organisationId }} }}"#,
        title, description, organisation_id
    )
}

pub fn get_delete_project_query(id: &str) -> String {
    format!(
        "mutation {{ deleteProject(id: {:?}) {{ id title description organisationId }} }}",
        id
    )
}

pub fn get_get_project_query(id: &str) -> String {
    format!(
        "query {{ project(id: {:?}) {{ id title description organisationId }} }} ",
        id
    )
}

pub fn get_list_projects_query() -> String {
    r#"query { projects { id title description organisationId } }"#.to_string()
}
