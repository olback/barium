use {
    std::rc::Rc,
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
    pub list_box: ListBox,
    friends: Vec<FriendRow>
}

impl FriendsList {

    pub fn new(friends_list_box: ListBox) -> Self {

        // TODO:

        Self {
            list_box: friends_list_box,
            friends: Vec::new()
        }

    }

}
