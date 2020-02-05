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
use barium_shared::{AfkStatus, ToClient};
use crate::error::BariumResult;
use bincode;

pub type Clients = Arc<RwLock<HashMap<[u8; 32], Client>>>;

#[derive(Debug)]
pub struct Client {
    id: [u8; 32],
    idle: RwLock<AfkStatus>,
    stream: Mutex<TcpStream>
}

impl Client {

    pub fn new(id: [u8; 32], idle: AfkStatus, stream: &TcpStream) -> BariumResult<Self> {

        Ok(Self {
            id: id,
            idle: RwLock::new(idle),
            stream: Mutex::new(stream.try_clone()?)
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
