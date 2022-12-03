use async_graphql::{EmptySubscription, Schema};

use crate::{
    graphql::{mutation::Mutation, query::Query},
    repo_provider::RepoProviderGraphql,
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

/// Builds the GraphQL Schema, attaching the Database to the context
pub async fn build_schema(repo_provider: RepoProviderGraphql) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(repo_provider)
        .finish()
}
