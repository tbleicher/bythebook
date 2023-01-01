use crate::config::AppConfig;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use graphql_schema::repo_provider::RepoProviderGraphql;

use super::token::generate_token_pair;
use super::verify_user_password::verify_user_password;

pub async fn signin(
    repo_provider: Data<RepoProviderGraphql>,
    config: Data<AppConfig>,
    credentials: BasicAuth,
) -> impl Responder {
    let pw_option = credentials.password();
    let password = match pw_option {
        None => return HttpResponse::Unauthorized().json("Must provide username and password"),
        Some(password) => password,
    };

    let verify_password_result = verify_user_password(
        repo_provider.get_ref(),
        credentials.user_id().to_string(),
        password.to_string(),
        config.password_hashing_secret.to_string(),
    )
    .await;

    let token_pair_result = match verify_password_result {
        Ok(user) => generate_token_pair(user, config.jwt_signing_secret.to_string()),
        Err(error) => return HttpResponse::Unauthorized().json(format!("{:?}", error)),
    };

    match token_pair_result {
        Ok(token_pair) => HttpResponse::Ok().json(token_pair),
        Err(error) => HttpResponse::Unauthorized().json(error.to_string()),
    }
}
