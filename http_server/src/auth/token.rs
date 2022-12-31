use chrono::{Duration, Utc};
use domain::entities::user::AuthUser;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::error::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessTokenClaims {
    pub id: String,
    pub organisation_id: String,
    pub exp: i64,
}

impl From<AuthUser> for AccessTokenClaims {
    fn from(user: AuthUser) -> Self {
        AccessTokenClaims {
            id: user.id,
            organisation_id: user.organisation_id,
            exp: get_token_timestamp(15),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenClaims {
    pub id: String,
    pub organisation_id: String,
    pub exp: i64,
}

impl From<AuthUser> for RefreshTokenClaims {
    fn from(user: AuthUser) -> Self {
        RefreshTokenClaims {
            id: user.id,
            organisation_id: user.organisation_id,
            exp: get_token_timestamp(15),
        }
    }
}

fn get_token_from_claims(claims: &impl Serialize, secret: String) -> String {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();

    let header = Header {
        ..Default::default()
    };

    let token = Token::new(header, claims)
        .sign_with_key(&jwt_secret)
        .unwrap();
    token.as_str().to_string()
}

fn get_token_timestamp(minutes: i64) -> i64 {
    let mut timer = Utc::now();
    timer = timer + Duration::minutes(minutes);
    timer.timestamp()
}

pub fn renew_token(token: &str, jwt_signing_secret: String) -> Result<String, Box<dyn Error>> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_signing_secret.as_bytes()).unwrap();

    let claims: AccessTokenClaims = match token.verify_with_key(&key) {
        Ok(claims) => claims,
        Err(error) => return Err(Box::new(error)),
    };

    if claims.exp < Utc::now().timestamp() {
        Err("token is expired".into())
    } else {
        let new_claims = AccessTokenClaims {
            exp: get_token_timestamp(15),
            ..claims
        };
        Ok(get_token_from_claims(&new_claims, jwt_signing_secret))
    }
}

pub fn generate_access_token(user: AuthUser, jwt_signing_secret: String) -> String {
    let claims = AccessTokenClaims::from(user);

    get_token_from_claims(&claims, jwt_signing_secret)
}

pub fn generate_refresh_token(user: AuthUser, jwt_signing_secret: String) -> String {
    let claims = RefreshTokenClaims::from(user);

    get_token_from_claims(&claims, jwt_signing_secret)
}
