pub mod auth;
pub mod config;
pub mod graphql;

use actix_web::{guard, Resource};
use actix_web::{middleware, web, web::Data, App, HttpServer};
use actix_web_httpauth::extractors::basic::Config;

use graphql_schema::graphql::build_schema;
use migration::{Migrator, MigratorTrait};

#[cfg(debug_assertions)]
use dotenvy::dotenv;

use crate::auth::refresh_token_handler;
use crate::auth::signin::signin;

fn graphiql_resource(path: &str) -> Resource {
    web::resource(path)
        .guard(guard::Get())
        .to(graphql::index_graphiql)
}

fn graphql_resource(path: &str) -> Resource {
    web::resource(path)
        .guard(guard::Post())
        .to(graphql::index_graphql)
}

#[tokio::main]
pub async fn main() -> std::io::Result<()> {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let config = config::get_app_config();

    let db = graphql_schema::db::Database::new(config.db_url.clone()).await;
    Migrator::up(db.get_connection(), None).await.unwrap();

    let repo_provider = graphql_schema::repo_provider::RepoProviderGraphql { db };
    let schema = build_schema(repo_provider.clone()).await;

    println!("GraphiQL IDE: http://localhost:8000/graphql");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::trim())
            .app_data(Config::default().realm("Restricted area"))
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(repo_provider.clone()))
            .app_data(Data::new(schema.clone()))
            .service(
                web::resource("/auth/refresh_token").route(web::post().to(refresh_token_handler)),
            )
            .service(web::resource("/auth/signin").route(web::get().to(signin)))
            .service(graphiql_resource("/graphql"))
            .service(graphql_resource("/graphql"))
    })
    .bind("localhost:8000")?
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
