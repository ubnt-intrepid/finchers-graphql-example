#![feature(
    async_await,
    await_macro,
    futures_api,
    pin,
    arbitrary_self_types,
    transpose_result,
)]

use failure::Fallible;
use log::info;
use std::env;

use finchers::endpoint::{EndpointExt, SharedEndpoint};
use finchers::{route, routes};

use finchers_graphql_example::database::ConnPool;
use finchers_graphql_example::graphql::create_schema;
use finchers_graphql_example::token::TokenManager;
use finchers_graphql_example::endpoints::{handle_graphql, redirect_to, Config};

fn main() -> Fallible<()> {
    dotenv::dotenv()?;
    pretty_env_logger::try_init()?;

    let config = Config {
        pool: ConnPool::init(env::var("DATABASE_URL")?)?,
        token_manager: TokenManager::new(env::var("JWT_SECRET")?),
        schema: create_schema(),
    };

    let endpoint = routes![
        route!(@get /).and(redirect_to("/graphiql").into_endpoint()),
        route!(@get / "graphiql" /).and(finchers_juniper::graphiql("/query")),
        route!(/ "query" /).and(handle_graphql(config).into_endpoint()),
    ];

    info!("Listening on http://127.0.0.1:4000");
    finchers::launch(endpoint).start("127.0.0.1:4000");
    Ok(())
}
