use serde::{Serialize, Deserialize};
use crate::error::BariumResult;
use std::path::PathBuf;

pub(super) mod friend;
mod server;
mod utils;
use server::Server;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    display_name: String,
    servers: Vec<Server>
}

impl Config {

    pub fn load() -> BariumResult<Self> {

        Self::from_fs().or(Ok(Self::default()))

    }

    pub fn servers(&mut self) -> &mut Vec<Server> {
        &mut self.servers
    }

    pub fn add_server(&mut self, server: Server) {
        unimplemented!()
    }

    pub fn remove_server(&mut self, server: Server) {
        unimplemented!()
    }

    pub fn save(&self) -> BariumResult<()> {

        println!("Saving config...");

        let path = Self::file()?;
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, &content)?;

        Ok(())

    }

    fn from_fs() -> BariumResult<Self> {

        let path = Self::file()?;
        let content = std::fs::read_to_string(&path)?;
        let this: Self = serde_json::from_str(&content)?;

        Ok(this)

    }

    fn file() -> BariumResult<PathBuf> {

        Ok(utils::get_conf_dir()?.join("config.json"))

    }

}

impl Default for Config {

    fn default() -> Self {

        Self {
            display_name: String::from("Me"),
            servers: Vec::new()
        }

    }

}
