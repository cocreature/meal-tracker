use chrono::prelude::*;
use futures::stream::TryStreamExt;

use cooking::{db, Command, Config, List, Meal};

async fn setup() -> Result<sqlx::sqlite::SqlitePool, sqlx::Error> {
    let config = Config {
        file: "sqlite::memory:".to_string(),
        cmd: Command::List(List {}),
    };
    db::connect(&config).await
}

#[async_std::test]
async fn db_migrations() -> Result<(), sqlx::Error> {
    let pool = setup().await?;
    db::migrate(&pool).await?;
    Ok(())
}

#[async_std::test]
async fn db_insert_list() -> Result<(), sqlx::Error> {
    let pool = setup().await?;
    db::migrate(&pool).await?;
    let meal1 = Meal {
        title: "Meal1".to_string(),
        description: "description1".to_string(),
        rating: None,
        entered: Utc.ymd(2014, 7, 8).and_hms(9, 10, 11),
        source: None,
    };
    let meal2 = Meal {
        title: "Meal2".to_string(),
        description: "description2".to_string(),
        rating: Some(42),
        entered: Utc.ymd(2014, 7, 8).and_hms(9, 10, 14),
        source: Some("some cookbook".to_string()),
    };
    db::insert(&pool, &meal1).await?;
    db::insert(&pool, &meal2).await?;
    let r: Vec<Meal> = db::list(&pool).await.try_collect().await?;
    assert_eq!(r, vec!(meal2, meal1));
    Ok(())
}
