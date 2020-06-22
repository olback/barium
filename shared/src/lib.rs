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
    DoNotDisturb,
    Offline
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ToServer {
    Ping,
    GetProperties,
    VerifyPassword(String),
    GetPublicKeys(UserId, Vec<UserHash>),
    Hello(UserId, rsa::RSAPublicKey, KeyBust, Option<String>), // My ID, Public Key, Key Bust, Server Password
    KeepAlive(UserId, Vec<UserHash>, AfkStatus), // My ID, Vec<Friend hash>
    Message(UserId, UserHash, EncryptedMessage) // My ID, Reciever Hash, Message data
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ToClient {
    Pong,
    Properties(ServerProperties),
    PasswordOk(bool),
    PublicKeys(Vec<(UserHash, rsa::RSAPublicKey)>),
    FriendsOnline(Vec<(UserHash, KeyBust, AfkStatus)>), // Vec<(Freind Hash, Key Bust, AFK Status)>
    Message(EncryptedMessage),
    Error(String)
}
