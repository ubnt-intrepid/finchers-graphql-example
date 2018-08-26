use juniper::{graphql_object, FieldResult};

use super::context::Context;
use crate::database::User as DbUser;

#[derive(Debug)]
pub struct User(pub DbUser);

graphql_object!(User: () |&self| {
    field username() -> &String {
        &self.0.username
    }

    field email() -> &String {
        &self.0.email
    }
});

#[derive(Debug)]
pub struct Query {
    pub(super) _priv: (),
}

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str { "1.0" }

    field me(&executor) -> FieldResult<Option<User>> {
        let token = executor.context().token.as_ref()
            .ok_or_else(|| "You are not authenticated")?;
        executor.context().find_user_by_id(token.user_id())
            .map_err(Into::into)
    }
});

#[derive(Debug)]
pub struct Mutation {
    pub(super) _priv: (),
}

graphql_object!(Mutation: Context | &self | {
    field signup(&executor, username: String, email: String, password: String) -> FieldResult<String> {
        executor.context().signin(username, email, password)
    }

    field login(&executor, email: String, password: String) -> FieldResult<String> {
        executor.context().login(email, password)
    }
});
