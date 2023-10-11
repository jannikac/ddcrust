use anyhow::Result;
use ddcrust::{get_wan_ip, handle_service, read_config, Cache};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let config = read_config()?;
    let mut cache = Cache::new()?;
    loop {
        let ip = get_wan_ip(config.ip_webservice.clone()).await?;
        for service in &config.services {
            handle_service(service, ip, &mut cache).await?;
        }
        sleep(Duration::from_secs(config.interval)).await
    }
}
