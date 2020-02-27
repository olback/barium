use std::{
    collections::HashMap,
    net::TcpStream,
    io::Write,
    sync::{
        Arc,
        Mutex,
        RwLock
    }
};
// use tokio::net::TcpStream;
use padlock;
use barium_shared::{AfkStatus, ToClient, UserHash};
use crate::error::BariumResult;
use bincode;
use rsa;

pub type Clients = Arc<RwLock<HashMap<UserHash, Client>>>;

#[derive(Debug)]
pub struct Client {
    stream: Mutex<TcpStream>,
    key: rsa::RSAPublicKey,
    idle: RwLock<AfkStatus>
}

impl Client {

    pub fn new(stream: &TcpStream, key: rsa::RSAPublicKey, idle: AfkStatus) -> BariumResult<Self> {

        Ok(Self {
            stream: Mutex::new(stream.try_clone()?),
            key: key,
            idle: RwLock::new(idle)
        })

    }

    pub fn set_idle(&self, idle: AfkStatus) {

        padlock::rw_write_lock(&self.idle, |lock| {
            *lock = idle;
        })

    }

    pub fn get_idle(&self) -> AfkStatus {

        padlock::rw_read_lock(&self.idle, |lock| {
            *lock
        })

    }

    pub fn send_data(&self, to_client: ToClient) -> BariumResult<()> {

        let data = bincode::serialize(&to_client).unwrap();

        padlock::mutex_lock(&self.stream, |lock| {
            lock.write_all(&data[..])
        })?;

        Ok(())

    }

}
