use {
    std::{rc::Rc, cell::RefCell},
    gtk::{Label, Image, Box as gBox, ListBox, ListBoxRow, EventBox, Orientation, prelude::*},
    crate::{resource, servers::{Server, Friend}},
    glib::clone
};

#[derive(Debug)]
pub struct FriendRow {
    pub row: ListBoxRow,
    inner: Friend
}

impl FriendRow {

    pub fn new(friend: Friend) -> Self {

        // FIXME: Away time label

        let status_icon = Image::new_from_resource(resource!("icons/user-offline.svg"));
        let name = Label::new(Some(&friend.display_name));
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

        Self {
            row: row,
            inner: friend
        }

    }

}

#[derive(Debug)]
pub struct FriendsList {
    list_box: ListBox,
    friends: RefCell<Vec<FriendRow>>
}

impl FriendsList {

    pub fn new(friends_list_box: ListBox) -> Self {

        // TODO:

        Self {
            list_box: friends_list_box,
            friends: RefCell::new(Vec::new())
        }

    }

    pub fn show(&self) {

        todo!()

    }

    pub fn has_unread(&self) -> bool {

        todo!()

    }

    pub fn set_unread(&self, state: bool) {

        todo!()

    }

    pub fn add_friend(&self) {

        todo!()

    }

    pub fn remove_friend(&self) {

        todo!()

    }

}
