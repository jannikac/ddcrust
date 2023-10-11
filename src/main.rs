use anyhow::Result;
use ddcrust::{get_wan_ip, handle_service, Cache, Config};
use tokio::time::{sleep, Duration};

#[tokio::main]
    let config = Config::from(args.config)?;
    let mut cache = Cache::new()?;
    loop {
        let ip = get_wan_ip(config.ip_webservice.clone()).await?;
        for service in &config.services {
            handle_service(service, ip, &mut cache).await?;
        }
        sleep(Duration::from_secs(config.interval)).await
    }
}
