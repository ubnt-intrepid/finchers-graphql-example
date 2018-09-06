use finchers::endpoint;
use finchers::endpoint::{EndpointExt, SendEndpoint};
use finchers::endpoints::header;
use finchers::error;
use finchers::output::payload::Empty;
use finchers_juniper::{GraphQLRequest, GraphQLResponse};

use http::{Response, StatusCode};
use std::sync::Arc;

use crate::database::ConnPool;
use crate::graphql::{Context, Schema};
use crate::token::TokenManager;

/// Creates an endpoint which always returns an HTTP response redirecting to the specified URI.
pub fn redirect_to(
    uri: impl AsRef<str>,
) -> impl for<'a> SendEndpoint<'a, Output = (Response<Empty>,)> {
    let uri = uri.as_ref().to_owned();
    endpoint::unit().map(move || {
        Response::builder()
            .status(StatusCode::MOVED_PERMANENTLY)
            .header("location", &*uri)
            .body(Empty)
            .expect("valid response")
    })
}

pub struct Config {
    pub pool: ConnPool,
    pub token_manager: TokenManager,
    pub schema: Schema,
}

pub fn handle_graphql(
    config: Config,
) -> impl for<'a> SendEndpoint<'a, Output = (GraphQLResponse,)> {
    let Config {
        pool,
        token_manager,
        schema,
    } = config;

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

    let fetch_graphql_context =
        acquire_conn
            .and(parse_token)
            .map(|conn, (token, token_manager)| Context {
                conn,
                token,
                token_manager,
            });

    finchers_juniper::request()
        .and(endpoint::value(Arc::new(schema)))
        .and(fetch_graphql_context)
        .and_then(|request: GraphQLRequest, schema, context| request.execute_async(schema, context))
}
