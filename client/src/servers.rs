use {
    std::{fs, path::PathBuf},
    serde::{Serialize, Deserialize},
    serde_json,
    barium_shared::{UserId, UserHash},
    crate::{new_err, error::BariumResult, utils::conf_dir},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Friend {
    pub display_name: String,
    #[serde(serialize_with="serialize_u8_32_arr", deserialize_with="deserialize_u8_32_arr")]
    pub hash: UserHash
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    #[serde(serialize_with="serialize_u8_32_arr", deserialize_with="deserialize_u8_32_arr")]
    pub user_id: UserId,
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

            let servers = Self::default();
            let json = serde_json::to_string_pretty(&servers.server_list)?;
            fs::write(&path, &json)?;

            Ok(servers)

        }

    }

    pub fn add(&mut self, server: Server) -> BariumResult<()> {

        if self.find(&server.address, &server.port).is_err() {
            self.server_list.push(server);
            self.save()?;
            return Ok(())
        }

        Err(new_err!("Cannot add the same server twice"))

    }

    pub fn remove(&mut self, address: String, port: u16) -> BariumResult<()> {

        for i in 0..self.len() {

            if self.server_list[i].address == address && self.server_list[i].port == port {
                self.server_list.remove(i);
                self.save()?;
                return Ok(())
            }

        }

        Err(new_err!(format!("Server \"{}:{}\" does not exist in server list", address, port)))

    }

    pub fn find(&self, address: &String, port: &u16) -> BariumResult<&Server> {

        for server in &self.server_list {

            if &server.address == address && &server.port == port {
                return Ok(server)
            }

        }

        Err(new_err!(format!("Server \"{}:{}\" does not exist in server list", address, port)))

    }

    pub fn find_mut(&mut self, address: &String, port: &u16) -> BariumResult<&mut Server> {

        for server in &mut self.server_list {

            if &server.address == address && &server.port == port {
                return Ok(server)
            }

        }

        Err(new_err!(format!("Server \"{}:{}\" does not exist in server list", address, port)))

    }

    pub fn len(&self) -> usize {

        self.server_list.len()

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

    pub fn iter<'s>(&'s self) -> std::slice::Iter<'s, Server> {

        self.server_list.iter()

    }

}

impl Default for Servers {

    fn default() -> Self {

        Self {
            server_list: Vec::new()
        }

    }

}

fn serialize_u8_32_arr<S>(u8_32_arr: &[u8; 32], serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {

    let b62 = base62::encode(u8_32_arr);

    serializer.serialize_str(b62.as_str())

}

fn deserialize_u8_32_arr<'de, D>(de: D) -> Result<[u8; 32], D::Error>
    where D: serde::Deserializer<'de> {

    use std::convert::TryInto;

    let u8_32_arr_str: &str = serde::de::Deserialize::deserialize(de)?;
    let u8_32_arr = base62::decode(u8_32_arr_str).map_err(serde::de::Error::custom)?;

    u8_32_arr.as_slice().try_into().map_err(serde::de::Error::custom)

}
