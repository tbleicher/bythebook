#![allow(dead_code)]

use actix_http::Request;
use actix_web::{
    body::to_bytes,
    dev::{Service, ServiceResponse},
    test, web,
    web::Data,
    App, Error,
};
use graphql_schema::graphql::{build_schema, AppSchema};
use graphql_schema::{db::Database, repo_provider::RepoProviderGraphql};
use http_server::auth::signin::signin;
use http_server::index_graphql;
use http_server::{auth::get_token::get_token, config::AppConfig as TestAppConfig};
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};

use super::db::seed_database;

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

pub async fn execute_post_request(
    app: impl Service<Request, Response = ServiceResponse, Error = Error>,
    uri: &str,
    data: impl Serialize,
) -> String {
    let req = test::TestRequest::post()
        .uri(uri)
        .set_json(&data)
        .to_request();

    let resp = test::call_service(&app, req).await;
    let body = to_bytes(resp.into_body()).await.unwrap();
    let body_as_string = std::str::from_utf8(&body).unwrap();

    body_as_string.to_owned()
}

pub async fn get_graphql_schema(config: &TestAppConfig) -> AppSchema {
    let repo_provider = get_repo_provider(config).await;
    build_schema(repo_provider).await
}

pub async fn get_repo_provider(config: &TestAppConfig) -> RepoProviderGraphql {
    let db = Database::new(config.db_url.to_owned()).await;
    Migrator::up(db.get_connection(), None).await.unwrap();

    RepoProviderGraphql { db }
}

pub async fn get_test_app_graphql(
) -> impl Service<Request, Response = ServiceResponse, Error = Error> {
    let config = get_test_config();
    let schema = get_graphql_schema(&config).await;

    let app = test::init_service(
        App::new()
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").to(index_graphql)),
    )
    .await;

    app
}

pub async fn get_test_app_rest() -> impl Service<Request, Response = ServiceResponse, Error = Error>
{
    let config = get_test_config();
    let repo_provider = get_repo_provider(&config.clone()).await;
    seed_database(&repo_provider, config.password_hashing_secret.to_string()).await;

    let app = test::init_service(
        App::new()
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(repo_provider.clone()))
            .service(web::resource("/").to(signin))
            .service(web::resource("/get_token").to(get_token)),
    )
    .await;

    app
}

pub fn get_test_config() -> TestAppConfig {
    TestAppConfig {
        db_url: "sqlite::memory:".to_string(),
        jwt_signing_secret: "secret".to_string(),
        password_hashing_secret: "secret".to_string(),
    }
}
