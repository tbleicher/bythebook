pub mod db;
pub mod graphql;
pub mod repo_provider;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::http::GraphiQLSource;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use db::Database;

use graphql::schema::{build_schema, AppSchema};
use migration::{Migrator, MigratorTrait};

#[cfg(debug_assertions)]
use dotenvy::dotenv;

use crate::repo_provider::RepoProviderGraphql;

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

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let db = Database::new(db_url).await;
    Migrator::up(db.get_connection(), None).await.unwrap();

    let repo_provider = RepoProviderGraphql { db };
    let schema = build_schema(repo_provider).await;

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
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
