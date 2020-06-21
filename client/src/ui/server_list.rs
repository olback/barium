use {
    crate::{get_obj, servers::{Server, Servers}},
    gtk::{Builder, ListBox, ListBoxRow, prelude::*},
    glib::clone,
    barium_shared::EncryptedMessage,
    std::sync::mpsc
};

pub struct ServerRow {
    row: ListBoxRow,
    name: String,
    address: String,
    port: u16,
    cert: Option<Vec<u8>>,
    cert_rx: glib::Receiver<Vec<u8>>,
    status_rx: glib::Receiver<()>,
    msg_rx: glib::Receiver<EncryptedMessage>,
    msg_tx: mpsc::Sender<EncryptedMessage>
}

pub struct ServerList {
    pub list_box: ListBox,
    pub servers: Vec<ServerRow>
}

impl ServerList {

    pub fn new(builder: &Builder) -> Self {

        Self {
            list_box: get_obj!(builder, "server_list"),
            servers: Vec::new()
        }

    }

    pub fn update(&self, servers: Servers) {

    }

    fn add(&self, server: Server) {

    }

    fn remove(&self, server: Server) {

    }

}
