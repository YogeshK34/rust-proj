use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn get_db_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("PG__USER").unwrap(),
        env::var("PG__PASSWORD").unwrap(),
        env::var("PG__HOST").unwrap(),
        env::var("PG__PORT").unwrap(),
        env::var("PG__DBNAME").unwrap()
    );

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}
