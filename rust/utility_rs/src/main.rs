mod scraper;
mod ws;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use url::Url;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(about = "Run the marquee scraper")]
    Scraper,
    #[clap(about = "Run the MAME WebSocket listener")]
    MameWs,
    #[clap(about = "Run the Supermodel WebSocket listener")]
    SupermodelWs,
    #[clap(about = "Run the Visual Pinball WebSocket listener")]
    VPinballWs,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Scraper => {
            let pool_file = PathBuf::from("scrap.pool");
            scraper::run(&pool_file).await?;
        }
        Commands::MameWs => {
            let url = Url::parse("ws://127.0.0.1:8080")?;
            ws::run_listener(url, ws::handle_mame_message).await?;
        }
        Commands::SupermodelWs => {
            let url = Url::parse("ws://127.0.0.1:8081")?;
            ws::run_listener(url, |msg| println!("Supermodel message: {}", msg)).await?;
        }
        Commands::VPinballWs => {
            let url = Url::parse("ws://127.0.0.1:8082")?;
            ws::run_listener(url, |msg| println!("Visual Pinball message: {}", msg)).await?;
        }
    }

    Ok(())
}
