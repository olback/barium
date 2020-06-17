use {
    std::{
        net::{TcpStream},
        sync::{Arc, Mutex, mpsc},
        thread
    },
    glib::{self, MainContext, Priority},
    native_tls::{TlsStream},
    barium_shared::{UserHash}
};

pub enum ToConnectionThread {
    Message(Vec<u8>),
    Stop
}

pub fn connect() -> (glib::Receiver<()>, mpsc::Sender<ToConnectionThread>) {

    todo!()

}
