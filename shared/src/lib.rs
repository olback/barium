use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum AfkStatus {
    Available,
    Away(Option<u32>),
    DoNotDisturb
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub to: [u8; 32], // Friend Hash
    pub data: Vec<u8> // RSA(enum MessageData { Message(String), Poke })
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ToServer {
    KeepAlive([u8; 32], Vec<[u8; 32]>, AfkStatus), // My id, Vec<Friend hash>
    Message(Message)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ToClient {
    FriendsOnline(Vec<([u8; 32], AfkStatus)>), //  Vec<Freind hash>
    Message(Vec<u8>)
}
