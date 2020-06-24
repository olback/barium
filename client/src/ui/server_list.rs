use {
    crate::{get_obj, resource, servers::{Server, Servers}, error::BariumResult,
    services::{connect, ServerStatus}},
    super::{friends_list::FriendsList, certificate_window::CertificateWindow},
    std::{rc::Rc, cell::RefCell},
    gtk::{Builder, Label, ListBox, Image, Box as gBox, ListBoxRow, Widget,
    Orientation, EventBox, Menu, MenuItem, prelude::*},
    gdk::prelude::*,
    glib::clone,
    barium_shared::{ToClient, ToServer, UserHash, EncryptedMessage, hash::sha3_256},
    std::sync::mpsc,
    log::{debug, info}
};

#[derive(Debug)]
pub struct ServerRow {
    hash: UserHash,
    row: ListBoxRow,
    status: Rc<RefCell<ServerStatus>>,
    address: String,
    port: u16,
    friends_list: FriendsList,
    certificate_window: Rc<CertificateWindow>,
    cert: Rc<RefCell<Option<Vec<u8>>>>,
    msg_rx: glib::Receiver<ToClient>,
    msg_tx: mpsc::Sender<ToServer>
}

impl ServerRow {

    pub fn new(
        friends_list_box: ListBox,
        certificate_window: Rc<CertificateWindow>,
        server: Server
    ) -> Self {

        let status_icon = Image::new_from_resource(resource!("icons/server-offline.svg"));
        let name = Label::new(Some(&server.name));
        let unread = Label::new(Some("●"));
        let unread_ctx = unread.get_style_context();
        unread_ctx.add_class("fade-2s");
        unread_ctx.add_class("unread-dot");
        let hbox = gBox::new(Orientation::Horizontal, 6);
        hbox.set_margin_top(6);
        hbox.set_margin_bottom(6);
        hbox.set_margin_start(6);
        hbox.set_margin_end(6);
        hbox.add(&status_icon);
        hbox.add(&name);
        hbox.add(&unread);
        let evt_box = EventBox::new();
        evt_box.add(&hbox);
        let row = ListBoxRow::new();
        row.add(&evt_box);

        let connection = connect(
            server.address.clone(),
            server.port,
            server.allow_invalid_cert,
            server.user_id,
            server.password
        );

        let inner = Self {
            hash: sha3_256(&server.user_id),
            row: row,
            status: Rc::new(RefCell::new(ServerStatus::Offline)),
            address: server.address,
            port: server.port,
            friends_list: FriendsList::new(friends_list_box.clone()), // TODO:
            certificate_window: certificate_window,
            cert: Rc::new(RefCell::new(None)),
            msg_rx: connection.msg_rx,
            msg_tx: connection.msg_tx
        };

        evt_box.connect_button_press_event(clone!(
            @strong inner.cert as cert,
            @strong inner.certificate_window as cw
        => move |_, evt_btn| {

            let btn_id = evt_btn.get_button();

            if btn_id == 1 { // Left click

            } else if btn_id == 3 { // Right click

                debug!("0");
                let view_cert_item = MenuItem::new_with_label("View Certificate");
                debug!("1");
                if cert.borrow().is_none() {
                    view_cert_item.set_sensitive(false);
                } else {
                    debug!("2");
                    view_cert_item.connect_activate(clone!(@strong cert, @strong cw => move |_| {
                        debug!("3");
                        let b = (*cert.borrow()).clone();
                        debug!("4");
                        cw.show(&b.unwrap());
                        debug!("5");
                    }));
                }

                let edit_server_item = MenuItem::new_with_label("Edit");


                let menu = Menu::new();
                menu.add(&view_cert_item);
                menu.add(&edit_server_item);

                menu.show_all();
                menu.popup_at_pointer(None);

            }

            Inhibit(false)

        }));

        connection.server_status_rx.attach(None, clone!(
            @strong inner.status as status,
            @strong status_icon
        => move |server_status| {

            match server_status {
                ServerStatus::Online => status_icon.set_from_resource(Some(resource!("icons/server-online.svg"))),
                ServerStatus::Offline => status_icon.set_from_resource(Some(resource!("icons/server-offline.svg"))),
            }

            status.replace(server_status);

            Continue(true)

        }));

        connection.cert_rx.attach(None, clone!(@strong inner.cert as cert => move |der_bytes| {

            debug!("Got cert");

            cert.replace(Some(der_bytes));
            Continue(true)

        }));

        inner

    }

}

#[derive(Debug)]
pub struct ServerList {
    pub keys_ready: Rc<RefCell<bool>>,
    pub certificate_window: Rc<CertificateWindow>,
    pub servers_list_box: ListBox,
    pub friends_list_box: ListBox,
    pub servers: RefCell<Vec<ServerRow>>
}

impl ServerList {

    pub fn build(builder: &Builder, keys_ready: Rc<RefCell<bool>>) -> BariumResult<Self> {

        let inner = Self {
            keys_ready: keys_ready,
            certificate_window: Rc::new(CertificateWindow::build(get_obj!(builder, "main_window"))?),
            servers_list_box: get_obj!(builder, "server_list"),
            friends_list_box: get_obj!(builder, "friends_list"),
            servers: RefCell::new(Vec::new())
        };

        inner.clear();

        Ok(inner)

    }

    pub fn clear(&self) {

        self.servers_list_box.foreach(clone!(@strong self.servers_list_box as servers_list_box => move |element: &Widget| {
            servers_list_box.remove(element);
        }));

    }

    pub fn update(&self, servers: &Servers) {

        if *self.keys_ready.borrow() {

            // info!("Updating Ui");

            // Add added servers
            for server in servers.iter() {

                if !self.exists(&server.address, &server.port) {

                    self.add(server.clone());
                    self.servers_list_box.show_all();

                }

            }

            // Remove removed servers
            for server in &*self.servers.borrow() {

                if servers.find(&server.address, &server.port).is_err() {

                    self.remove(&server.address, &server.port);
                    self.servers_list_box.show_all();

                }

            }

        }

    }

    fn add(&self, server: Server) {

        info!("Adding server");

        let row = ServerRow::new(
            self.friends_list_box.clone(),
            Rc::clone(&self.certificate_window),
            server
        );
        self.servers_list_box.add(&row.row);
        self.servers.borrow_mut().push(row);

    }

    fn remove(&self, address: &String, port: &u16) {

    }

    fn exists(&self, address: &String, port: &u16) -> bool {

        for server in &*self.servers.borrow() {

            if &server.address == address && &server.port == port {
                return true
            }

        }

        false

    }

}
