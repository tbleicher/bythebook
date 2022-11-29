use actix_http::Request;
use actix_web::{
    body::to_bytes,
    dev::{Service, ServiceResponse},
    test, web,
    web::Data,
    App, Error,
};
use graphql_server::db::Database;
use graphql_server::graphql::schema::{build_schema, AppSchema};
use graphql_server::index_graphql;
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLQuery {
    pub query: String,
}

impl GraphQLQuery {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
        }
    }
}

pub async fn execute_query(
    app: impl Service<Request, Response = ServiceResponse, Error = Error>,
    query_string: String,
) -> String {
    let query = GraphQLQuery::new(&query_string);
    let req = test::TestRequest::post().set_json(&query).to_request();
    let resp = test::call_service(&app, req).await;

    let body = to_bytes(resp.into_body()).await.unwrap();
    let body_as_string = std::str::from_utf8(&body).unwrap();

    body_as_string.to_owned()
}

pub async fn get_test_app() -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let schema = get_graphql_schema().await;
    let app = test::init_service(
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").to(index_graphql)),
    )
    .await;

    app
}

// #[cfg(debug_assertions)]
// use dotenvy::dotenv;

pub async fn get_graphql_schema() -> AppSchema {
    // #[cfg(debug_assertions)]
    // dotenv().ok();

    let db = Database::new("sqlite::memory:".to_owned()).await;
    Migrator::up(db.get_connection(), None).await.unwrap();

    build_schema(db).await
}
