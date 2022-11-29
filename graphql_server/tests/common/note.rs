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
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub text: String,
    pub project: Project,
    pub organisationId: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateNoteData {
    pub createNote: Note,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CreateNoteResponse {
    pub data: CreateNoteData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DeleteNoteData {
    pub deleteNote: Note,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DeleteNoteResponse {
    pub data: DeleteNoteData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GetNoteData {
    pub note: Option<Note>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GetNoteResponse {
    pub data: GetNoteData,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ListNotesData {
    pub notes: Vec<Note>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ListNotesResponse {
    pub data: ListNotesData,
}

pub struct NoteFixture {
    title: String,
    text: String,
    organisation_id: String,
    project_id: String,
}

impl NoteFixture {
    pub fn new(organisation_id: &str, project_id: &str) -> Self {
        Self {
            title: "Note title".to_string(),
            text: "note text".to_string(),
            organisation_id: organisation_id.to_string(),
            project_id: project_id.to_string(),
        }
    }

    pub async fn execute(
        &self,
        app: &impl Service<Request, Response = ServiceResponse, Error = Error>,
    ) -> Note {
        let query = get_create_note_query(
            &self.title,
            &self.text,
            &self.organisation_id,
            &self.project_id,
        );
        let body_as_string = execute_query(app, query).await;

        let response: CreateNoteResponse = serde_json::from_str(&body_as_string).unwrap();
        let note = response.data.createNote;

        note
    }

    pub fn set_title(&self, title: &str) -> Self {
        Self {
            title: title.to_string(),
            text: self.text.clone(),
            organisation_id: self.organisation_id.clone(),
            project_id: self.project_id.clone(),
        }
    }
}

pub fn get_create_note_query(
    title: &str,
    text: &str,
    organisation_id: &str,
    project_id: &str,
) -> String {
    format!(
        r#"mutation {{ createNote(input: {{ title: {:?}, text: {:?}, organisationId: {:?}, projectId: {:?} }}) {{ id title text project {{ id }} organisationId }} }}"#,
        title, text, organisation_id, project_id
    )
}

pub fn get_delete_note_query(id: &str) -> String {
    format!(
        "mutation {{ deleteNote(id: {:?}) {{ id title text project {{ id }} organisationId }} }}",
        id
    )
}

pub fn get_get_note_query(id: &str) -> String {
    format!(
        "query {{ note(id: {:?}) {{ id title text project {{ id }} organisationId }} }} ",
        id
    )
}

pub fn get_list_notes_query() -> String {
    r#"query { notes { id title text project { id } organisationId } }"#.to_string()
}
