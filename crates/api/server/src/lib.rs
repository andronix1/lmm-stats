#![feature(trait_alias)]
#![feature(async_fn_traits)]
#![feature(async_closure)]

pub mod auth;
pub mod users;
pub mod systems;
pub mod clients;
pub mod stats;

pub mod tokens_pair;

pub extern crate api_common as common;