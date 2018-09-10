use std::sync::Arc;

use finchers::endpoint;
use finchers::endpoint::{EndpointExt, SendEndpoint};
use finchers::endpoints::header;
use finchers::error;

use crate::database::ConnPool;
use crate::graphql::Context;
use crate::token::TokenManager;

pub fn fetch_graphql_context(
    pool: ConnPool,
    token_manager: TokenManager,
) -> impl for<'a> SendEndpoint<'a, Output = (Context,)> {
    let acquire_conn = endpoint::unit().and_then(move || {
        let future = pool.acquire_connection();
        async move { await!(future).map_err(Into::into) }
    });

    let parse_token = header::raw("authorization")
        .and(endpoint::value(Arc::new(token_manager)))
        .and_then(
            async move |value: Option<_>, token_manager: Arc<TokenManager>| {
                let token = value
                    .map(|value| token_manager.decode(value))
                    .transpose()
                    .map_err(error::bad_request)?;
                Ok((token, token_manager))
            },
        );

    acquire_conn
        .and(parse_token)
        .map(|conn, (token, token_manager)| Context {
            conn,
            token,
            token_manager,
        })
}
