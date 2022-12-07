use crate::auth::token::generate_user_token;
use crate::config::AppConfig;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use domain::use_cases::UserUseCases;
use graphql_schema::repo_provider::RepoProviderGraphql;
use serde::{Deserialize, Serialize};

use super::password::verify_password;

#[derive(Debug, Deserialize, Serialize)]
pub struct GetTokenRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenResponse {
    token: String,
}

pub async fn get_token(
    repo_provider: Data<RepoProviderGraphql>,
    config: Data<AppConfig>,
    item: Json<GetTokenRequest>,
) -> impl Responder {
    let user_result =
        UserUseCases::get_auth_user(repo_provider.get_ref(), item.email.to_string()).await;

    let user = match user_result {
        Ok(user) => user,
        Err(error) => return HttpResponse::InternalServerError().json(format!("{:?}", error)),
    };

    let is_valid = verify_password(
        &item.password,
        &user.password_hash,
        config.password_hashing_secret.to_string(),
    );

    if is_valid {
        let token = generate_user_token(user, config.jwt_signing_secret.to_string());
        HttpResponse::Ok().json(TokenResponse { token })
    } else {
        HttpResponse::Unauthorized().json("Incorrect username or password")
    }
}
