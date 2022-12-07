#[derive(Clone, Debug)]
pub struct AppConfig {
    pub db_url: String,
    pub jwt_signing_secret: String,
    pub password_hashing_secret: String,
}

pub fn get_app_config() -> AppConfig {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let jwt_signing_secret =
        std::env::var("JWT_SIGNING_SECRET").expect("JWT_SIGNING_SECRET must be set!");
    let password_hashing_secret =
        std::env::var("PASSWORD_HASHING_SECRET").expect("PASSWORD_HASHING_SECRET must be set!");

    AppConfig {
        db_url,
        jwt_signing_secret,
        password_hashing_secret,
    }
}
