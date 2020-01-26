use serde::Deserialize;
use serde_json;
use crate::error::BariumResult;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub password: Option<String>
}

impl Config {

    pub fn load(path: Option<String>) -> BariumResult<Self> {

        let content = std::fs::read_to_string(path.unwrap_or("config.json".to_string()))?;
        let this: Self = serde_json::from_str(&content)?;

        Ok(this)

    }

}
