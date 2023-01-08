use std::{
    future::{ready, Ready},
    task::{Context, Poll},
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use domain::entities::user::SessionUser;

use crate::auth::token::validate_access_token;

#[derive(Debug, Clone)]
pub struct Msg(pub String);

#[doc(hidden)]
pub struct AddSessionUserService<S> {
    service: S,
    jwt_signing_secret: String,
}

impl<S, B> Service<ServiceRequest> for AddSessionUserService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let session_user = get_session_user(&req, self.jwt_signing_secret.clone());

        match session_user {
            Some(user) => {
                req.extensions_mut().insert(user);
            }
            _ => {}
        };

        self.service.call(req)
    }
}

#[derive(Clone, Debug)]
pub struct AddSessionUser {
    jwt_signing_secret: String,
}

impl AddSessionUser {
    pub fn new(secret: String) -> Self {
        Self {
            jwt_signing_secret: secret,
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AddSessionUser
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    type Transform = AddSessionUserService<S>;
    type InitError = ();

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AddSessionUserService {
            service,
            jwt_signing_secret: self.jwt_signing_secret.clone(),
        }))
    }
}

fn get_session_user(req: &ServiceRequest, secret: String) -> Option<SessionUser> {
    let header = req.headers().get(actix_web::http::header::AUTHORIZATION);
    let header_value = match header {
        Some(header_value) => match header_value.to_str() {
            Ok(token_string) => token_string,
            Err(_) => "",
        },
        None => "",
    };

    let (_, token) = header_value.split_at(7); // strip 'Bearer ' prefix

    match validate_access_token(token, secret) {
        Ok(claims) => Some(SessionUser {
            id: claims.custom.id,
            organisation_id: claims.custom.organisation_id,
        }),
        Err(_) => None,
    }
}
