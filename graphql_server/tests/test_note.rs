#![allow(non_snake_case)]

use assert_json_diff::assert_json_include;
use serde_json::json;

mod common;
use common::note::{
    get_create_note_query, get_delete_note_query, get_get_note_query, get_list_notes_query,
    CreateNoteResponse, DeleteNoteResponse, GetNoteResponse, ListNotesResponse, NoteFixture,
};
use common::organisation::OrganisationFixture;
use common::project::ProjectFixture;
use common::{execute_query, get_test_app};

#[actix_web::test]
async fn test_create_note() {
    let app = get_test_app().await;
    let org = OrganisationFixture::new("Test Org", "admin@example.com")
        .execute(&app)
        .await;
    let project = ProjectFixture::new(&org.id).execute(&app).await;

    let query = get_create_note_query("Test note", "test description", &org.id, &project.id);
    let body_as_string = execute_query(app, query).await;
    let create_note_response: CreateNoteResponse = serde_json::from_str(&body_as_string).unwrap();

    let expected = json!({
        "data": {
            "createNote": {
                "title": "Test note",
                "text": "test description",
                "project": { "id": project.id },
                "organisationId": org.id
            }
        }
    });

    assert_json_include!(actual: create_note_response, expected: expected);
}

#[actix_web::test]
async fn test_delete_note() {
    let app = get_test_app().await;
    let org = OrganisationFixture::new("Test Org", "admin@example.com")
        .execute(&app)
        .await;
    let project = ProjectFixture::new(&org.id).execute(&app).await;
    let note = NoteFixture::new(&org.id, &project.id).execute(&app).await;

    let query = get_delete_note_query(&note.id);
    let body_as_string = execute_query(app, query).await;
    let delete_note_response: DeleteNoteResponse = serde_json::from_str(&body_as_string).unwrap();

    let expected = json!({
        "data": {
            "deleteNote": {
                "id": note.id
            }
        }
    });
    assert_json_include!(actual: delete_note_response, expected: expected);
}

#[actix_web::test]
async fn test_get_note_existing() {
    let app = get_test_app().await;
    let org = OrganisationFixture::new("Test Org", "admin@example.com")
        .execute(&app)
        .await;
    let project = ProjectFixture::new(&org.id).execute(&app).await;
    let note = NoteFixture::new(&org.id, &project.id).execute(&app).await;

    let query = get_get_note_query(&note.id);
    let body_as_string = execute_query(app, query).await;
    let get_note_response: GetNoteResponse = serde_json::from_str(&body_as_string).unwrap();

    let expected = json!({
        "data": {
            "note": {
                "title": "Note title",
                "text": "note text",
                "organisationId": org.id,
                "project": {
                    "id": project.id
                }
            }
        }
    });

    assert_json_include!(actual: get_note_response, expected: expected);
}

#[actix_web::test]
async fn test_list_notes_empty() {
    let app = get_test_app().await;

    let query = get_list_notes_query();
    let body_as_string = execute_query(app, query).await;
    let list_notes_response: ListNotesResponse = serde_json::from_str(&body_as_string).unwrap();

    let expected = json!({
        "data": {
            "notes": []
        }
    });

    assert_json_include!(actual: list_notes_response, expected: expected);
}

#[actix_web::test]
async fn test_list_notes_multiple() {
    let app = get_test_app().await;
    let org = OrganisationFixture::new("Test Org", "admin@example.com")
        .execute(&app)
        .await;
    let project = ProjectFixture::new(&org.id).execute(&app).await;
    let _note1 = NoteFixture::new(&org.id, &project.id).execute(&app).await;
    let _note2 = NoteFixture::new(&org.id, &project.id)
        .set_title("Second note")
        .execute(&app)
        .await;

    let query = get_list_notes_query();
    let body_as_string = execute_query(app, query).await;
    let list_notes_response: ListNotesResponse = serde_json::from_str(&body_as_string).unwrap();

    let expected = json!({
        "data": {
            "notes": [
                {"title": "Note title"},
                {"title": "Second note"}
            ]
        }
    });

    assert_json_include!(actual: list_notes_response, expected: expected);
    assert_eq!(list_notes_response.data.notes.len(), 2);
}
