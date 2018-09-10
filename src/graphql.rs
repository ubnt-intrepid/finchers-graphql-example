//! GraphQL のスキーマ定義

use juniper::{graphql_object, EmptyMutation, RootNode};

pub struct Context {}

impl juniper::Context for Context {}

pub struct Query {
    _p: (),
}

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str {
        "1.0"
    }
});

pub type Mutation = EmptyMutation<Context>;

pub type Schema = RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query { _p: () }, Mutation::new())
}
