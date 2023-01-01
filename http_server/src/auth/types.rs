use domain::entities::user::AuthUser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessTokenClaims {
    pub id: String,
    pub organisation_id: String,
}

impl From<AuthUser> for AccessTokenClaims {
    fn from(user: AuthUser) -> Self {
        AccessTokenClaims {
            id: user.id,
            organisation_id: user.organisation_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefreshTokenClaims {
    pub email: String,
    pub organisation_id: String,
}

impl From<AuthUser> for RefreshTokenClaims {
    fn from(user: AuthUser) -> Self {
        RefreshTokenClaims {
            email: user.email,
            organisation_id: user.organisation_id,
        }
    }
}

#[derive(Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
}
