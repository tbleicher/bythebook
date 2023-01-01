use domain::entities::user::AuthUser;
use domain::errors::RepositoryError;
use graphql_schema::repo_provider::RepoProviderGraphql;

use domain::use_cases::UserUseCases;

use super::password::verify_password;

pub async fn verify_user_password(
    repo_provider: &RepoProviderGraphql,
    user_id: String,
    password: String,
    hashing_secret: String,
) -> Result<AuthUser, RepositoryError> {
    let user_result = UserUseCases::get_auth_user(repo_provider, user_id).await;

    let user = match user_result {
        Ok(user) => user,
        Err(error) => {
            return Err(error);
        }
    };

    let is_valid = verify_password(&password, &user.password_hash, hashing_secret);

    if !is_valid {
        return Err(RepositoryError::new("Incorrect username or password"));
    }

    Ok(user)
}
