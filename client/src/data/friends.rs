use serde::{Serialize, Deserialize};
use crate::error::BariumResult;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct JsonFriend {
    display_name: String,
    public_key: String,
    public_key_id: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Friend {
    display_name: String,
    public_key: rsa::RSAPublicKey,
    public_key_id: String
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Friends {
    friends: Vec<Friend>
}

impl Friend {

    pub fn new<Name: Into<String>, Key: Into<String>, Id: Into<String>>(display_name: Name, public_key: Key, key_id: Id) -> BariumResult<Self> {

        let key: rsa::RSAPublicKey = bincode::deserialize(&base64::decode(&public_key.into())?)?;

        Ok(Self {
            display_name: display_name.into(),
            public_key: key,
            public_key_id: key_id.into()
        })

    }

}

impl Friends {

    pub fn load() -> BariumResult<Friends> {

        let path = Self::file()?;

        let content = std::fs::read_to_string(&path)?;
        let friends_json: Vec<JsonFriend> = serde_json::from_str(&content)?;

        let mut friends = Vec::<Friend>::new();
        for f in friends_json {
            friends.push(Friend::new(f.display_name, f.public_key, f.public_key_id)?);
        }

        Ok(Self {
            friends: friends
        })

    }

    pub fn add_friend(&mut self, friend: Friend) -> BariumResult<&Friend> {

        self.friends.push(friend);
        self.save()?;

        Ok(self.friends.last().unwrap())

    }

    pub fn save(&self) -> BariumResult<()> {

        let mut json_friends = Vec::<JsonFriend>::new();
        for f in &self.friends {
            let key = base64::encode(&bincode::serialize(&f.public_key)?);
            json_friends.push(JsonFriend {
                display_name: f.display_name.clone(),
                public_key: key,
                public_key_id: f.public_key_id.clone()
            });

        }

        let path = Self::file()?;
        let content = serde_json::to_string_pretty(&json_friends)?;
        std::fs::write(&path, &content)?;

        Ok(())

    }

    fn file() -> BariumResult<PathBuf> {

        Ok(super::get_conf_dir()?.join("friends.json"))

    }

}
