use async_graphql;

pub mod auth;
pub mod note;
pub mod organisation;
pub mod project;

pub use auth::AuthMutation;
pub use note::NotesMutation;
pub use organisation::OrganisationMutation;
pub use project::ProjectsMutation;

// Add your other ones here to create a unified Mutation object
// e.x. Mutation(NoteMutation, OtherMutation, OtherOtherMutation)
#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    AuthMutation,
    OrganisationMutation,
    NotesMutation,
    ProjectsMutation,
);
