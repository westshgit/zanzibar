#![allow(unused_variables, dead_code, unused_imports)]
use toasty::Db;
use toasty_cli::{Config, MigrationConfig, MigrationPrefixStyle::Sequential, ToastyCli};
use zanzibar::toasty_database::connect_toasty_database;

#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate typed_builder;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate toasty;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let migration_config = MigrationConfig::new().path("./psql-toasty").prefix_style(Sequential);
    let config = Config::new().migration(migration_config);
    let db = connect_toasty_database().await?;

    let cli = ToastyCli::with_config(db, config);
    cli.parse_and_run().await?;
    Ok(())
}
