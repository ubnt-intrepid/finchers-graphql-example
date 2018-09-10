#![feature(async_await, await_macro, futures_api)]

mod graphql;

use finchers::endpoint;
use finchers::endpoint::EndpointExt;
use finchers::path;

use crate::graphql::{create_schema, Context};

fn main() {
    // GraphQL クエリを処理する際に用いるコンテキスト値を返すエンドポイント
    // 今回はダミーの値を返しておく
    let fetch_context = endpoint::unit().map(|| Context {});

    // GraphQL リクエストを処理するエンドポイント
    let graphql = path!(/ "graphql" /)
        .and(fetch_context)
        .with(finchers_juniper::execute(create_schema()));

    // GraphiQL (GraphQL 用の Web IDE) を返すエンドポイント
    let graphiql = path!(@get /).and(finchers_juniper::graphiql("/graphql"));

    // 2つのルートのいずれかにマッチするエンドポイント
    let endpoint = graphiql.or(graphql);

    println!("Listening on http://127.0.0.1:4000");
    finchers::launch(endpoint).start("127.0.0.1:4000");
}
