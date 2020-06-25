use {
    crate::{get_obj, resource, servers::{Server, Servers}, error::BariumResult,
    services::{connect, ServerStatus}, utils::clone_inner},
    super::{friends_list::FriendsList, certificate_dialog::CertificateDialog},
    std::{rc::Rc, cell::RefCell, sync::mpsc},
    gtk::{Builder, Label, ListBox, Image, Box as gBox, ListBoxRow, Widget,
    Orientation, EventBox, Menu, MenuItem, prelude::*},
    gdk::prelude::*,
    glib::clone,
    barium_shared::{ToClient, ToServer, UserHash, EncryptedMessage, hash::sha3_256},
    clipboard::{ClipboardProvider, ClipboardContext},
    base62,
    log::{debug, info},
};

#[derive(Debug)]
pub struct ServerRow {
    hash: UserHash,
    row: ListBoxRow,
    menu: Menu,
    status: Rc<RefCell<ServerStatus>>, // TODO: Not needed?
    address: String,
    port: u16,
    friends_list: FriendsList,
    certificate_window: Rc<CertificateDialog>, // TODO: Not needed
    cert: Rc<RefCell<Option<Vec<u8>>>>,
    msg_rx: glib::Receiver<ToClient>,
    msg_tx: mpsc::Sender<ToServer>
}

impl ServerRow {

    pub fn new(
        friends_list_box: ListBox,
        certificate_window: Rc<CertificateDialog>,
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
            menu: Menu::new(),
            status: Rc::new(RefCell::new(ServerStatus::Offline)),
            address: server.address,
            port: server.port,
            friends_list: FriendsList::new(friends_list_box.clone()), // TODO:
            certificate_window: certificate_window,
            cert: Rc::new(RefCell::new(None)),
            msg_rx: connection.msg_rx,
            msg_tx: connection.msg_tx
        };

        // Server right-click menu
        let copy_identity_item = MenuItem::new_with_label("Copy my Identity");
        let view_cert_item = MenuItem::new_with_label("View Certificate");
        let edit_server_item = MenuItem::new_with_label("Edit");
        inner.menu.add(&copy_identity_item);
        inner.menu.add(&view_cert_item);
        inner.menu.add(&edit_server_item);
        copy_identity_item.connect_activate(clone!(
            @strong inner.hash as hash
        => move |_| {
            let b62_hash = base62::encode(&hash);
            let mut ctx: ClipboardContext = match ClipboardProvider::new() {
                Ok(c) => c,
                Err(_) => return
            };
            drop(ctx.set_contents(b62_hash));
        }));
        view_cert_item.connect_activate(clone!(
            @strong inner.cert as cert,
            @strong inner.certificate_window as cw
         => move |_| {
            let b = clone_inner(&*cert);
            cw.show(&b.unwrap());
        }));
        // edit_server_item.connect_activate(clone!(
        //     @strong inner.edit_server_dialog as edit_server_dialog
        // => move |_| {

        // }));

        evt_box.connect_button_press_event(clone!(
            @strong inner.cert as cert,
            @strong inner.menu as menu
        => move |_, evt_btn| {

            let btn_id = evt_btn.get_button();

            if btn_id == 1 { // Left click

            } else if btn_id == 3 { // Right click

                match *cert.borrow() {
                    Some(_) => view_cert_item.set_sensitive(true),
                    None => view_cert_item.set_sensitive(false)
                }

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
    pub certificate_window: Rc<CertificateDialog>,
    pub servers_list_box: ListBox,
    pub friends_list_box: ListBox,
    pub servers: RefCell<Vec<ServerRow>>
}

impl ServerList {

    pub fn build(builder: &Builder, keys_ready: Rc<RefCell<bool>>) -> BariumResult<Self> {

        let inner = Self {
            keys_ready: keys_ready,
            certificate_window: Rc::new(CertificateDialog::build(get_obj!(builder, "main_window"))?),
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

                if servers.find(&server.address, &server.port).is_none() {

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
