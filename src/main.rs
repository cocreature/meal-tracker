use clap::Clap;
use futures::stream::TryStreamExt;

use cooking::{Command, Config, Meal, db};

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();
    let config = Config::parse();
    let pool = db::connect(&config).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    match config.cmd {
        Command::Add(_) => {
            let meal = Meal::read().await?;
            db::insert(&pool, &meal).await?;
            println!("Meal added.");
        }
        Command::List(_) => {
            let mut stream = db::list(&pool).await;
            while let Some(meal) = stream.try_next().await? {
                println!("{}", meal.render());
            }
        }
    }
    Ok(())
}
