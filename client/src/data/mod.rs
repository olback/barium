use dirs;
use std::path::PathBuf;
use crate::{
    new_err,
    error::BariumResult
};

mod config;
mod friends;
mod identity;

pub(super) fn get_conf_dir() -> BariumResult<PathBuf> {

    let conf_dir = match dirs::config_dir() {
        Some(dir) => dir,
        None => return Err(new_err!("Could not get config dir"))
    };


    let barium_conf_dir = conf_dir.join("barium");
    if !barium_conf_dir.exists() {
        std::fs::create_dir_all(&barium_conf_dir)?;
    }

    Ok(barium_conf_dir)

}
