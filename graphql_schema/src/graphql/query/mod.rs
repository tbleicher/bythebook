use async_graphql;

mod health;
pub mod note;
pub mod project;

use health::HealthQuery;
pub use note::NotesQuery;
pub use project::ProjectsQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(NoteQuery, OtherQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(HealthQuery, NotesQuery, ProjectsQuery);
