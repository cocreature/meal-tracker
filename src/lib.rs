use chrono::prelude::*;
use clap::Clap;

pub mod db;

#[derive(sqlx::FromRow, Debug, PartialEq)]
pub struct Meal {
    pub title: String,
    pub source: Option<String>,
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
        let unparsed_source = read_value(&stdin, "URL: ").await?;
        let source = if unparsed_source.is_empty() {
            None
        } else {
            Some(unparsed_source)
        };
        let description = read_value(&stdin, "Description: ").await?;
        let rating = None;
        let entered = Utc::now();
        Ok(Meal {
            title,
            source,
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
    std::io::stdout().flush()?;
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
