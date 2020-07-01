use {
    crate::{get_obj, resource, servers::{Server, Servers, ComparableServer}, error::BariumResult,
    services::{connect, ServerStatus, IdleTracker}, utils::clone_inner, MainStack},
    super::{friends_list::FriendsList, certificate_dialog::CertificateDialog, edit_server_dialog::EditServerDialog},
    std::{rc::Rc, cell::RefCell, sync::{Arc, Mutex, mpsc}},
    gtk::{Builder, Label, ListBox, Image, Box as gBox, ListBoxRow, Widget,
    Orientation, EventBox, Menu, MenuItem, prelude::*},
    glib::{clone, MainContext, Priority},
    barium_shared::{AfkStatus, ToClient, ToServer, UserHash, EncryptedMessage, hash::sha3_256},
    clipboard::{ClipboardProvider, ClipboardContext},
    base62,
    log::{debug, info, warn, error},
    lazy_static::lazy_static
};

lazy_static! {
    pub static ref IDLE_TRACKER: IdleTracker = IdleTracker::new();
}

#[derive(Debug)]
pub struct ServerRow {
    pub hash: UserHash,
    pub server: Rc<Server>,
    pub row: ListBoxRow,
    pub menu: Menu,
    pub status: Rc<RefCell<ServerStatus>>, // TODO: Not needed?
    pub friends_list: Rc<FriendsList>,
    pub certificate_window: Rc<CertificateDialog>, // TODO: Not needed?
    pub edit_server_dialog: Rc<EditServerDialog>,
    pub cert: Rc<RefCell<Option<Vec<u8>>>>,
    pub msg_tx: mpsc::Sender<ToServer>
}

impl ServerRow {

    pub fn new(
        friends_list_box: ListBox,
        certificate_window: Rc<CertificateDialog>,
        edit_server_dialog: Rc<EditServerDialog>,
        // edit_friend_dialog: Rc<EditFriendDialog>,
        // main_stack: Rc<MainStack>,
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
            server.password.clone()
        );

        let inner = Self {
            hash: sha3_256(&server.user_id),
            server: Rc::new(server),
            row: row,
            menu: Menu::new(),
            status: Rc::new(RefCell::new(ServerStatus::Offline)),
            friends_list: Rc::new(FriendsList::new(friends_list_box)), // TODO:
            certificate_window: certificate_window,
            edit_server_dialog: edit_server_dialog,
            cert: Rc::new(RefCell::new(None)),
            msg_tx: connection.msg_tx
        };

        let (keep_alive_tx, keep_alive_rx) = MainContext::channel::<u64>(Priority::default());
        keep_alive_rx.attach(None, clone!(
            @strong inner.server as server,
            @strong inner.friends_list as friends_list,
            @strong inner.msg_tx as msg_tx
        => move |idle_time| {

            let hashes = friends_list.get_friend_hashes();

            let afk_staus = if idle_time >= 300 {
                AfkStatus::Away(Some(idle_time))
            } else {
                AfkStatus::Available
            };

            let msg = ToServer::KeepAlive(server.user_id, hashes, afk_staus);

            match msg_tx.send(msg) {
                Ok(_) => Continue(true),
                Err(_) => Continue(false)
            }

        }));
        drop(keep_alive_tx.send(0));
        IDLE_TRACKER.add(keep_alive_tx);

        connection.msg_rx.attach(None, clone!(
            @strong inner.server as server,
            @strong inner.friends_list as friends_list,
            @strong inner.msg_tx as msg_tx
        => move |to_client| {

            match to_client {

                ToClient::Error(user, error) => match user {
                    Some(u) => friends_list.handle_error(u, error),
                    None => todo!("{}", error)
                }

                ToClient::FriendsOnline(users_online) => {
                    let key_requests = friends_list.handle_friends_online(users_online);
                    if key_requests.len() > 0 {
                        debug!("Requesting keys");
                        match msg_tx.send(ToServer::GetPublicKeys(server.user_id, key_requests)) {
                            Ok(_) => debug!("Keys requested"),
                            Err(e) => {
                                error!("{}", e);
                                return Continue(false);
                            }
                        }
                    }
                },

                ToClient::Message(msg) => {
                    friends_list.handle_message(msg);
                },

                ToClient::PublicKeys(keys) => {
                    debug!("Got {} keys", keys.len());
                    friends_list.handle_keys(keys);
                    debug!("{:#?}", friends_list);
                },

                _ => warn!("Invalid message") // Do nothing

            }

            Continue(true)

        }));

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
        edit_server_item.connect_activate(clone!(
            @strong inner.edit_server_dialog as edit_server_dialog,
            @strong inner.server as server
        => move |_| {
            edit_server_dialog.show(&server.as_comparable())
        }));

