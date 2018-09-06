#![feature(
    async_await,
    await_macro,
    futures_api,
    pin,
    arbitrary_self_types,
    transpose_result
)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;

pub mod database;
pub mod endpoints;
pub mod graphql;
pub mod token;
