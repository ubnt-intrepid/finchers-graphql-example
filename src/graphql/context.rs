use juniper::FieldResult;
use std::sync::Arc;

use super::resolver::User;
use crate::database::{Conn, User as DbUser};
use crate::token::{Token, TokenManager};

/// The context type used in the resolvers defined by Query and Mutation.
pub struct Context {
    pub conn: Conn,
    pub token_manager: Arc<TokenManager>,
    pub token: Option<Token>,
}

impl juniper::Context for Context {}

impl Context {
    fn generate_token(&self, user: DbUser) -> FieldResult<String> {
        self.token_manager
            .generate(user.id, user.email)
            .map_err(Into::into)
    }

    pub fn signin(&self, username: String, email: String, password: String) -> FieldResult<String> {
        let user = DbUser::create(&self.conn, username, email, password)?;
        self.generate_token(user)
    }

    pub fn login(&self, email: String, password: String) -> FieldResult<String> {
        let user = DbUser::find_by_email(&self.conn, email)?.ok_or_else(|| "No such user")?;

        if !user.verify(&password) {
            return Err("incorrect password".into());
        }

        self.generate_token(user)
    }

    pub fn find_user_by_id(&self, id: i32) -> FieldResult<Option<User>> {
        DbUser::find_by_id(&self.conn, id)
            .map(|user_opt| user_opt.map(User))
            .map_err(Into::into)
    }
}
