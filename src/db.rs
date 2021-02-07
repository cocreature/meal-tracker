use futures::stream::BoxStream;
use sqlx::sqlite::SqlitePoolOptions;
use tracing::instrument;

use super::{Config, Meal};

#[instrument]
pub async fn connect(cfg: &Config) -> Result<sqlx::SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&cfg.file)
        .await
}

#[instrument]
pub async fn migrate(pool: &sqlx::SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

#[instrument]
pub async fn list(pool: &sqlx::SqlitePool) -> BoxStream<'_, Result<Meal, sqlx::Error>> {
    sqlx::query_as::<_, Meal>(
        "select title, source, description, rating, entered from meals order by entered desc",
    )
    .fetch(pool)
}

#[instrument]
pub async fn insert(
    pool: &sqlx::SqlitePool,
    meal: &Meal,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        "insert into meals (title, source, description, rating, entered) values (?, ?, ?, ?, ?)",
    )
    .bind(meal.title.clone())
    .bind(meal.source.clone())
    .bind(meal.description.clone())
    .bind(meal.rating)
    .bind(meal.entered)
    .execute(pool)
    .await
}
