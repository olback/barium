use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerProperties {
    pub requires_password: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    cip: Vec<u8>,
    sig: Vec<u8>
}

impl EncryptedMessage {

    pub fn new(cip: Vec<u8>, sig: Vec<u8>) -> Self {

        assert_eq!(cip.len(), sig.len());

        Self {
            cip,
            sig
        }

    }

    pub fn from_slice(slice: &[u8]) -> Self {

        assert!(slice.len() % 2 == 0);

        let mid = slice.len() / 2;

        Self {
            cip: Vec::from(&slice[0..mid]),
            sig: Vec::from(&slice[mid..])
        }

    }

    pub fn to_vec(&self) -> Vec<u8> {

        [&self.cip[..], &self.sig[..]].concat()

    }

    pub fn ciphertext(&self) -> &Vec<u8> {

        &self.cip

    }

    pub fn signature(&self) -> &Vec<u8> {

        &self.sig

    }

}
