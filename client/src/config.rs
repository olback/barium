use {
    std::{fs, path::PathBuf, str::FromStr},
    serde::{Serialize, Deserialize},
    serde_json,
    crate::{error::BariumResult, utils::conf_dir}
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(serialize_with="serialize_log_level", deserialize_with="deserialize_log_level")]
    pub log_level: log::LevelFilter
}

impl Config {

    pub fn load() -> BariumResult<Self> {

        let path = Self::path()?;

        if path.is_file() {

            Ok(serde_json::from_str(&fs::read_to_string(&path)?)?)

        } else {

            let config = Self::default();
            let json = serde_json::to_string_pretty(&config)?;
            fs::write(&path, &json)?;

            Ok(config)

        }

    }

    fn path() -> BariumResult<PathBuf> {

        Ok(conf_dir()?.join("config.json"))

    }

}

impl Default for Config {

    fn default() -> Self {

        Self {
            log_level: log::LevelFilter::Info
        }

    }

}

pub fn serialize_log_level<S>(level: &log::LevelFilter, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {

    serializer.serialize_str(&level.to_string())

}

fn deserialize_log_level<'de, D>(de: D) -> Result<log::LevelFilter, D::Error>
where D: serde::Deserializer<'de> {

    let level: &str = serde::de::Deserialize::deserialize(de)?;

    log::LevelFilter::from_str(level).map_err(serde::de::Error::custom)

}
