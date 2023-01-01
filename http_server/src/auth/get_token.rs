use crate::config::AppConfig;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use graphql_schema::repo_provider::RepoProviderGraphql;
use serde::{Deserialize, Serialize};

use super::token::generate_token_pair;
use super::verify_user_password::verify_user_password;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTokenRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
}

pub async fn get_token(
    repo_provider: Data<RepoProviderGraphql>,
    config: Data<AppConfig>,
    item: Json<GetTokenRequest>,
) -> impl Responder {
    let verify_password_result = verify_user_password(
        repo_provider.get_ref(),
        item.email.to_string(),
        item.password.to_string(),
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
