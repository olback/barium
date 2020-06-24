use {
    std::{fs, path::PathBuf, time::{SystemTime, UNIX_EPOCH}},
    serde::{Serialize, Deserialize},
    serde_json,
    barium_shared::{UserId, UserHash},
    crate::{new_err, error::BariumResult, utils::conf_dir},
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Friend {
    pub display_name: String,
    #[serde(serialize_with="serialize_u8_32_arr", deserialize_with="deserialize_u8_32_arr")]
    pub hash: UserHash,
    pub added_on: u64
}

impl Friend {

    pub fn new(name: String, hash: UserHash) -> Self {

        Self {
            display_name: name,
            hash: hash,
            added_on: SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
        }

    }

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
    friends: Vec<Friend>
}

impl Server {

    pub fn new(
        user_id: UserId,
        name: String,
        address: String,
        port: u16,
        password: Option<String>,
        allow_invalid_cert: bool
    ) -> Self {

        Self {
            user_id,
            name,
            address,
            port,
            password,
            allow_invalid_cert,
            friends: Vec::new()
        }

    }

    pub fn add_friend(&mut self, new: Friend) -> BariumResult<()> {

        if self.find_friend(&new.hash).is_none() {

            self.friends.push(new);
            return Ok(())

        }

        Err(new_err!("Friend already added"))

    }

    pub fn remove_friend(&mut self, hash: &UserHash) -> BariumResult<()> {

        for i in 0..self.friends.len() {

            if &self.friends[i].hash == hash {

                self.friends.remove(i);
                return Ok(())

            }

        }

        Err(new_err!("Friend not found"))

    }

    pub fn find_friend(&self, hash: &UserHash) -> Option<&Friend> {

        for friend in &self.friends {

            if &friend.hash == hash {
                return Some(friend)
            }

        }

        None

    }

}

#[derive(Debug)]
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

    pub fn iter_mut<'s>(&'s mut self) -> std::slice::IterMut<'s, Server> {

        self.server_list.iter_mut()

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
