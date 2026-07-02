#![cfg_attr(debug_assertions, allow(unused_variables, unused_imports, dead_code))]
#![allow(non_camel_case_types)]

use anyhow::Context as _;

#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate typed_builder;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate toasty;

pub mod models;
pub mod toasty_database;
pub mod zanzibar;

#[derive(Debug, Clone, Serialize, Deserialize, new, TypedBuilder)]
pub struct Config {
    pub psql_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();
        let psql_url = std::env::var("PSQL_URL").with_context(|| "Failed to read environment variable PSQL_URL")?;
        Ok(Self { psql_url })
    }
}
