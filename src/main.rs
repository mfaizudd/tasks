use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, env, default_value = "local")]
    environment: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let settings = tasks::config::get_config(&args.environment)?;
    let app_settings = settings.application;
    let address = format!("{}:{}", app_settings.host, app_settings.port).parse::<SocketAddr>()?;
    tasks::run(&address, settings.oauth).await?;
    Ok(())
}
