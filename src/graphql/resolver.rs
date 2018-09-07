use juniper::{graphql_object, FieldResult};

use super::context::Context;
use crate::database::model::{Post as PostModel, User as UserModel};

#[derive(Debug)]
pub struct User(pub UserModel);

graphql_object!(User: () |&self| {
    field username() -> &String {
        &self.0.username
    }

    field email() -> &String {
        &self.0.email
    }
});

#[derive(Debug)]
pub struct Post(pub PostModel);

graphql_object!(Post: () | &self | {
    field title() -> &String {
        &self.0.title
    }

    field body() -> &String {
        &self.0.body
    }

    field published() -> bool {
        self.0.published
    }
});

#[derive(Debug)]
pub struct Query {
    pub(super) _priv: (),
}

graphql_object!(Query: Context |&self| {
    field apiVersion() -> &str { "1.0" }

    field me(&executor) -> FieldResult<Option<User>> {
        executor.context().current_user()
    }

    field posts(&executor) -> FieldResult<Vec<Post>> {
        executor.context().posts()
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

    field createPost(&executor, title: String, body: String) -> FieldResult<Post> {
        executor.context().create_post(title, body)
    }
});
