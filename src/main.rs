use clap::Parser;

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
    tasks::run(settings).await?;
    Ok(())
}
