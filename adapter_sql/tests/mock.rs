mod prepare;

use adapter_sql::note;
use domain::{Mutation, NoteUseCases};
use prepare::prepare_mock_db;

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();

    {
        let note = NoteUseCases::find_note_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(note.id, 1);
    }

    {
        let note = NoteUseCases::find_note_by_id(db, 5).await.unwrap().unwrap();

        assert_eq!(note.id, 5);
    }

    {
        let note = Mutation::create_note(
            db,
            note::Model {
                id: 0,
                title: "Title D".to_owned(),
                text: "Text D".to_owned(),
                organisation_id: "organisation-id".to_owned(),
                project_id: "project-id".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            note,
            note::Model {
                id: 6,
                title: "Title D".to_owned(),
                text: "Text D".to_owned(),
                organisation_id: "organisation-id".to_owned(),
                project_id: "project-id".to_owned(),
            }
        );
    }

    {
        let note = Mutation::update_note_by_id(
            db,
            1,
            note::Model {
                id: 1,
                title: "New Title A".to_owned(),
                text: "New Text A".to_owned(),
                organisation_id: "organisation-id".to_owned(),
                project_id: "project-id".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            note,
            note::Model {
                id: 1,
                title: "New Title A".to_owned(),
                text: "New Text A".to_owned(),
                organisation_id: "organisation-id".to_owned(),
                project_id: "project-id".to_owned(),
            }
        );
    }

    {
        let result = Mutation::delete_note(db, 5).await.unwrap();

        assert_eq!(result.rows_affected, 1);
    }

    {
        let result = Mutation::delete_all_notes(db).await.unwrap();

        assert_eq!(result.rows_affected, 5);
    }
}
