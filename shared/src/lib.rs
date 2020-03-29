use serde::{Serialize, Deserialize};
use rsa;

pub mod hash;
mod types;
pub use types::*;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AfkStatus {
    Available,
    Away(Option<u32>),
    DoNotDisturb
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub to: UserHash, // Friend Hash
    pub data: Vec<u8> // RSA(enum MessageData { Message(String), Poke })
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ToServer {
    Ping,
    Hello(UserId, rsa::RSAPublicKey, Option<String>),
    KeepAlive(UserId, Vec<UserHash>, AfkStatus), // My id, Vec<Friend hash>
    Message(UserId, Message)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ToClient {
    Pong,
    FriendsOnline(Vec<(UserHash, AfkStatus)>), //  Vec<Freind hash>
    Message(Vec<u8>)
}
