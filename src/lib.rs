use anyhow::{Ok, Result};
use async_trait::async_trait;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use services::dyndns2::Dyndns2;
use std::{collections::HashMap, env, fs, net::IpAddr, path::PathBuf, str::FromStr};
use url::Url;
mod services;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub interval: u64,
    pub ip_webservice: Url,
    pub services: Vec<ServiceTypes>,
}

impl Config {
    pub fn from(config_path: PathBuf) -> Result<Config> {
        debug!(
            "Reading config from {}",
            config_path.canonicalize()?.to_string_lossy()
        );
        let file = fs::read_to_string(config_path)?;
        let config = toml::from_str::<Config>(&file)?;
        debug!("Successfully read config");
        return Ok(config);
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
    async fn update_remote(&self, wan_ip: IpAddr) -> Result<()>;
    /** This function should check the cache and call update_remote and update the cache with the ip if
     * the ip address is not in the cache or do nothing if it is in the cache
     */
    async fn update(&self, wan_ip: IpAddr, cache: &mut Cache) -> Result<()> {
        match cache.get(self.get_cache_key()) {
            Some(cache_ip) => {
                // cache key found. if cache_ip is same as wan_ip dont do anythink
                if cache_ip == &wan_ip {
                    info!(
                        "Cache IP ({}) and WAN IP ({}) are identical, not updating remote ip",
                        cache_ip, wan_ip
                    )
                } else {
                    self.update_remote(wan_ip).await?;
                }
            }
            None => {
                //cache key not found, update it and add ip to cache
                self.update_remote(wan_ip).await?;
                cache.insert(self.get_cache_key(), wan_ip)?;
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Cache {
    pub path: PathBuf,
    cache: HashMap<String, IpAddr>,
}

impl Cache {
    pub fn insert(&mut self, k: String, v: IpAddr) -> Result<()> {
        self.cache.insert(k, v);
        let bytes = bincode::serialize(self)?;
        fs::write(&self.path, bytes)?;
        Ok(())
    }
    pub fn get(&self, k: String) -> Option<&IpAddr> {
        self.cache.get(&k)
    }
    pub fn new() -> Result<Cache> {
        let mut cache_path = env::current_dir()?;
        cache_path.push("cache.bin");
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
            let cache: HashMap<String, IpAddr> = HashMap::new();
            return Ok(Cache {
                path: cache_path,
                cache,
            });
        }
    }
}

pub async fn get_wan_ip(ip_webservice_url: Url) -> Result<IpAddr> {
    let ip_raw = reqwest::get(ip_webservice_url).await?.text().await?;
    let ip = IpAddr::from_str(&ip_raw.trim())?;
    Ok(ip)
}

pub async fn handle_service(
    service: &ServiceTypes,
    wan_ip: IpAddr,
    cache: &mut Cache,
) -> Result<()> {
    match service {
        ServiceTypes::Dyndns2(service) => service.update(wan_ip, cache).await?,
    }
    Ok(())
}
