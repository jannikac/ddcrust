use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use local_ip_address::{local_ip, local_ipv6};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use services::dyndns2::Dyndns2;
use std::{collections::HashMap, env, fmt::Display, fs, net::IpAddr, path::PathBuf};
use url::Url;
mod services;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub interval: u64,
    pub ip_webservice: Url,
    pub services: Vec<ServiceTypes>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct WanIps {
    ip: Option<IpAddr>,
    ipv6: Option<IpAddr>,
}

impl Display for WanIps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IPV4: {}, IPV6: {}",
            self.ip.map_or("None".to_string(), |v| v.to_string()),
            self.ipv6.map_or("None".to_string(), |v| v.to_string())
        )
    }
}

impl Config {
    pub fn from(config_path: PathBuf) -> Result<Config> {
        let abs_path = match config_path.is_relative() {
            true => {
                let current_dir = env::current_dir().unwrap(); //this should never error
                PathBuf::from_iter([current_dir, config_path])
            }
            false => config_path,
        };

        let canonical_path = abs_path
            .canonicalize()
            .with_context(|| format!("Could not read path {}", abs_path.to_string_lossy()))?;

        debug!("Reading config from {}", canonical_path.to_string_lossy());
        let file = fs::read_to_string(canonical_path).context("Could not read config file")?;
        let config = toml::from_str::<Config>(&file).context("Could not parse config file")?;
        debug!("Successfully read config");
        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServiceTypes {
    #[serde(rename = "dyndns2")]
    Dyndns2(Dyndns2),
}

#[async_trait]
pub trait ServiceTrait {
    /// This fn returns a field of the struct called identifier that has to be defined
    fn get_identifier(&self) -> &String;
    /// This fn returns a field of the struct called server that has to be defined
    fn get_server(&self) -> &String;
    /// This fn should create a key for the cache from the identifier and server. default is identifier@server
    fn get_cache_key(&self) -> String {
        format!("{}@{}", self.get_identifier(), self.get_server())
    }
    /// This function should update the remote service with the wan_ip adress parameter
    async fn update_remote(&self, wan_ip: &WanIps) -> Result<()>;
    /** This function should check the cache and call update_remote and update the cache with the ip if
     * the ip address is not in the cache or do nothing if it is in the cache
     */
    async fn update(&self, wan_ip: WanIps, cache: &mut Cache) -> Result<()> {
        match cache.get(self.get_cache_key()) {
            Some(cache_ip) => {
                // cache key found. if cache_ip is same as wan_ip dont do anything
                if cache_ip == &wan_ip {
                    info!(
                        "Cache IP ({}) and WAN IP ({}) are identical, not updating remote ip",
                        cache_ip, wan_ip
                    )
                } else {
                    self.update_remote(&wan_ip).await?;
                }
            }
            None => {
                //cache key not found, update it and add ip to cache
                self.update_remote(&wan_ip).await?;
                cache.insert(self.get_cache_key(), wan_ip)?;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub path: PathBuf,
    cache: HashMap<String, WanIps>,
}

impl Cache {
    pub fn insert(&mut self, k: String, v: WanIps) -> Result<()> {
        self.cache.insert(k, v);
        let bytes = bincode::serialize(self)?;
        fs::write(&self.path, bytes)?;
        Ok(())
    }
    pub fn get(&self, k: String) -> Option<&WanIps> {
        self.cache.get(&k)
    }
    pub fn new() -> Result<Cache> {
        let cache_path = PathBuf::from_iter([env::current_dir()?, PathBuf::from("cache.bin")]);
        if cache_path.exists() {
            debug!(
                "Cache file found ({})",
                cache_path.canonicalize()?.to_string_lossy()
            );
            let bytes = fs::read(&cache_path)?;
            let cache = bincode::deserialize::<Cache>(&bytes).unwrap();
            return Ok(cache);
        } else {
            debug!(
                "No cache file found in ({}), creating one",
                cache_path.canonicalize()?.to_string_lossy()
            );
            let cache: HashMap<String, WanIps> = HashMap::new();
            return Ok(Cache {
                path: cache_path,
                cache,
            });
        }
    }
}

enum IpKind {
    V4,
    V6,
}

async fn get_ip(ip_webservice_url: Url, kind: IpKind) -> Result<IpAddr> {
    let local_ip = match kind {
        IpKind::V4 => local_ip()?,
        IpKind::V6 => local_ipv6()?,
    };
    let client = reqwest::Client::builder()
        .local_address(local_ip)
        .build()
        .unwrap();
    let wan_ip_raw = client.get(ip_webservice_url).send().await?.text().await?;
    Ok(wan_ip_raw.trim().parse()?)
}

pub async fn get_wan_ip(ip_webservice_url: Url) -> Result<WanIps> {
    let ip = get_ip(ip_webservice_url.clone(), IpKind::V4)
        .await
        .map_err(|e| debug!("{}", e))
        .ok();
    let ipv6 = get_ip(ip_webservice_url.clone(), IpKind::V6)
        .await
        .map_err(|e| debug!("{}", e))
        .ok();
    if ip.is_none() && ipv6.is_none() {
        return Err(anyhow!("No WAN ipv4 or ipv6 could be determined"));
    }

    Ok(WanIps { ip, ipv6 })
}

pub async fn handle_service(
    service: &ServiceTypes,
    wan_ip: WanIps,
    cache: &mut Cache,
) -> Result<()> {
    match service {
        ServiceTypes::Dyndns2(service) => service.update(wan_ip, cache).await?,
    }
    Ok(())
}
