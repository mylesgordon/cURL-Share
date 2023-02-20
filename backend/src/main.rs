use backend::{
    application::{Application, ApplicationPoolSettings},
    observability::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_subscriber("backend", "info", std::io::stdout);
    init_subscriber(subscriber);

    // TODO: turn sqlite address into enum
    let application = Application::build(8080, ApplicationPoolSettings::Standard).await?;
    application.run_until_stopped().await?;

    Ok(())
}
