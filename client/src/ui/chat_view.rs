use {
    crate::{get_obj, error::BariumResult},
    super::{chat_feed::{ChatFeed, ChatTypes}, chat_input::ChatInput},
    gtk::{Align, Builder, Box as gBox, Label, ListBoxRow, ListBox,
    Widget, Orientation, Justification, ScrolledWindow, prelude::*},
    std::{rc::Rc, cell::RefCell},
    pango::WrapMode,
    glib::{clone, MainContext, Sender, Receiver, Priority}
};

#[derive(Debug)]
pub struct ChatView {
    chat_with_label: Label,
    chat_feed: Rc<ChatFeed>,
    chat_input: Rc<ChatInput>,
    msg_tx: Rc<RefCell<Option<Sender<String>>>>,
    chat_view_tx: Sender<ChatTypes>
}

impl ChatView {

    pub fn build(builder: &Builder) -> BariumResult<Self> {

        let (chat_view_tx, msg_rx) = MainContext::channel::<ChatTypes>(Priority::default());

        let inner = Self {
            chat_with_label: get_obj!(builder, "chat_with_label"),
            chat_feed: Rc::new(ChatFeed::build(builder)?),
            chat_input: Rc::new(ChatInput::build(builder)?),
            msg_tx: Rc::new(RefCell::new(None)),
            chat_view_tx: chat_view_tx
        };

        inner.chat_with_label.set_text("");
        inner.chat_feed.clear();

        msg_rx.attach(None, clone!(
            @strong inner.chat_feed as chat_feed
        => move |msg| {
            chat_feed.add_row(&msg);
            Continue(true)
        }));

        inner.chat_input.send_button.connect_clicked(clone!(
            @strong inner.chat_feed as chat_feed,
            @strong inner.chat_input as chat_input,
            @strong inner.msg_tx as msg_tx
        => move |_| {

            let text = chat_input.get_entry();
            if text.trim().len() == 0 {
                return
            }

            match *msg_tx.borrow() {
                Some(ref tx) => match tx.send(text) {
                    Ok(_) => chat_input.set_entry(""),
                    Err(e) => chat_feed.add_row(&ChatTypes::Error(format!("Error: {}", e)))
                },
                None => chat_feed.add_row(&ChatTypes::Error("No recipient selected".into()))
            }

        }));

        Ok(inner)

    }

    pub fn get_chat_view_tx(&self) -> Sender<ChatTypes> {

        self.chat_view_tx.clone()

    }

    pub fn render_chat_feed(&self, with: &str, feed: &[ChatTypes], msg_tx: Sender<String>) {

        self.chat_with_label.set_text(with);
        self.chat_feed.add_rows(feed);
        self.msg_tx.replace(Some(msg_tx));

    }

}
