use {
    std::{rc::Rc, cell::RefCell},
    gtk::{Label, Image, Box as gBox, ListBox, prelude::*},
    crate::servers::Friend,
    glib::clone
};

#[derive(Debug)]
pub struct FriendRow {

}

impl FriendRow {

    pub fn new(friend: Friend) -> Self {

        todo!()

    }

}

#[derive(Debug)]
pub struct FriendsList {
    list_box: ListBox,
    selected_index: Rc<RefCell<Option<usize>>>,
    friends: Vec<FriendRow>
}

impl FriendsList {

    pub fn new(friends_list_box: ListBox) -> Self {

        // TODO:

        Self {
            list_box: friends_list_box,
            selected_index: Rc::new(RefCell::new(None)),
            friends: Vec::new()
        }

    }

    pub fn has_unread(&self) -> bool {

        todo!()

    }

}
