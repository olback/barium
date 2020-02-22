use rsa::{RSAPublicKey, RSAPrivateKey};
use crate::{
    data::server::Server,
    error::BariumResult
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Key {
    public: RSAPublicKey,
    private: RSAPrivateKey
}

impl Key {

    pub fn new(size: usize) -> BariumResult<Self> {

        let mut rng = rand::thread_rng();

        let private_key = rsa::RSAPrivateKey::new(&mut rng, size)?;
        let public_key = private_key.to_public_key();

        Ok(Self {
            public: public_key,
            private: private_key
        })

    }

}

#[derive(Debug)]
pub struct KeyStore {
    keys: HashMap<String, Key>
}

impl KeyStore {

    pub fn new() -> Self {

        Self {
            keys: HashMap::new()
        }

    }

    pub fn add(&mut self, server: &Server) -> BariumResult<()> {

        let index = format!("{}:{}", server.address, server.port);
        let key = Key::new(server.key_size)?;
        self.keys.insert(index, key);

        Ok(())

    }

    // pub fn from_server_list(servers: &Vec<Server>) -> BariumResult<Self> {

    //     let mut inner = Self::new();

    //     for s in servers {
    //         inner.add(s)?;
    //     }

    //     Ok(inner)

    // }

}
