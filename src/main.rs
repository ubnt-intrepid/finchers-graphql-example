#![feature(
    async_await,
    await_macro,
    futures_api,
    pin,
    arbitrary_self_types,
    transpose_result,
)]

use finchers::endpoint;
use finchers::endpoint::EndpointExt;
use finchers::endpoints;
use finchers::error;
use finchers::output::payload::Empty;
use finchers::{route, routes};
use finchers_juniper::GraphQLRequest;

use failure::Fallible;
use futures::future;
use futures::future::TryFutureExt;
use http::{Response, StatusCode};
use log::info;
use std::env;
use std::sync::Arc;

use finchers_graphql_example::database::ConnPool;
use finchers_graphql_example::graphql::{create_schema, Context};
use finchers_graphql_example::token::TokenManager;

fn main() -> Fallible<()> {
    dotenv::dotenv()?;
    pretty_env_logger::try_init()?;

    let pool = ConnPool::init(env::var("DATABASE_URL")?)?;

    // Variables for encoding/decoding JSON Web Tokens.
    let token_manager = Arc::new(TokenManager::new(env::var("JWT_SECRET")?));

    let fetch_graphql_context = {
        let acquire_conn = endpoint::unit().and_then(move || pool.acquire_connection().err_into());

        let parse_token = endpoints::header::raw("authorization").and_then({
            let token_manager = token_manager.clone();
            move |value: Option<_>| {
                future::ready(
                    value
                        .map(|value| token_manager.decode(value))
                        .transpose()
                        .map_err(error::bad_request),
                )
            }
        });

        acquire_conn
            .and(parse_token)
            .map(move |conn, token| Context {
                conn,
                token,
                token_manager: token_manager.clone(),
            })
    };

    let handle_graphql_queries = finchers_juniper::request()
        .and(endpoint::value(Arc::new(create_schema())))
        .and(fetch_graphql_context)
        .and_then(|request: GraphQLRequest, schema, context| {
            request.execute_async(schema, context)
        });

    let endpoint = routes![
        route!(@get /).map(|| redirect_to("/graphiql")),
        route!(@get / "graphiql" /).and(finchers_juniper::graphiql("/query")),
        route!(/ "query" /).and(handle_graphql_queries),
    ];

    info!("Listening on http://127.0.0.1:4000");
    finchers::launch(endpoint).start("127.0.0.1:4000");
    Ok(())
}

fn redirect_to(url: impl AsRef<str>) -> Response<Empty> {
    Response::builder()
        .status(StatusCode::MOVED_PERMANENTLY)
        .header("location", url.as_ref())
        .body(Empty)
        .expect("valid response")
}
