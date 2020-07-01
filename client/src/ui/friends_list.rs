use {
    std::{rc::Rc, cell::RefCell, sync::mpsc::Sender},
    gtk::{Label, Image, Box as gBox, ListBox, ListBoxRow, EventBox, Orientation, prelude::*},
    crate::{resource, servers::{Server, Friend}, utils::format_time, ui::chat_feed::ChatTypes},
    barium_shared::{AfkStatus, ToServer, UserHash, EncryptedMessage, KeyBust},
    rsa::RSAPublicKey,
    log::{debug, info},
    glib::clone
};

#[derive(Debug)]
pub struct FriendRow {
    row: ListBoxRow,
    status_icon: Image,
    afk_label: Label,
    afk_status: RefCell<AfkStatus>,
    inner: Friend,
    public_key: RefCell<Option<RSAPublicKey>>,
    key_bust: RefCell<KeyBust>,
    chat_history: RefCell<Vec<ChatTypes>>
}

impl FriendRow {

    pub fn new(friend: Friend) -> Self {

        let status_icon = Image::new_from_resource(resource!("icons/user-offline.svg"));
        let name = Label::new(Some(&friend.display_name));
        let afk = Label::new(None);
        afk.get_style_context().add_class("dim-label");
        afk.set_xalign(0f32);
        let unread = Label::new(Some("●"));
        let unread_ctx = unread.get_style_context();
        let vbox = gBox::new(Orientation::Vertical, 0);
        vbox.add(&name);
        vbox.add(&afk);
        unread_ctx.add_class("fade-2s");
        unread_ctx.add_class("unread-dot");
        let hbox = gBox::new(Orientation::Horizontal, 6);
        hbox.set_margin_top(6);
        hbox.set_margin_bottom(6);
        hbox.set_margin_start(6);
        hbox.set_margin_end(6);
        hbox.add(&status_icon);
        hbox.add(&vbox);
        hbox.add(&unread);
        let evt_box = EventBox::new();
        evt_box.add(&hbox);
        let row = ListBoxRow::new();
        row.add(&evt_box);

        let inner = Self {
            row: row,
            status_icon: status_icon,
            afk_label: afk,
            afk_status: RefCell::new(AfkStatus::Offline),
            inner: friend,
            public_key: RefCell::new(None),
            key_bust: RefCell::new(KeyBust::default()),
            chat_history: RefCell::new(Vec::new())
        };

        inner.set_away_status(AfkStatus::Offline);

        inner

    }

    pub fn set_away_status(&self, status: AfkStatus) {

        self.afk_status.replace(status);

    }

}

#[derive(Debug)]
pub struct FriendsList {
    list_box: ListBox,
    friends: RefCell<Vec<FriendRow>>
}

impl FriendsList {

    pub fn new(friends_list_box: ListBox) -> Self {

        Self {
            list_box: friends_list_box,
            friends: RefCell::new(Vec::new())
        }

    }

    pub fn show(&self) {

        self.list_box.foreach(|widget| {
            self.list_box.remove(widget);
        });

        for friend in &*self.friends.borrow() {
            self.list_box.add(&friend.row);
        }

    }

    pub fn friends_exists(&self, hash: &UserHash) -> bool {

        for friend in &*self.friends.borrow() {

            if &friend.inner.hash == hash {
                return true
            }

        }

        false

    }

    pub fn update(&self, fs_server: &Server) {

        // Add added servers
        for fs_friend in fs_server.friends.iter() {

            if !self.friends_exists(&fs_friend.hash) {

                self.add_friend(fs_friend.clone());
                self.list_box.show_all();

            }

        }

        // Remove removed servers
        // * SAFETY: This is safe as long as self.remove() loops backwards
        // * This also has to loop backwards to avoid looping into an index
        // * that has been removed.
        let borrow = unsafe { self.friends.try_borrow_unguarded().unwrap() };
        for ui_friend in borrow.iter().rev() {

            if fs_server.find_friend(&ui_friend.inner.hash).is_none() {

                self.remove_friend(&ui_friend.inner.hash);
                self.list_box.show_all();

            }

        }

        // debug!("Update friends status");
        for friend in borrow.iter() {

            match *friend.afk_status.borrow() {

                AfkStatus::Available => {
                    friend.status_icon.set_from_resource(Some(resource!("icons/user-online.svg")));
                    friend.afk_label.set_text("Available");
                },

                AfkStatus::Away(time) => {
                    friend.status_icon.set_from_resource(Some(resource!("icons/user-away.svg")));
                    match time {
                        Some(seconds) => friend.afk_label.set_text(&format_time(seconds as u64)),
                        None => friend.afk_label.set_text("Away")
                    }
                },

                AfkStatus::DoNotDisturb => {
                    friend.status_icon.set_from_resource(Some(resource!("icons/user-dnd.svg")));
                    friend.afk_label.set_text("Do Not Disturb");
                },

                AfkStatus::Offline => {
                    friend.status_icon.set_from_resource(Some(resource!("icons/user-offline.svg")));
                    friend.afk_label.set_text("Offline");
                }

            }

        }

        self.list_box.show_all();

    }

    pub fn handle_error(&self, user: UserHash, error: String) {

        for friend in &*self.friends.borrow() {

            if friend.inner.hash == user {
                friend.chat_history.borrow_mut().push(ChatTypes::Error(error));
                return
            }

        }

    }

    pub fn handle_friends_online(&self, friends_status: Vec<(UserHash, KeyBust, AfkStatus)>) -> Vec<UserHash> {

        self.set_all_offline();

        let ui_friends = &mut*self.friends.borrow_mut();

        let mut key_requests = Vec::<UserHash>::new();

        for fsf in friends_status {

            for uif in ui_friends.iter_mut() {

                if fsf.0 == uif.inner.hash {

                    if *uif.key_bust.borrow() != fsf.1 {
                        key_requests.push(uif.inner.hash);
                    }

                    uif.set_away_status(fsf.2);

                }

            }

        }

        key_requests

    }

    pub fn handle_message(&self, msg: EncryptedMessage) {

    }

    pub fn handle_keys(&self, keys: Vec<(UserHash, KeyBust, RSAPublicKey)>) {

        for (hash, bust, key) in keys {

            for friend in &*self.friends.borrow() {

                if friend.inner.hash == hash {

                    friend.public_key.replace(Some(key));
                    friend.key_bust.replace(bust);
                    break;

                }

            }

        }

    }

    pub fn has_unread(&self) -> bool {

        todo!()

    }

    pub fn set_unread(&self, state: bool) {

        todo!()

    }

    pub fn add_friend(&self, friend: Friend) {

        info!("Adding friend");

        let row = FriendRow::new(friend);
        let mut friends = self.friends.borrow_mut();
        self.list_box.add(&row.row);
        friends.push(row);

    }

    pub fn remove_friend(&self, hash: &UserHash) {

        todo!()

    }

    pub fn get_friend_hashes(&self) -> Vec<UserHash> {

        let friends = &*self.friends.borrow();
        let mut hashes = Vec::<UserHash>::with_capacity(friends.len());
        for f in friends {
            hashes.push(f.inner.hash);
        }

        hashes

    }

    pub fn set_all_offline(&self) {

        for friend in &*self.friends.borrow() {
            friend.set_away_status(AfkStatus::Offline);
        }

    }

}
