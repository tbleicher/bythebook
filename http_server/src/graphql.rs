use actix_web::{web, HttpResponse, Result};

use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use domain::entities::user::SessionUser;
use graphql_schema::graphql::AppSchema;

pub async fn index_graphql(
    user_data: web::ReqData<SessionUser>,
    schema: web::Data<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema
        .execute(req.into_inner().data(user_data.into_inner()))
        .await
        .into()
}

pub async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8000/graphql")
                .finish(),
        ))
}
