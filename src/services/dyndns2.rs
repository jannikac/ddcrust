use std::net::IpAddr;

use crate::ServiceTrait;

use anyhow::{Ok, Result};
use async_trait::async_trait;
use log::info;
use reqwest::Client;
use serde::{Deserialize, Serialize};

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
    async fn update_remote(&self, wan_ip: IpAddr) -> Result<()> {
        let update_url: String = match wan_ip {
            IpAddr::V4(ip) => format!(
                "https://{server}/nic/update?hostname={identifier}&myip={ip}",
                server = self.server,
                identifier = self.identifier
            ),
            IpAddr::V6(ip) => format!(
                "https://{server}/nic/update?hostname={identifier}&myipv6={ip}",
                server = self.server,
                identifier = self.identifier
            ),
        };
        let client = Client::new();
        let res = client
            .get(update_url)
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
