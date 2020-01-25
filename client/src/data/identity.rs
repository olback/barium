use serde::{Serialize, Deserialize};
use bincode;
use base64;

#[derive(Clone, Serialize, Deserialize)]
pub struct Identity {
    private_key: String,
    public_key: String
}

impl Identity {

    pub fn from_private_key(private_key: &rsa::RSAPrivateKey) -> Self {

        let public_key = private_key.to_public_key();

        // Unwrap is probably safe here.
        let private_b64 = base64::encode(&bincode::serialize(private_key).unwrap());
        let public_b64 = base64::encode(&bincode::serialize(&public_key).unwrap());

        Self {
            private_key: private_b64,
            public_key: public_b64
        }

    }

    pub fn save(&self) -> std::io::Result<()> {

        unimplemented!();

        Ok(())

    }

}
