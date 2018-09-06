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

use finchers::endpoint::{EndpointExt, SendEndpoint};
use finchers::{path, routes};

use finchers_graphql_example::database::ConnPool;
use finchers_graphql_example::endpoints::{handle_graphql, redirect_to, Config};
use finchers_graphql_example::graphql::create_schema;
use finchers_graphql_example::token::TokenManager;

fn main() -> Fallible<()> {
    dotenv::dotenv()?;
    pretty_env_logger::try_init()?;

    let config = Config {
        pool: ConnPool::init(env::var("DATABASE_URL")?)?,
        token_manager: TokenManager::new(env::var("JWT_SECRET")?),
        schema: create_schema(),
    };

    let endpoint = routes![
        path!(@get /).and(redirect_to("/graphiql").into_local()),
        path!(@get / "graphiql" /).and(finchers_juniper::graphiql("/query")),
        path!(/ "query" /).and(handle_graphql(config).into_local()),
    ];

    info!("Listening on http://127.0.0.1:4000");
    finchers::launch(endpoint).start("127.0.0.1:4000");
    Ok(())
}
