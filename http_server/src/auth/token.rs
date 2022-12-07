use chrono::{Duration, Utc};
use domain::entities::user::AuthUser;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub id: String,
    pub organisation_id: String,
    pub exp: i64,
}

impl From<AuthUser> for TokenClaims {
    fn from(user: AuthUser) -> Self {
        let mut timer = Utc::now();
        timer = timer + Duration::minutes(15);

        TokenClaims {
            id: user.id,
            organisation_id: user.organisation_id,
            exp: timer.timestamp(),
        }
    }
}

pub fn generate_user_token(user: AuthUser, jwt_signing_secret: String) -> String {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(jwt_signing_secret.as_bytes()).unwrap();

    let header = Header {
        ..Default::default()
    };
    let claims = TokenClaims::from(user);

    let token = Token::new(header, claims)
        .sign_with_key(&jwt_secret)
        .unwrap();
    token.as_str().to_string()
}
