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
    pub deleted: bool,
    pub email: String,
    pub emailVerified: bool,
    pub name: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Organisation {
    pub id: String,
    pub name: String,
    pub active: bool,
    pub deleted: bool,
    pub admin: User,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateOrganisationData {
    pub createOrganisation: Organisation,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateOrganisationResponse {
    pub data: CreateOrganisationData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DeleteOrganisationData {
    pub deleteOrganisation: Organisation,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DeleteOrganisationResponse {
    pub data: DeleteOrganisationData,
}

pub struct OrganisationFixture {
    name: String,
    admin_email: String,
}

impl OrganisationFixture {
    pub fn new(name: &str, admin_email: &str) -> Self {
        Self {
            name: name.to_string(),
            admin_email: admin_email.to_string(),
        }
    }

    pub async fn execute(
        &self,
        app: &impl Service<Request, Response = ServiceResponse, Error = Error>,
    ) -> Organisation {
        let query = get_create_organisation_query(&self.name, &self.admin_email, "Admin User");
        let body_as_string = execute_query(app, query).await;

        let response: CreateOrganisationResponse = serde_json::from_str(&body_as_string).unwrap();
        let organisation = response.data.createOrganisation;

        organisation
    }
}

pub fn get_create_organisation_query(name: &str, admin_email: &str, admin_name: &str) -> String {
    format!(
        r#"mutation {{ createOrganisation(input: {{ name: {:?}, adminEmail: {:?}, adminName: {:?} }}) {{ id name active deleted admin {{ name email emailVerified deleted }} }} }}"#,
        name, admin_email, admin_name
    )
}

pub fn get_delete_organisation_query(id: &str) -> String {
    format!(
        r#"mutation {{ deleteOrganisation(id: {:?}) {{ id name active deleted admin {{ name email emailVerified deleted }} }} }}"#,
        id
    )
}
