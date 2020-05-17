use serde::{Serialize, Deserialize};
use barium_shared::UserHash;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Friend {
    hash: UserHash,
    display_name: String
}

impl Friend {

    pub fn new<N: Into<String>>(hash: UserHash, display_name: N) -> Self {

        Self {
            hash: hash,
            display_name: display_name.into()
        }

    }

    pub fn hash(&self) -> &UserHash {
        &self.hash
    }

    pub fn display_name(&self) -> &String {
        &self.display_name
    }

}
