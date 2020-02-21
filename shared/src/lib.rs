use serde::{Serialize, Deserialize};

pub type UserHash = [u8; 32];
pub type UserId = [u8; 32];

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
    KeepAlive(UserId, Vec<UserHash>, AfkStatus), // My id, Vec<Friend hash>
    Message(Message)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ToClient {
    FriendsOnline(Vec<(UserHash, AfkStatus)>), //  Vec<Freind hash>
    Message(Vec<u8>)
}
