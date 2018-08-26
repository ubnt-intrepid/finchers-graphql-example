#![feature(futures_api, pin, arbitrary_self_types)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;

pub mod database;
pub mod graphql;
pub mod token;
