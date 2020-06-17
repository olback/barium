use {
    rand::{self, Rng},
    rsa::{RSAPublicKey, RSAPrivateKey},
    crate::error::BariumResult,
    barium_shared::KeyBust
};

#[derive(Debug)]
pub struct KeyPair {
    public: RSAPublicKey,
    private: RSAPrivateKey,
    key_bust: KeyBust
}

impl KeyPair {

    pub fn new(size: usize) -> BariumResult<Self> {

        let mut rng = rand::thread_rng();

        let private = rsa::RSAPrivateKey::new(&mut rng, size)?;
        let public = private.to_public_key();
        let key_bust = rng.gen::<KeyBust>();

        Ok(Self { public, private, key_bust })

    }

    pub fn public_key(&self) -> &RSAPublicKey {

        &self.public

    }

    pub fn private_key(&self) -> &RSAPrivateKey {

        &self.private

    }

    pub fn key_bust(&self) -> KeyBust {

        self.key_bust.clone()

    }

}
