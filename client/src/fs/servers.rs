use {
    std::{fs, path::PathBuf},
    serde::{Serialize, Deserialize},
    serde_json,
    barium_shared::UserHash,
    crate::{
        error::BariumResult,
        utils::{conf_dir, serialize_u8_32_arr, deserialize_u8_32_arr}
    },
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Friend {
    pub display_name: String,
    #[serde(serialize_with="serialize_u8_32_arr", deserialize_with="deserialize_u8_32_arr")]
    pub hash: UserHash
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    pub name: String,
    pub address: String,
    pub port: u16,
    pub password: Option<String>,
    pub allow_invalid_cert: bool,
    pub friends: Vec<Friend>
}

#[derive(Clone, Debug)]
pub struct Servers {
    server_list: Vec<Server>
}

impl Servers {

    pub fn load() -> BariumResult<Self> {

        let path = Self::path()?;

        if path.is_file() {

            Ok(Self {
                server_list: serde_json::from_str(&fs::read_to_string(&path)?)?
            })

        } else {

            let servers = Self { server_list: Vec::new() };
            let json = serde_json::to_string_pretty(&servers.server_list)?;
            fs::write(&path, &json)?;

            Ok(servers)

        }

    }

    pub fn save(&self) -> BariumResult<()> {

        let path = Self::path()?;
        let json = serde_json::to_string_pretty(&self.server_list)?;
        fs::write(&path, &json)?;

        Ok(())

    }

    fn path() -> BariumResult<PathBuf> {

        Ok(conf_dir()?.join("servers.json"))

    }

}
