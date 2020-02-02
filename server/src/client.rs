use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
        RwLock
    }
};
use tokio::sync::mpsc;
use padlock;
use barium_shared::{AfkStatus, ToClient};
use crate::utils::sha3_256;
use bincode;

pub type Clients = Arc<RwLock<HashMap<[u8; 32], Client>>>;

#[derive(Debug)]
pub struct Client {
    hash: [u8; 32],
    idle: RwLock<AfkStatus>,
    sender: Mutex<mpsc::Sender<Vec<u8>>>
}

impl Client {

    pub fn new(id: [u8; 32], idle: AfkStatus, sender: mpsc::Sender<Vec<u8>>) -> Self {

        Self {
            hash: sha3_256(id),
            idle: RwLock::new(idle),
            sender: Mutex::new(sender)
        }

    }

    pub fn get_hash(&self) -> [u8; 32] {
        self.hash
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

    pub fn send_data(&self, to_client: ToClient) {

        let data = bincode::serialize(&to_client).unwrap();

        padlock::mutex_lock(&self.sender, |lock| {
            lock.send(data);
        })

    }

}
