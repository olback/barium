use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Poke,
    Text(String)
}

impl Message {

    pub fn poke() -> Self {
        Self::Poke
    }

    pub fn text<M: Into<String>>(msg: M) -> Self {
        Self::Text(msg.into())
    }

}
