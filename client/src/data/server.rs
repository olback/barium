use serde::{Serialize, Deserialize};
use super::friend::Friend;
use crate::{new_err, error::BariumResult};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
    pub password: Option<String>,
    pub key_size: usize,
    pub friends: Vec<Friend>,
    pub unsafe_allow_invalid_cert: Option<bool>
}

impl Server {

    pub fn add_friend(&mut self, friend: Friend) -> BariumResult<()> {

        for f in &self.friends {
            if f.hash() == friend.hash() {
                return Err(new_err!("Friend already added"))
            }
        }

        self.friends.push(friend);

        Ok(())

    }

    pub fn remove_friend(&mut self, friend: Friend) -> BariumResult<()> {

        for (i, f) in self.friends.iter().enumerate() {
            if f.hash() == friend.hash() {
                self.friends.remove(i);
                return Ok(())
            }
        }

        Err(new_err!("Friend not found"))

    }

}
