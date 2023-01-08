#[derive(Clone, Debug)]
pub struct User {
    pub id: String,
    pub deleted: bool,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub organisation_id: String,
}

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub deleted: bool,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub organisation_id: String,
    pub password_hash: String,
    pub verify_token: String,
}

#[derive(Clone, Debug)]
pub struct NewUserDTO {
    pub email: String,
    pub name: String,
    pub organisation_id: String,
}

#[derive(Clone, Debug)]
pub struct VerifyEmailDTO {
    pub password: String,
    pub token: String,
}

#[derive(Debug, Clone)]
pub struct SessionUser {
    pub id: String,
    pub organisation_id: String,
}
