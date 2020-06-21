use {
    crate::{get_obj, error::BariumResult},
    gtk::{Align, Builder, Box as gBox, Label, ListBoxRow, ListBox, Widget, Orientation, prelude::*},
    glib::clone
};

pub enum ChatTypes {
    IncommingPoke(String),
    OutgoingPoke(String),
    IncommingMessage(String, String),
    OutgoingMessage(String),
    Error(String)
}

#[derive(Clone, Debug)]
pub struct ChatFeed {
    chat_feed: ListBox
}

impl ChatFeed {

    pub fn build(builder: &Builder) -> BariumResult<Self> {

        let inner = Self {
            chat_feed: get_obj!(builder, "chat_feed")
        };

        Ok(inner)

    }

    pub fn clear(&self) {

        self.chat_feed.foreach(clone!(@strong self.chat_feed as chat_feed => move |element: &Widget| {
            chat_feed.remove(element);
        }));

    }

    pub fn add_row(&self, row: ChatTypes) {

        let new_row = match row {
            ChatTypes::IncommingPoke(from) => Self::incomming_poke(from),
            ChatTypes::OutgoingPoke(to) => Self::outgoing_poke(to),
            ChatTypes::IncommingMessage(from, body) => Self::incomming_message(from, body),
            ChatTypes::OutgoingMessage(body) => Self::outgoing_message(body),
            ChatTypes::Error(body) => Self::error(body)
        };

        self.chat_feed.insert(&new_row, -1);

    }

    fn incomming_poke(from: String) -> ListBoxRow {

        Self::poke_row(format!("{} poked you!", from), Align::Start)

    }

    fn outgoing_poke(to: String) -> ListBoxRow {

        Self::poke_row(format!("You poked {}!", to), Align::End)

    }

    fn incomming_message(from: String, body: String) -> ListBoxRow {

        let row = ListBoxRow::new();
        let vbox = Self::message_vbox();

        let from_label = Label::new(Some(from.as_str()));
        let body_label = Label::new(None);
        from_label.set_halign(Align::Start);
        body_label.set_halign(Align::Start);
        body_label.set_markup(body.as_str());
        body_label.set_selectable(true);
        vbox.add(&from_label);
        vbox.add(&body_label);
        row.add(&vbox);
        row.show_all();

        row

    }

    fn outgoing_message(body: String) -> ListBoxRow {

        let row = ListBoxRow::new();
        let vbox = Self::message_vbox();

        let ctx = row.get_style_context();
        ctx.add_class("light-gray-bg");

        let from_label = Label::new(Some("Me"));
        let body_label = Label::new(None);
        from_label.set_halign(Align::End);
        body_label.set_halign(Align::End);
        body_label.set_markup(body.as_str());
        body_label.set_selectable(true);
        vbox.add(&from_label);
        vbox.add(&body_label);
        row.add(&vbox);
        row.show_all();

        row

    }

    fn error(body: String) -> ListBoxRow {

        let row = ListBoxRow::new();

        let ctx = row.get_style_context();
        ctx.add_class("error-row");

        let body = Label::new(Some(format!("Error: {}", body).as_str()));
        body.set_halign(Align::Start);
        body.set_margin_top(12);
        body.set_margin_bottom(12);
        body.set_margin_start(12);
        body.set_margin_end(12);

        row.add(&body);
        row.show_all();

        row

    }

    fn poke_row(body: String, align: Align) -> ListBoxRow {

        let row = ListBoxRow::new();

        let ctx = row.get_style_context();
        ctx.add_class("poke-row");

        let body = Label::new(Some(body.as_str()));
        body.set_halign(align);
        body.set_margin_top(12);
        body.set_margin_bottom(12);
        body.set_margin_start(12);
        body.set_margin_end(12);

        row.add(&body);
        row.show_all();

        row

    }

    fn message_vbox() -> gBox {

        let vbox = gBox::new(Orientation::Vertical, 2);

        vbox.set_spacing(6);
        vbox.set_margin_top(12);
        vbox.set_margin_bottom(12);
        vbox.set_margin_start(12);
        vbox.set_margin_end(12);

        vbox

    }

}