        evt_box.connect_button_press_event(clone!(
            @strong inner.server as server,
            @strong inner.friends_list as friends_list,
            @strong inner.cert as cert,
            @strong inner.menu as menu
        => move |_, evt_btn| {

            let btn_id = evt_btn.get_button();

            if btn_id == 1 { // Left click

                friends_list.show()

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

    pub fn update_friends(&self) {

        self.friends_list.update(&self.server);

    }

}

#[derive(Debug)]
pub struct ServerList {
    pub keys_ready: Rc<RefCell<bool>>,
    pub certificate_window: Rc<CertificateDialog>,
    pub edit_server_dialog: Rc<EditServerDialog>,
    pub servers_list_box: ListBox,
    pub servers_selected_index: Rc<RefCell<Option<usize>>>,
    pub friends_list_box: ListBox, // TODO: not needed?
    servers: RefCell<Vec<ServerRow>>
}

impl ServerList {

    pub fn build(
        builder: &Builder,
        keys_ready: Rc<RefCell<bool>>,
        fs_servers: Arc<Mutex<Servers>>
    ) -> BariumResult<Self> {

        let inner = Self {
            keys_ready: keys_ready,
            certificate_window: Rc::new(CertificateDialog::build(&builder)?),
            edit_server_dialog: Rc::new(EditServerDialog::build(&builder, fs_servers)?),
            servers_list_box: get_obj!(builder, "server_list"),
            servers_selected_index: Rc::new(RefCell::new(None)), // TODO:
            friends_list_box: get_obj!(builder, "friends_list"),
            servers: RefCell::new(Vec::new())
        };

        // inner.servers_list_box.connect_row_selected(|lbox, row| {

        //     let lbox_children = lbox.get_children();

        //     if let Some(row) = row {

        //         for i in 0..lbox_children.len() {
        //             if lbox_children[i] == row.clone().upcast::<Widget>() {
        //                 debug!("Index: {}", i);
        //             }
        //         }

        //     }

        // });

        inner.clear_servers();

        inner.friends_list_box.foreach(|widget| {
            inner.friends_list_box.remove(widget);
        });

        Ok(inner)

    }

    pub fn clear_servers(&self) {

        self.servers_list_box.foreach(clone!(@strong self.servers_list_box as servers_list_box => move |element: &Widget| {
            servers_list_box.remove(element);
        }));

    }

    pub fn update(&self, fs_servers: &Servers) {

        if *self.keys_ready.borrow() {

            // info!("Updating Ui");

            // Add added servers
            for server in fs_servers.iter() {

                if !self.exists(&server.as_comparable()) {

                    self.add(server.clone());
                    self.servers_list_box.show_all();

                }

            }

            // Remove removed servers
            // * SAFETY: This is safe as long as self.remove() loops backwards
            // * This also has to loop backwards to avoid looping into an index
            // * that has been removed.
            let borrow = unsafe { self.servers.try_borrow_unguarded().unwrap() };
            for ui_server in borrow.iter().rev() {

                if fs_servers.find(&ui_server.server.as_comparable()).is_none() {

                    self.remove(&ui_server.server.as_comparable());
                    self.servers_list_box.show_all();

                }

            }

            for server in &*self.servers.borrow() {
                server.update_friends();
            }

        }


    }

    fn add(&self, server: Server) {

        info!("Adding server");

        let row = ServerRow::new(
            self.friends_list_box.clone(),
            Rc::clone(&self.certificate_window),
            Rc::clone(&self.edit_server_dialog),
            server
        );
        self.servers_list_box.add(&row.row);
        self.servers.borrow_mut().push(row);

    }

    fn remove(&self, other: &ComparableServer) {

        let mut ui_servers = self.servers.borrow_mut();

        // * SAFETY: This has to loop backwards to make
        // * the unsafe part of self.update() not panic.
        for i in (0..ui_servers.len()).rev() {

            if &ui_servers[i].server.as_comparable() == other {

                let select_new = ui_servers[i].row.is_selected() && ui_servers.len() > 1;

                self.servers_list_box.remove(&ui_servers[i].row);
                ui_servers.remove(i);

                if select_new {
                    self.servers_list_box.select_row(Some(&ui_servers[0].row));
                }

            }

        }

    }

    fn exists(&self, other: &ComparableServer) -> bool {

        for ui_server in &*self.servers.borrow() {

            if &ui_server.server.as_comparable() == other {
                return true
            }

        }

        false

    }

}
