use crate::ServiceTrait;

use crate::WanIps;
use anyhow::{Ok, Result};
use async_trait::async_trait;
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct Dyndns2 {
    pub server: String,
    pub login: String,
    pub password: String,
    pub identifier: String,
}

#[async_trait]
impl ServiceTrait for Dyndns2 {
    fn get_server(&self) -> &String {
        &self.server
    }
    fn get_identifier(&self) -> &String {
        &self.identifier
    }
    async fn update_remote(&self, wan_ip: &WanIps) -> Result<()> {
        let mut params = vec![];
        if let Some(ip) = wan_ip.ip {
            params.push(("myip", ip.to_string()));
        }
        if let Some(ip) = wan_ip.ipv6 {
            params.push(("myipv6", ip.to_string()));
        }
        params.push(("hostname", self.server.clone()));

        let url = Url::parse_with_params(&format!("https://{}/nic/update", self.server), &params)?;
        let client = Client::new();
        let res = client
            .get(url)
            .basic_auth(&self.login, Some(&self.password))
            .send()
            .await?;
        info!(
            "{status} {message}",
            status = res.status(),
            message = res.text().await?
        );
        Ok(())
    }
}
