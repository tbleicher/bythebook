mod note_repository;
mod organisation_repository;
mod project_repository;
mod user_repository;

pub use note_repository::NoteRepository;
pub use organisation_repository::OrganisationRepository;
pub use project_repository::ProjectRepository;
pub use user_repository::UserRepository;

pub trait RepoProvider {
    fn get_note_repo(&self) -> Box<dyn NoteRepository + Send + Sync + '_>;

    fn get_organisation_repo(&self) -> Box<dyn OrganisationRepository + Send + Sync + '_>;

    fn get_project_repo(&self) -> Box<dyn ProjectRepository + Send + Sync + '_>;

    fn get_user_repo(&self) -> Box<dyn UserRepository + Send + Sync + '_>;
}
