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

use finchers::endpoints::logging::logging;
use finchers::prelude::*;
use finchers::{path, routes};

use finchers_graphql_example::database::ConnPool;
use finchers_graphql_example::endpoints::fetch_graphql_context;
use finchers_graphql_example::graphql::create_schema;
use finchers_graphql_example::token::TokenManager;

fn main() -> Fallible<()> {
    dotenv::dotenv()?;
    pretty_env_logger::try_init()?;

    let pool = ConnPool::init(env::var("DATABASE_URL")?)?;
    let token_manager = TokenManager::new(env::var("JWT_SECRET")?);
    let fetch_graphql_context = fetch_graphql_context(pool, token_manager);

    let endpoint = routes![
        path!(@get /).and(finchers_juniper::graphiql("/graphql")),
        path!(/ "graphql" /)
            .and(fetch_graphql_context)
            .wrap(finchers_juniper::execute(create_schema())),
    ];

    let endpoint = endpoint.wrap(logging());

    info!("Listening on http://127.0.0.1:4000");
    finchers::launch(endpoint).start("127.0.0.1:4000");
    Ok(())
}
