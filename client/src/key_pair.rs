use rsa::{RSAPublicKey, RSAPrivateKey};
use crate::error::BariumResult;

#[derive(Debug)]
pub struct KeyPair {
    public: RSAPublicKey,
    private: RSAPrivateKey
}

impl KeyPair {

    pub fn new(size: usize) -> BariumResult<Self> {

        let mut rng = rand::thread_rng();

        let private_key = rsa::RSAPrivateKey::new(&mut rng, size)?;
        let public_key = private_key.to_public_key();

        Ok(Self {
            public: public_key,
            private: private_key
        })

    }

    pub fn public_key(&self) -> RSAPublicKey {

        self.public.clone()

    }

    pub fn private_key(&self) -> RSAPrivateKey {

        self.private.clone()

    }

}
