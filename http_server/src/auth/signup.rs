use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, Responder};
use domain::entities::user::NewUserDTO;
use domain::use_cases::UserUseCases;
use graphql_schema::repo_provider::RepoProviderGraphql;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserBody {
    email: String,
    name: String,
}

#[derive(Serialize)]
struct UserResponse {
    email: String,
}

pub async fn signup(
    repo_provider: Data<RepoProviderGraphql>,
    body: Json<CreateUserBody>,
) -> impl Responder {
    let dto = NewUserDTO {
        email: body.email.to_string(),
        name: body.name.to_string(),
        organisation_id: "organisation_id".to_string(), // XXX
    };
    let user_result = UserUseCases::create_user(repo_provider.get_ref(), dto).await;

    match user_result {
        Ok(user) => HttpResponse::Ok().json(UserResponse { email: user.email }),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}
