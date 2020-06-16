use serde::{Serialize, Deserialize};
use rsa;

pub mod hash;
mod structs;
mod types;
pub use types::*;
pub use structs::*;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AfkStatus {
    Available,
    Away(Option<u32>),
    DoNotDisturb
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ToServer {
    Ping,
    GetProperties,
    VerifyPassword(String),
    GetPublicKey(UserId, UserHash),
    Hello(UserId, rsa::RSAPublicKey, Option<String>), // My ID, Public Key, Server Password
    KeepAlive(UserId, Vec<UserHash>, AfkStatus), // My ID, Vec<Friend hash>
    Message(UserId, UserHash, Vec<u8>) // My ID, Reciever Hash, Message data
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ToClient {
    Pong,
    Properties(ServerProperties),
    PasswordOk(bool),
    PublicKey(UserHash, rsa::RSAPublicKey),
    FriendsOnline(Vec<(UserHash, AfkStatus)>), // Vec<(Freind Hash, AFK Status)>
    Message(Vec<u8>)
}
