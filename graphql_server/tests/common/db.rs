use chrono::Utc;
use domain::{
    entities::{
        organisation::Organisation,
        user::{AuthUser, User},
    },
    interfaces::RepoProvider,
};
use graphql_server::auth::password::hash_password;
use nanoid::nanoid;

pub async fn seed_database(repo_provider: &impl RepoProvider, hashing_secret: String) {
    let org = seed_organisation(repo_provider, "ACME".to_string()).await;
    seed_user(
        repo_provider,
        hashing_secret.to_string(),
        "orgadmin@example.com".to_string(),
        org.id.to_string(),
    )
    .await;
    seed_user(
        repo_provider,
        hashing_secret.to_string(),
        "user2@example.com".to_string(),
        org.id.to_string(),
    )
    .await;
}

pub async fn seed_organisation(repo_provider: &impl RepoProvider, name: String) -> Organisation {
    let repo = repo_provider.get_organisation_repo();

    repo.create(Organisation {
        id: nanoid!(10, &nanoid::alphabet::SAFE),
        name: name.to_string(),
        active: false,
        admin_id: "temporary".to_string(),
        created_at: Utc::now(),
        deleted: false,
    })
    .await
    .unwrap()
}

pub async fn seed_user(
    repo_provider: &impl RepoProvider,
    hashing_secret: String,
    email: String,
    organisation_id: String,
) -> User {
    let repo = repo_provider.get_user_repo();

    let user_tmp = AuthUser {
        id: nanoid!(10, &nanoid::alphabet::SAFE),
        deleted: false,
        email: email.to_string(),
        email_verified: true,
        name: email.to_string(),
        organisation_id: organisation_id.to_string(),
        password_hash: hash_password("password", hashing_secret),
        verify_token: "".to_string(),
    };

    repo.create(user_tmp).await.unwrap()
}
