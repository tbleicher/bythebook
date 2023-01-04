use actix_web::{body::to_bytes, http::header::ContentType, test, web, web::Data, App};
use graphql_schema::graphql::AppSchema;
use http_server::graphql::index_graphiql;

use crate::common::{execute_query, get_graphql_schema, get_test_app_graphql, get_test_config};
mod common;

#[actix_web::test]
async fn test_graphiql_route() {
    let config = get_test_config();
    let schema: AppSchema = get_graphql_schema(&config).await;
    let app = test::init_service(
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").to(index_graphiql)),
    )
    .await;

    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body = to_bytes(resp.into_body()).await.unwrap();
    let body_as_string = std::str::from_utf8(&body).unwrap();
    assert!(String::from(body_as_string).contains("<title>GraphiQL IDE</title>"));
}

#[actix_web::test]
async fn test_graphql_route() {
    let app = get_test_app_graphql().await;

    let body_as_string = execute_query(&app, "query { health }".to_string()).await;

    assert_eq!(body_as_string, "{\"data\":{\"health\":\"ok\"}}");
}
