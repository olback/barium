use serde::Deserialize;
use serde_json;
use crate::error::BariumResult;
use log::info;
use std::{
    path::PathBuf,
    net::IpAddr,
    str::FromStr
};
use ipnet::IpNet;
use either::Either;

type BlacklistEntry = Either<IpAddr, IpNet>;
type Blacklist = Vec<BlacklistEntry>;

#[derive(Debug, Deserialize)]
pub struct Cert {
    pub path: PathBuf,
    pub password: String
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
    pub password: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct Runtime {
    /// The amount of native threads to use. Set to null to use all available.
    pub core_threads: Option<usize>,
    /// Max green threads to spawn.
    pub max_threads: Option<usize>
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub cert: Cert,
    pub server: Server,
    pub runtime: Runtime,
    #[serde(deserialize_with = "deserialize_blacklist")]
    pub blacklist: Blacklist,
    #[serde(deserialize_with = "deserialize_log_level")]
    pub log_level: Option<log::LevelFilter>
}

impl Config {

    pub fn load(path: String) -> BariumResult<Self> {

        let content = std::fs::read_to_string(path)?;
        let this: Self = serde_json::from_str(&content)?;

        Ok(this)

    }

    pub fn is_blacklisted(&self, addr: &IpAddr) -> bool {

        for b in &self.blacklist {
            if match b {
                Either::Left(ref baddr) => addr == baddr,
                Either::Right(ref bnet) => bnet.contains(addr),
            } {
                return true
            }
        }

        false

    }

    pub fn log(&self) {

        // Server address/port
        info!("Listening on {}:{}", self.server.address, self.server.port);

        // Cert info
        info!("Certificate: {}", self.cert.path.canonicalize().unwrap().to_str().unwrap());

        // Password
        match &self.server.password {
            Some(p) => info!("Connection password: \"{}\"", p),
            None => info!("Connection password: <not set>")
        };

        // Blacklist
        if self.blacklist.is_empty() {

            info!("Blacklist: Empty");

        } else {

            let mut count = 0usize;
            for b in &self.blacklist {
                match b {
                    Either::Left(_) => count += 1,
                    Either::Right(ref net) => count += net.hosts().count()
                }
            }

            info!("Blocking {} remotes:", count);
            for b in &self.blacklist {
                info!("=> {}", b);
            }

        }

    }

}

fn deserialize_blacklist<'de, D>(de: D) -> Result<Blacklist, D::Error>
    where D: serde::Deserializer<'de>
{

    let raw_s_v: Vec<&str> = serde::de::Deserialize::deserialize(de)?;

    let mut ret = Vec::<BlacklistEntry>::new();

    for e in raw_s_v {

        let mut errors = 0u8;

        match e.parse::<IpAddr>() {
            Ok(addr) => {
                ret.push(Either::Left(addr));
                continue;
            },
            Err(_) => {
                errors += 1;
            }
        };

        match e.parse::<IpNet>() {
            Ok(net) => {
                ret.push(Either::Right(net));
                continue;
            },
            Err(_) => {
                errors += 1;
            }
        };

        if errors == 2 {

            let err: Result<Blacklist, String> = Err(format!("Could not parse \"{}\" as std::net::IpAddr or ipnet::IpNet", e));
            return err.map_err(serde::de::Error::custom)

        } // else, no error

    }

    Ok(ret)
}

fn deserialize_log_level<'de, D>(de: D) -> Result<Option<log::LevelFilter>, D::Error>
where D: serde::Deserializer<'de> {

    let level: Option<&str> = serde::de::Deserialize::deserialize(de)?;

    match level {
        Some(l) => Ok(Some(log::LevelFilter::from_str(l).map_err(serde::de::Error::custom)?)),
        None => Ok(None)
    }

}
