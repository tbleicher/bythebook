use actix_web::{dev::ServiceRequest, get, web, App, Error, HttpServer, Responder};
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

fn validate_credentials(user_id: &str, user_password: &str) -> Result<bool, std::io::Error> {
    if (user_id.eq("karl") && user_password.eq("password")) {
        return Ok(true);
    }
    return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Authentication failed!",
    ));
}

pub async fn basic_auth_validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);
    match validate_credentials(
        credentials.user_id(),
        credentials.password().unwrap().trim(),
    ) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}
