use {
    std::{rc::Rc, cell::RefCell, sync::{Arc, Mutex}},
    crate::{get_obj, error::BariumResult, servers::Servers},
    gtk::{Builder, prelude::*},
    gio::{SimpleAction, SimpleActionGroup, prelude::*},
    glib::{clone, Continue},
    padlock
};

mod initial_setup;
mod chat_input;
mod chat_feed;
mod friends_list;
mod server_list;
mod certificate_dialog;
mod add_friend_dialog;
mod add_server_dialog;
mod edit_server_dialog;

pub type ServerIdentity = (String, u16);

#[derive(Debug)]
pub struct Ui {
    pub main_window: gtk::ApplicationWindow,
    pub initial_setup: initial_setup::InitialSetup,
    pub chat_input: chat_input::ChatInput,
    pub chat_feed: chat_feed::ChatFeed,
    pub server_list: Rc<server_list::ServerList>,
    pub add_friend_dialog: Rc<add_friend_dialog::AddFriendDialog>,
    // pub edit_friend_dialog: Rc<edit_friend_dialog::EditFriendDialog>,
    pub add_server_dialog: Rc<add_server_dialog::AddServerDialog>
}

impl Ui {

    pub fn build(builder: &Builder, keys_ready: Rc<RefCell<bool>>, servers: Arc<Mutex<Servers>>) -> BariumResult<Self> {

        let inner = Self {
            main_window: get_obj!(builder, "main_window"),
            initial_setup: initial_setup::InitialSetup::build(&builder, Arc::clone(&servers))?,
            chat_input: chat_input::ChatInput::build(&builder)?,
            chat_feed: chat_feed::ChatFeed::build(&builder)?,
            server_list: Rc::new(server_list::ServerList::build(&builder, keys_ready, Arc::clone(&servers))?),
            add_friend_dialog: Rc::new(add_friend_dialog::AddFriendDialog::build(&builder, Arc::clone(&servers))?),
            // ! Remove this line: edit_friend_dialog: Rc::new(edit_friend_dialog::EditFriendDialog::build(&builder, Arc::clone(&servers))?),
            add_server_dialog: Rc::new(add_server_dialog::AddServerDialog::build(&builder, Arc::clone(&servers))?)
        };

        inner.chat_feed.clear();
        inner.chat_feed.add_row(chat_feed::ChatTypes::IncommingPoke("Friend 2".into()));
        inner.chat_feed.add_row(chat_feed::ChatTypes::IncommingMessage("Friend 2".into(), "This is a <b>bold</b> body!".into()));
        inner.chat_feed.add_row(chat_feed::ChatTypes::OutgoingPoke("Friend 2".into()));
        inner.chat_feed.add_row(chat_feed::ChatTypes::OutgoingMessage("This is a link <a href=\"https://olback.net\">https://olback.net</a> body!".into()));
        inner.chat_feed.add_row(chat_feed::ChatTypes::Error("Friend 2 is offline".into()));

        // 'Add' action group
        let add_actions = SimpleActionGroup::new();
        inner.main_window.insert_action_group("add", Some(&add_actions));
        inner.connect_add_friend_dialog(&add_actions, Arc::clone(&servers));
        inner.connect_add_server_dialog(&add_actions);

        // Clear server list
        inner.server_list.clear();

        // Add main loop servers sync
        gtk::timeout_add(100, clone!(
            @strong servers,
            @strong inner.server_list as server_list
        => move || {

            padlock::mutex_lock(&servers, |lock| server_list.update(lock));

            Continue(true)

        }));

        Ok(inner)

    }

    fn connect_add_friend_dialog(&self, ac: &SimpleActionGroup, fs_servers: Arc<Mutex<Servers>>) {

        let open_add_friend_dialog = SimpleAction::new("friend", None);

        open_add_friend_dialog.connect_activate(clone!(
            @strong self.add_friend_dialog as add_friend_dialog
        => move |_, _| add_friend_dialog.show(Arc::clone(&fs_servers))));

        ac.add_action(&open_add_friend_dialog);

    }

    fn connect_add_server_dialog(&self, ac: &SimpleActionGroup) {

        let open_add_server_dialog = SimpleAction::new("server", None);

        open_add_server_dialog.connect_activate(clone!(
            @strong self.add_server_dialog as add_server_dialog
        => move |_, _| add_server_dialog.show()));

        ac.add_action(&open_add_server_dialog);

    }

}
