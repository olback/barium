use {
    std::{rc::Rc, cell::RefCell, sync::{Arc, Mutex}},
    crate::{get_obj, error::BariumResult, servers::Servers},
    gtk::Builder,
    glib::{clone, Continue},
    padlock
};

mod initial_setup;
mod chat_input;
mod chat_feed;
mod friends_list;
mod server_list;
mod certificate_window;

#[derive(Debug)]
pub struct Ui {
    pub main_window: gtk::ApplicationWindow,
    pub initial_setup: initial_setup::InitialSetup,
    pub chat_input: chat_input::ChatInput,
    pub chat_feed: chat_feed::ChatFeed,
    pub server_list: Rc<server_list::ServerList>
}

impl Ui {

    pub fn build(builder: &Builder, keys_ready: Rc<RefCell<bool>>, servers: Arc<Mutex<Servers>>) -> BariumResult<Self> {

        let inner = Self {
            main_window: get_obj!(builder, "main_window"),
            initial_setup: initial_setup::InitialSetup::build(&builder, Arc::clone(&servers))?,
            chat_input: chat_input::ChatInput::build(&builder)?,
            chat_feed: chat_feed::ChatFeed::build(&builder)?,
            server_list: Rc::new(server_list::ServerList::build(&builder, keys_ready)?)
        };

        inner.chat_feed.clear();
        inner.chat_feed.add_row(chat_feed::ChatTypes::IncommingPoke("Friend 2".into()));
        inner.chat_feed.add_row(chat_feed::ChatTypes::IncommingMessage("Friend 2".into(), "This is a <b>bold</b> body!".into()));
        inner.chat_feed.add_row(chat_feed::ChatTypes::OutgoingPoke("Friend 2".into()));
        inner.chat_feed.add_row(chat_feed::ChatTypes::OutgoingMessage("This is a link <a href=\"https://olback.net\">https://olback.net</a> body!".into()));
        inner.chat_feed.add_row(chat_feed::ChatTypes::Error("Friend 2 is offline".into()));

        inner.server_list.clear();

        // inner.server_list.update(servers: Servers)
        gtk::timeout_add(100, clone!(
            @strong servers,
            @strong inner.server_list as server_list
        => move || {

            let servers_clone = padlock::mutex_lock(&servers, |lock: &mut Servers| lock.clone());
            server_list.update(servers_clone);

            Continue(true)

        }));

        Ok(inner)

    }

}
