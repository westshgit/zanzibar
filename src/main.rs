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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Ok(())
}
