pub mod auth;
pub mod config;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use actix_web_httpauth::extractors::basic::Config;
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use graphql_schema::graphql::{build_schema, AppSchema};
use migration::{Migrator, MigratorTrait};

#[cfg(debug_assertions)]
use dotenvy::dotenv;

use crate::auth::refresh_token_handler;
use crate::auth::signin::signin;

pub async fn index_graphql(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(
            GraphiQLSource::build()
                .endpoint("http://localhost:8000")
                .finish(),
        ))
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let config = config::get_app_config();

    // let db_url = std::env::var("DATABASE_URL").unwrap();
    let db = graphql_schema::db::Database::new(config.db_url.clone()).await;
    Migrator::up(db.get_connection(), None).await.unwrap();

    let repo_provider = graphql_schema::repo_provider::RepoProviderGraphql { db };
    let schema = build_schema(repo_provider.clone()).await;

    println!("GraphiQL IDE: http://localhost:8000/");

    HttpServer::new(move || {
        App::new()
            .app_data(Config::default().realm("Restricted area"))
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(repo_provider.clone()))
            .app_data(Data::new(schema.clone()))
            .service(
                web::resource("/auth/refresh_token").route(web::post().to(refresh_token_handler)),
            )
            .service(
                web::resource("/auth/refresh_token/").route(web::post().to(refresh_token_handler)),
            )
            .service(web::resource("/auth/signin").route(web::get().to(signin)))
            .service(web::resource("/auth/signin/").route(web::get().to(signin)))
            .service(web::resource("/").guard(guard::Post()).to(index_graphql))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
