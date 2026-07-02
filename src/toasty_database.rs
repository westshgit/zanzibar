use toasty::Db;

pub async fn create_toasty_database() -> anyhow::Result<Db> {
    let psql_url = std::env::var("PSQL_URL")
        .unwrap_or_else(|_| "postgres://postgres:123456789@localhost:5432/postgres".to_string());
    let db = toasty::Db::builder()
        .models(toasty::models!(crate::*))
        .connect(&psql_url)
        .await?;

    Ok(db)
}
