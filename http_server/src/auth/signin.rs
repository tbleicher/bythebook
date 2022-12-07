use crate::auth::token::generate_user_token;
use crate::config::AppConfig;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use graphql_schema::repo_provider::RepoProviderGraphql;

use domain::use_cases::UserUseCases;

use super::password::verify_password;

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

    let user_result =
        UserUseCases::get_auth_user(repo_provider.get_ref(), credentials.user_id().to_string())
            .await;

    let user = match user_result {
        Ok(user) => user,
        Err(error) => return HttpResponse::InternalServerError().json(format!("{:?}", error)),
    };

    let is_valid = verify_password(
        &password,
        &user.password_hash,
        config.password_hashing_secret.to_string(),
    );

    if is_valid {
        let token_str = generate_user_token(user, config.jwt_signing_secret.to_string());
        HttpResponse::Ok().json(token_str)
    } else {
        HttpResponse::Unauthorized().json("Incorrect username or password")
    }
}
