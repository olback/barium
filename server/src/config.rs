use serde::{Serialize, Deserialize};
use serde_json;
use crate::error::BariumResult;
use log::{info, warn};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Cert {
    pub path: PathBuf,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
    pub password: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub cert: Cert,
    pub server: Server,
    pub blacklist: Vec<String>
}

impl Config {

    pub fn load(path: Option<String>) -> BariumResult<Self> {

        let content = std::fs::read_to_string(path.unwrap_or("config.json".to_string()))?;
        let this: Self = serde_json::from_str(&content)?;

        Ok(this)

    }

    pub fn log(&self) {

        // Server address/port
        info!("Listening on {}:{}", self.server.address, self.server.port);

        // Cert info
        info!("Using certificate at {}", self.cert.path.to_str().unwrap());

        // Password
        match &self.server.password {
            Some(p) => info!("Connection password '{}' set", p),
            None => info!("No password set")
        };

        // Blacklist
        if self.blacklist.is_empty() {
            info!("Blacklist empty");
        } else {
            info!("Blocking {} remotes:", self.blacklist.len());
            for b in &self.blacklist {
                info!("=> {}", b);
            }
        }

    }

}

impl Default for Config {

    fn default() -> Self {

        warn!("No configuration specified. Using default!");

        Self {
            cert: Cert {
                path: PathBuf::from("cert.p12"),
                password: String::from("")
            },
            server: Server {
                address: String::from("0.0.0.0"),
                port: 13337,
                password: None
            },
            blacklist: Vec::new()
        }

    }

}
