use serde::{Serialize, Deserialize};
use serde_json;
use bincode;
use base64;
use crate::error::BariumResult;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
pub struct Identity {
    private_key: String,
    public_key: String
}

impl Identity {

    pub fn from_private_key(private_key: &rsa::RSAPrivateKey) -> BariumResult<Self> {

        let public_key = private_key.to_public_key();

        let private_b64 = base64::encode(&bincode::serialize(private_key)?);
        let public_b64 = base64::encode(&bincode::serialize(&public_key)?);

        Ok(Self {
            private_key: private_b64,
            public_key: public_b64
        })

    }

    pub fn load() -> BariumResult<Self> {

        let path = Self::file()?;

        let content = std::fs::read_to_string(&path)?;
        let this: Self = serde_json::from_str(&content)?;

        Ok(this)

    }

    pub fn save(&self) -> BariumResult<()> {

        let path = Self::file()?;

        let content = serde_json::to_string(&self)?;
        std::fs::write(&path, &content)?;

        Ok(())

    }

    pub fn get_key_pair(&self) -> BariumResult<(rsa::RSAPublicKey, rsa::RSAPrivateKey)> {

        let private_bytes = base64::decode(&self.private_key)?;
        let public_bytes = base64::decode(&self.public_key)?;

        let private: rsa::RSAPrivateKey = bincode::deserialize(&private_bytes)?;
        let public: rsa::RSAPublicKey = bincode::deserialize(&public_bytes)?;

        Ok((public, private))

    }

    fn file() -> BariumResult<PathBuf> {

        Ok(super::get_conf_dir()?.join("identity.json"))

    }

}
