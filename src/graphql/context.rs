use juniper::FieldResult;
use std::sync::Arc;

use crate::database::model::{Post as PostModel, User as UserModel};
use crate::database::Conn;
use crate::token::{Token, TokenManager};

use super::resolver::{Post, User};

/// The context type used in the resolvers defined by Query and Mutation.
pub struct Context {
    pub conn: Conn,
    pub token_manager: Arc<TokenManager>,
    pub token: Option<Token>,
}

impl juniper::Context for Context {}

impl Context {
    fn generate_token(&self, user: UserModel) -> FieldResult<String> {
        self.token_manager
            .generate(user.id, user.email)
            .map_err(Into::into)
    }

    pub fn signin(&self, username: String, email: String, password: String) -> FieldResult<String> {
        let user = UserModel::create(&self.conn, username, email, password)?;
        self.generate_token(user)
    }

    pub fn login(&self, email: String, password: String) -> FieldResult<String> {
        let user = UserModel::find_by_email(&self.conn, email)?.ok_or_else(|| "No such user")?;

        if !user.verify(&password) {
            return Err("incorrect password".into());
        }

        self.generate_token(user)
    }

    pub fn current_user(&self) -> FieldResult<Option<User>> {
        let user_id = self
            .token
            .as_ref()
            .ok_or_else(|| "Not authenticated")?
            .user_id();
        UserModel::find_by_id(&self.conn, user_id)
            .map(|user_opt| user_opt.map(User))
            .map_err(Into::into)
    }

    pub fn create_post(&self, title: String, body: String) -> FieldResult<Post> {
        let user_id = self
            .token
            .as_ref()
            .ok_or_else(|| "Not authenticated")?
            .user_id();
        PostModel::create(&self.conn, user_id, title, body)
            .map(Post)
            .map_err(Into::into)
    }

    pub fn posts(&self) -> FieldResult<Vec<Post>> {
        let user_id = self
            .token
            .as_ref()
            .ok_or_else(|| "Not authenticated")?
            .user_id();
        UserModel::all_posts(&self.conn, user_id)
            .map(|posts| posts.into_iter().map(Post).collect())
            .map_err(Into::into)
    }
}
