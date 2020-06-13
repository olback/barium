use {
    std::{fs, path::PathBuf},
    crate::{consts, error::{BariumError, BariumResult}},
    dirs
};

pub fn conf_dir() -> BariumResult<PathBuf> {

    let dir = dirs::config_dir().ok_or(
        BariumError::new("could not determine config dir", std::file!(), std::line!())
    )?.join(consts::CONFIG_DIR);

    if !dir.is_dir() {
        fs::create_dir_all(&dir)?;
    }

    Ok(dir)

}
