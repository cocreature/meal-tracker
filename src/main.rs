use cooking::Command;
use cooking::Config;
use cooking::Meal;
use clap::Clap;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    let config = Config::parse();
    let pool = cooking::connect(&config).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    match config.cmd {
        Command::Add(_) => {
            let meal = Meal::read().await?;
            cooking::insert(&pool, &meal).await?;
            println!("Meal added.");
        }
        Command::List(_) => {
            use futures::stream::TryStreamExt;
            let mut stream = cooking::list(&pool).await;
            while let Some(meal) = stream.try_next().await? {
                println!("{}", meal.render());
            }
        }
    }
    Ok(())
}
