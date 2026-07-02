use anyhow::Context;
use toasty::Db;

use crate::Config;

/// Connect to the database without applying any schema changes.
/// Use this when schema management is handled externally (e.g. via the migration CLI).
pub async fn connect_toasty_database() -> anyhow::Result<Db> {
    let config = Config::from_env()?;

    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(&config.psql_url)
        .await?;

    Ok(db)
}

/// Connect to the database and push the current schema directly.
/// Intended for tests and development only — use migrations in production.
pub async fn create_toasty_database() -> anyhow::Result<Db> {
    let db = connect_toasty_database().await?;
    db.push_schema().await?;
    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_psql_connection() -> anyhow::Result<()> {
        let mut db = create_toasty_database().await?;
        let row = toasty::sql::query("SELECT 1 as one").exec(&mut db).await?;
        println!("Row: {:?}", row);
        Ok(())
    }
}
