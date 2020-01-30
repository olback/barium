use serde::{Serialize, Deserialize};
use serde_json;
use crate::error::BariumResult;

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
    pub password: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cert {
    pub password: String,
    pub path: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub server: Server,
    pub cert: Cert,
    pub blacklist: Vec<String>
}

impl Config {

    pub fn load(path: Option<String>) -> BariumResult<Self> {

        let content = std::fs::read_to_string(path.unwrap_or("config.json".to_string()))?;
        let this: Self = serde_json::from_str(&content)?;

        Ok(this)

    }

}

// impl Default for Config {

//     fn default() -> Self {

//         Self {
//             server: Server {
//                 address: String::from("0.0.0.0"),
//                 port: 13337,
//                 password: None
//             },
//             cert: Cert {
//                 path: String::from("cert/certificate.p12"),
//                 password: String::new()
//             },
//             blacklist: Vec::new()
//         }

//     }

// }
