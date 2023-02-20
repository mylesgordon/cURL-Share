use backend::application::Application;
use dotenv::dotenv;
use env_logger::Env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    // TODO: turn sqlite address into enum
    let application = Application::build("data.db", 8080).await?;
    application.run_until_stopped().await?;

    Ok(())
}
