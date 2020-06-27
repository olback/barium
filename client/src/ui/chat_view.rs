use {
    crate::{get_obj, error::BariumResult},
    super::{chat_feed::{ChatFeed, ChatTypes}, chat_input::ChatInput},
    gtk::{Align, Builder, Box as gBox, Label, ListBoxRow, ListBox,
    Widget, Orientation, Justification, ScrolledWindow, prelude::*},
    std::rc::Rc,
    pango::WrapMode,
    glib::clone
};

#[derive(Debug)]
pub struct ChatView {
    chat_with_label: Label,
    chat_feed: Rc<ChatFeed>,
    chat_input: Rc<ChatInput>
}

impl ChatView {

    pub fn build(builder: &Builder) -> BariumResult<Self> {

        let inner = Self {
            chat_with_label: get_obj!(builder, "chat_with_label"),
            chat_feed: Rc::new(ChatFeed::build(builder)?),
            chat_input: Rc::new(ChatInput::build(builder)?)
        };

        inner.chat_with_label.set_text("");
        inner.chat_feed.clear();

        inner.chat_input.send_button.connect_clicked(clone!(
            @strong inner.chat_feed as chat_feed
        => move |_| {
            let row = ChatTypes::OutgoingMessage("Hello there!".into());
            chat_feed.add_row(&row);
        }));

        Ok(inner)

    }

    pub fn render_chat_feed(&self, with: &str, feed: &[ChatTypes]) {

        self.chat_with_label.set_text(with);
        self.chat_feed.add_rows(feed);

    }

}
