//! The definition of GraphQL schema and resolvers.

mod context;
mod resolver;

pub use self::context::Context;
pub use self::resolver::{Mutation, Query};

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query { _priv: () }, Mutation { _priv: () })
}
