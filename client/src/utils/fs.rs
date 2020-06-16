use {
    std::{fs, path::PathBuf},
    crate::{new_err, consts::CONFIG_DIR, error::{BariumResult}},
    dirs
};

pub fn conf_dir() -> BariumResult<PathBuf> {

    let dir = dirs::config_dir().ok_or(
        new_err!("could not determine config dir")
    )?.join(CONFIG_DIR);

    if !dir.is_dir() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)

}
