use async_graphql::{Object, Result};

#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    async fn health(&self) -> Result<String> {
        Ok("ok".to_owned())
    }
}
