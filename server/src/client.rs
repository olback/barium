use std::{
    collections::HashMap,
    sync::{
        Arc,
        Mutex,
        RwLock,
        mpsc
    }
};
use padlock;

pub type Clients = Arc<RwLock<HashMap<[u8; 32], Client>>>;

pub struct Client {
    idle: RwLock<u32>,
    sender: Mutex<mpsc::Sender<Vec<u8>>>
}

impl Client {

    pub fn new(idle: u32, sender: mpsc::Sender<Vec<u8>>) -> Self {

        Self {
            idle: RwLock::new(idle),
            sender: Mutex::new(sender)
        }

    }

    pub fn set_idle(&self, idle: u32) {

        padlock::rw_write_lock(&self.idle, |lock| {
            *lock += idle;
        })

    }

    pub fn get_idle(&self) -> u32 {

        padlock::rw_read_lock(&self.idle, |lock| {
            *lock
        })

    }

    pub fn send_data(&self, data: Vec<u8>) {

        padlock::mutex_lock(&self.sender, |lock| {
            let _ = lock.send(data);
        })

    }

}
