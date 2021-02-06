use chrono::prelude::*;
use clap::Clap;
use futures_core::stream::BoxStream;
use sqlx::sqlite::SqlitePoolOptions;

#[derive(sqlx::FromRow, Debug)]
pub struct Meal {
    pub title: String,
    pub url: Option<String>,
    pub description: String,
    pub rating: Option<i8>,
    pub entered: DateTime<Utc>,
}

impl Meal {
    pub fn render(&self) -> String {
        format!("{:?}", self)
    }
    pub async fn read() -> Result<Meal, sqlx::Error> {
        let stdin = async_std::io::stdin();
        let title = read_value(&stdin, "Title: ").await?;
        let unparsed_url = read_value(&stdin, "URL: ").await?;
        let url = if unparsed_url.is_empty() {
            None
        } else {
            Some(unparsed_url)
        };
        let description = read_value(&stdin, "Description: ").await?;
        let rating = None;
        let entered = Utc::now();
        Ok(Meal {
            title,
            url,
            description,
            rating,
            entered,
        })
    }
}

async fn read_value(
    stdin: &async_std::io::Stdin,
    description: &str,
) -> Result<String, sqlx::Error> {
    print!("{}", description);
    use std::io::Write;
    std::io::stdout().flush();
    let mut line = String::new();
    stdin.read_line(&mut line).await?;
    Ok(line)
}

#[derive(Clap, Debug)]
pub struct Config {
    #[clap(short, long)]
    pub file: String,
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Clap, Debug)]
pub enum Command {
    List(List),
    Add(Add),
}

#[derive(Clap, Debug)]
pub struct List {}

#[derive(Clap, Debug)]
pub struct Add {}

pub async fn connect(cfg: &Config) -> Result<sqlx::SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(1)
        .connect(&cfg.file)
        .await
}

pub async fn list(pool: &sqlx::SqlitePool) -> BoxStream<'_, Result<Meal, sqlx::Error>> {
    sqlx::query_as::<_, Meal>(
        "select title, url, description, rating, entered from meals order by entered desc",
    )
    .fetch(pool)
}

pub async fn insert(
    pool: &sqlx::SqlitePool,
    meal: &Meal,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query(
        "insert into meals (title, url, description, rating, entered) values (?, ?, ?, ?, ?)",
    )
    .bind(meal.title.clone())
    .bind(meal.url.clone())
    .bind(meal.description.clone())
    .bind(meal.rating)
    .bind(meal.entered)
    .execute(pool)
    .await
}
