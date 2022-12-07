use crate::db::Database;
use adapter_sql::repositories::{
    NoteRepositorySql, OrganisationRepositorySql, ProjectRepositorySql, UserRepositorySql,
};
use domain::interfaces::{
    NoteRepository, OrganisationRepository, ProjectRepository, RepoProvider, UserRepository,
};

#[derive(Clone, Debug)]
pub struct RepoProviderGraphql {
    pub db: Database,
}

impl RepoProvider for RepoProviderGraphql {
    fn get_note_repo(&self) -> Box<dyn NoteRepository + Send + Sync + '_> {
        Box::new(NoteRepositorySql {
            db: self.db.get_connection(),
        })
    }

    fn get_organisation_repo(&self) -> Box<dyn OrganisationRepository + Send + Sync + '_> {
        Box::new(OrganisationRepositorySql {
            db: self.db.get_connection(),
        })
    }

    fn get_project_repo(&self) -> Box<dyn ProjectRepository + Send + Sync + '_> {
        Box::new(ProjectRepositorySql {
            db: self.db.get_connection(),
        })
    }

    fn get_user_repo(&self) -> Box<dyn UserRepository + Send + Sync + '_> {
        Box::new(UserRepositorySql {
            db: self.db.get_connection(),
        })
    }
}
