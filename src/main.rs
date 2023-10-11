use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use ddcrust::{get_wan_ip, handle_service, Cache, Config};
use env_logger::Env;
use tokio::time::{sleep, Duration};

#[derive(Parser, Debug)]
pub struct Args {
    /// Path to the config file
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,
    /// Instruct the program to just run once and not indefiniely
    #[arg(short, long, default_value = "false")]
    once: bool,
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();
    run(args)?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn run(args: Args) -> Result<()> {
    let config = Config::from(args.config)?;
    let mut cache = Cache::new()?;
    loop {
        let ip = get_wan_ip(config.ip_webservice.clone()).await?;
        for service in &config.services {
            handle_service(service, ip, &mut cache).await?;
        }
        if args.once {
            break;
        }
        sleep(Duration::from_secs(config.interval)).await
    }
    Ok(())
}
