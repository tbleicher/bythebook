use crate::auth::token::renew_token;
use crate::config::AppConfig;

use actix_web::web::Data;
use actix_web::{HttpResponse, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use serde::Serialize;

#[derive(Serialize)]
struct RefreshTokenResponse {
    token: String,
}

pub async fn refresh_token(auth: BearerAuth, config: Data<AppConfig>) -> impl Responder {
    let renew_result = renew_token(auth.token(), config.jwt_signing_secret.to_string());

    match renew_result {
        Ok(token) => HttpResponse::Ok().json(RefreshTokenResponse { token }),
        Err(error) => HttpResponse::Unauthorized().json(error.to_string()),
    }
}
