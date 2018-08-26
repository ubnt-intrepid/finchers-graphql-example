use failure::{format_err, Fallible};
use jsonwebtoken;
use jsonwebtoken::{Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::str;
use time;
use time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: i32,
    email: String,
    iat: i64,
    exp: i64,
    nbf: i64,
}

#[derive(Debug)]
pub struct Token(TokenData<Claims>);

impl Token {
    pub fn user_id(&self) -> i32 {
        self.0.claims.user_id
    }
}

#[derive(Debug)]
pub struct TokenManager {
    secret_key: String,
    header: Header,
    validation: Validation,
}

impl TokenManager {
    pub fn new(secret_key: String) -> TokenManager {
        TokenManager {
            secret_key,
            header: Default::default(),
            validation: Validation {
                leeway: 60,
                ..Default::default()
            },
        }
    }

    pub fn decode(&self, value: impl AsRef<[u8]>) -> Fallible<Token> {
        let value = value.as_ref();
        match value.get(0..7) {
            Some(b"bearer ") | Some(b"Bearer ") => {
                let token = str::from_utf8(&value[7..])?;
                let data = jsonwebtoken::decode(token, self.secret_key.as_ref(), &self.validation)?;
                Ok(Token(data))
            }
            Some(..) => Err(format_err!("")),
            None => Err(format_err!("")),
        }
    }

    pub fn generate(&self, user_id: i32, email: String) -> Fallible<String> {
        let now = time::now_utc().to_timespec();
        let claims = Claims {
            user_id,
            email,
            iat: now.sec,
            exp: (now + Duration::days(1)).sec,
            nbf: now.sec,
        };
        jsonwebtoken::encode(&self.header, &claims, self.secret_key.as_bytes()).map_err(Into::into)
    }
}
