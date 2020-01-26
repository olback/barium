use serde::{Serialize, Deserialize};
use crate::error::BariumResult;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {

}

impl Config {

    fn file() -> BariumResult<PathBuf> {

        Ok(super::get_conf_dir()?.join("config.json"))

    }

}
