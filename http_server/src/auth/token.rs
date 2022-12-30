use chrono::{Duration, Utc};
use domain::entities::user::AuthUser;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: String,
    pub organisation_id: String,
    pub exp: i64,
}

impl From<AuthUser> for TokenClaims {
    fn from(user: AuthUser) -> Self {
        TokenClaims {
            id: user.id,
            organisation_id: user.organisation_id,
            exp: get_token_timestamp(),
        }
    }
}

fn get_token_from_claims(claims: TokenClaims, secret: String) -> String {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();

    let header = Header {
        ..Default::default()
    };

    let token = Token::new(header, claims)
        .sign_with_key(&jwt_secret)
        .unwrap();
    token.as_str().to_string()
}

fn get_token_timestamp() -> i64 {
    let mut timer = Utc::now();
    timer = timer + Duration::minutes(15);
    timer.timestamp()
}

pub fn renew_token(token: &str, jwt_signing_secret: String) -> Result<String, Box<dyn Error>> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_signing_secret.as_bytes()).unwrap();

    let claims: TokenClaims = match token.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(error) => return Err(Box::new(error)),
    };

    if claims.exp < Utc::now().timestamp() {
        Err("token is expired".into())
    } else {
        let new_claims = TokenClaims {
            exp: get_token_timestamp(),
            ..claims
        };
        Ok(get_token_from_claims(new_claims, jwt_signing_secret))
    }
}

pub fn generate_user_token(user: AuthUser, jwt_signing_secret: String) -> String {
    let claims = TokenClaims::from(user);

    get_token_from_claims(claims, jwt_signing_secret)
}
