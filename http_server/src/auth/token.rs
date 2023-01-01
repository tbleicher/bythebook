use domain::entities::user::AuthUser;

use super::types::{AccessTokenClaims, RefreshTokenClaims, TokenPair};

use jwt_simple::prelude::*;

pub fn generate_access_token(
    user: AuthUser,
    jwt_signing_secret: String,
) -> Result<std::string::String, jwt_simple::Error> {
    let key = HS256Key::from_bytes(jwt_signing_secret.as_bytes());
    let custom_claims = AccessTokenClaims::from(user);

    let claims = Claims::with_custom_claims(custom_claims, Duration::from_mins(15));
    key.authenticate(claims)
}

pub fn generate_refresh_token(
    user: AuthUser,
    jwt_signing_secret: String,
) -> Result<std::string::String, jwt_simple::Error> {
    let key = HS256Key::from_bytes(jwt_signing_secret.as_bytes());
    let custom_claims = RefreshTokenClaims::from(user);

    let claims = Claims::with_custom_claims(custom_claims, Duration::from_mins(30));
    key.authenticate(claims)
}

pub fn generate_token_pair(
    user: AuthUser,
    jwt_signing_secret: String,
) -> Result<TokenPair, jwt_simple::Error> {
    let access_token = match generate_access_token(user.clone(), jwt_signing_secret.to_string()) {
        Ok(token) => token,
        Err(err) => {
            return Err(err);
        }
    };

    let refresh_token = match generate_refresh_token(user, jwt_signing_secret) {
        Ok(token) => token,
        Err(err) => {
            return Err(err);
        }
    };

    Ok(TokenPair {
        access_token,
        refresh_token,
    })
}

pub fn validate_access_token(
    token: &str,
    jwt_signing_secret: String,
) -> Result<JWTClaims<AccessTokenClaims>, jwt_simple::Error> {
    let key = HS256Key::from_bytes(jwt_signing_secret.as_bytes());
    key.verify_token::<AccessTokenClaims>(token, None)
}

pub fn validate_refresh_token(
    token: &str,
    jwt_signing_secret: String,
) -> Result<JWTClaims<RefreshTokenClaims>, jwt_simple::Error> {
    let key = HS256Key::from_bytes(jwt_signing_secret.as_bytes());
    key.verify_token::<RefreshTokenClaims>(token, None)
}
