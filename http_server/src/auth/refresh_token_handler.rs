use crate::config::AppConfig;

use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use domain::use_cases::UserUseCases;
use graphql_schema::repo_provider::RepoProviderGraphql;

use serde::Serialize;

use super::token::{generate_token_pair, validate_refresh_token};

#[derive(Serialize)]
struct RefreshTokenResponse {
    access_token: String,
    refresh_token: String,
}

pub async fn refresh_token_handler(
    auth: BearerAuth,
    config: Data<AppConfig>,
    repo_provider: Data<RepoProviderGraphql>,
) -> impl Responder {
    let validation_result =
        validate_refresh_token(auth.token(), config.jwt_signing_secret.to_string());

    let claims = match validation_result {
        Ok(claims) => claims,
        Err(error) => return HttpResponse::Unauthorized().json(error.to_string()),
    };

    let user_result =
        UserUseCases::get_auth_user(repo_provider.get_ref(), claims.custom.email).await;

    let user = match user_result {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().json("user not found".to_string()),
    };

    let token_pair_result = generate_token_pair(user, config.jwt_signing_secret.to_string());

    match token_pair_result {
        Ok(token_pair) => HttpResponse::Ok().json(token_pair),
        Err(error) => HttpResponse::Unauthorized().json(error.to_string()),
    }
}
