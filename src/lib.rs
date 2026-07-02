#![cfg_attr(debug_assertions, allow(unused_variables, unused_imports, dead_code))]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate typed_builder;

#[macro_use]
extern crate serde;

pub mod toasty_database;

#[derive(Debug, Clone, Serialize, Deserialize, new, TypedBuilder)]
pub struct Config {
    pub psql_url: String,
}
