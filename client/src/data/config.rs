use serde::{Serialize, Deserialize};
use crate::error::BariumResult;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {

}

impl Config {

    pub fn load() -> BariumResult<Self> {

        let path = Self::file()?;
        let content = std::fs::read_to_string(&path)?;
        let this: Self = serde_json::from_str(&content)?;

        Ok(this)

    }

    pub fn save(&self) -> BariumResult<()> {

        let path = Self::file()?;
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, &content)?;

        Ok(())

    }

    fn file() -> BariumResult<PathBuf> {

        Ok(super::get_conf_dir()?.join("config.json"))

    }

}
