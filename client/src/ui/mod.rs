use {
    std::sync::{Arc, Mutex},
    crate::{get_obj, error::BariumResult, fs::Servers},
    gtk::Builder
};

mod initial_setup;
mod chat_input;
mod chat_feed;

#[derive(Debug)]
pub struct Ui {
    pub main_window: gtk::ApplicationWindow,
    pub initial_setup: initial_setup::InitialSetup,
    pub chat_input: chat_input::ChatInput,
    pub chat_feed: chat_feed::ChatFeed,
}

impl Ui {

    pub fn build(builder: &Builder, servers: Arc<Mutex<Servers>>) -> BariumResult<Self> {

        let inner = Self {
            main_window: get_obj!(builder, "main_window"),
            initial_setup: initial_setup::InitialSetup::build(&builder, Arc::clone(&servers))?,
            chat_input: chat_input::ChatInput::build(&builder)?,
            chat_feed: chat_feed::ChatFeed::build(&builder)?,
        };

        inner.chat_feed.clear();
        inner.chat_feed.add_row(chat_feed::ChatTypes::IncommingPoke("Friend 2".into()));
        inner.chat_feed.add_row(chat_feed::ChatTypes::IncommingMessage("Friend 2".into(), "This is a <b>bold</b> body!".into()));
        inner.chat_feed.add_row(chat_feed::ChatTypes::OutgoingPoke("Friend 2".into()));
        inner.chat_feed.add_row(chat_feed::ChatTypes::OutgoingMessage("This is a link <a href=\"https://olback.net\">https://olback.net</a> body!".into()));

        Ok(inner)

    }

}
