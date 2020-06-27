use {
    crate::{get_obj, error::BariumResult, utils::escape_markdown},
    gtk::{Align, Builder, Box as gBox, Label, ListBoxRow, ListBox,
    Widget, Orientation, Justification, ScrolledWindow, prelude::*},
    pango::WrapMode,
    glib::clone,
    log::debug
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
    chat_feed: ListBox,
    scroll_window: ScrolledWindow
}

impl ChatFeed {

    pub fn build(builder: &Builder) -> BariumResult<Self> {

        let inner = Self {
            chat_feed: get_obj!(builder, "chat_feed"),
            scroll_window: get_obj!(builder, "chat_scroll_window")
        };

        Ok(inner)

    }

    pub fn clear(&self) {

        self.chat_feed.foreach(clone!(@strong self.chat_feed as chat_feed => move |element: &Widget| {
            chat_feed.remove(element);
        }));

    }

    pub fn add_row(&self, row: &ChatTypes) {

        // TODO: Only scroll to bottom if already at the bottom-ish
        let scroll_to_bottom = match self.scroll_window.get_vadjustment() {
            Some(adj) => {
                let height = self.scroll_window.get_allocated_height() as f64;
                let value = adj.get_value();
                let upper = adj.get_upper();
                debug!("Height: {}", height);
                debug!("Value: {}", value);
                debug!("Upper: {}", upper);
                debug!("Calc: {}", upper - 200f64 - height);
                match value >= upper - 200f64 - height || value == 0f64 {
                    true => Some(adj),
                    false => None
                }
            },
            None => None
        };

        self.internal_add_row(row);

        gtk::timeout_add(10, move || {
            if let Some(adj) = &scroll_to_bottom {
                adj.set_value(adj.get_upper());
            }
            Continue(false)
        });

    }

    pub fn add_rows(&self, rows: &[ChatTypes]) {

        // TODO: Only scroll to bottom if already at the bottom-ish
        let scroll_to_bottom = match self.scroll_window.get_vadjustment() {
            Some(adj) => {
                let height = self.scroll_window.get_allocated_height() as f64;
                let value = adj.get_value();
                let upper = adj.get_upper();
                debug!("Height: {}", height);
                debug!("Value: {}", value);
                debug!("Upper: {}", upper);
                debug!("Calc: {}", upper - 200f64 - height);
                match value >= upper - 200f64 - height || value == 0f64 {
                    true => Some(adj),
                    false => None
                }
            },
            None => None
        };

        for row in rows {
            self.internal_add_row(row);
        }

        gtk::timeout_add(10, move || {
            if let Some(adj) = &scroll_to_bottom {
                adj.set_value(adj.get_upper());
            }
            Continue(false)
        });

    }

    fn internal_add_row(&self, row: &ChatTypes) {

        let new_row = match row {
            ChatTypes::IncommingPoke(from) => Self::incomming_poke(from),
            ChatTypes::OutgoingPoke(to) => Self::outgoing_poke(to),
            ChatTypes::IncommingMessage(from, body) => Self::incomming_message(from, body),
            ChatTypes::OutgoingMessage(body) => Self::outgoing_message(body),
            ChatTypes::Error(body) => Self::error(body)
        };

        self.chat_feed.insert(&new_row, -1);

    }

    fn incomming_poke(from: &str) -> ListBoxRow {

        Self::poke_row(&format!("{} poked you!", from), Align::Start)

    }

    fn outgoing_poke(to: &str) -> ListBoxRow {

        Self::poke_row(&format!("You poked {}!", to), Align::End)

    }

    fn incomming_message(from: &str, body: &str) -> ListBoxRow {

        let row = ListBoxRow::new();
        let vbox = Self::message_vbox();

        let from_label = Label::new(Some(from));
        let body_label = Label::new(None);
        from_label.set_halign(Align::Start);
        from_label.set_xalign(0f32);
        body_label.set_line_wrap(true);
        body_label.set_line_wrap_mode(WrapMode::WordChar);
        body_label.set_halign(Align::Start);
        body_label.set_xalign(0f32);
        body_label.set_justify(Justification::Left);
        body_label.set_markup(&escape_markdown(&body));
        body_label.set_selectable(true);
        vbox.add(&from_label);
        vbox.add(&body_label);
        row.add(&vbox);
        row.show_all();

        row

    }

    fn outgoing_message(body: &str) -> ListBoxRow {

        let row = ListBoxRow::new();
        let vbox = Self::message_vbox();

        let ctx = row.get_style_context();
        ctx.add_class("light-gray-bg");

        let from_label = Label::new(Some("Me"));
        let body_label = Label::new(None);
        from_label.set_halign(Align::End);
        from_label.set_xalign(1f32);
        body_label.set_line_wrap(true);
        body_label.set_line_wrap_mode(WrapMode::WordChar);
        body_label.set_halign(Align::End);
        body_label.set_xalign(1f32);
        body_label.set_justify(Justification::Right);
        body_label.set_markup(&escape_markdown(&body));
        body_label.set_selectable(true);
        vbox.add(&from_label);
        vbox.add(&body_label);
        row.add(&vbox);
        row.show_all();

        row

    }

    fn error(body: &str) -> ListBoxRow {

        let row = ListBoxRow::new();

        let ctx = row.get_style_context();
        ctx.add_class("error-row");

        let body = Label::new(Some(format!("Error: {}", body).as_str()));
        body.set_line_wrap(true);
        body.set_line_wrap_mode(WrapMode::WordChar);
        body.set_halign(Align::Start);
        body.set_xalign(1f32);
        body.set_margin_top(12);
        body.set_margin_bottom(12);
        body.set_margin_start(12);
        body.set_margin_end(12);

        row.add(&body);
        row.show_all();

        row

    }

    fn poke_row(body: &str, align: Align) -> ListBoxRow {

        let row = ListBoxRow::new();

        let ctx = row.get_style_context();
        ctx.add_class("poke-row");

        let body = Label::new(Some(body));
        body.set_line_wrap(true);
        body.set_line_wrap_mode(WrapMode::WordChar);
        body.set_halign(align);
        body.set_margin_top(12);
        body.set_margin_bottom(12);
        body.set_margin_start(12);
        body.set_margin_end(12);

        match align {
            Align::Start => body.set_xalign(0f32),
            Align::End => body.set_xalign(1f32),
            _ => panic!("what are you doing")
        }

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
