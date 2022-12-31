use crate::config::AppConfig;
use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use graphql_schema::repo_provider::RepoProviderGraphql;

use domain::use_cases::UserUseCases;
use serde::Serialize;

use super::password::verify_password;
use super::token::{generate_access_token, generate_refresh_token};

#[derive(Serialize)]
struct SigninResponse {
    access_token: String,
    refresh_token: String,
}

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
        let access_token =
            generate_access_token(user.clone(), config.jwt_signing_secret.to_string());
        let refresh_token = generate_refresh_token(user, config.jwt_signing_secret.to_string());

        HttpResponse::Ok().json(SigninResponse {
            access_token,
            refresh_token,
        })
    } else {
        HttpResponse::Unauthorized().json("Incorrect username or password")
    }
}
