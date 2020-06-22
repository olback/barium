use {
    crate::{get_obj, consts, utils, error::BariumResult},
    gtk::{Builder, Entry, prelude::*},
    log::info
};

#[derive(Debug, Clone)]
pub struct ChatInput {
    pub poke_button: gtk::Button,
    pub chat_entry: gtk::Entry,
    pub send_button: gtk::Button
}

impl ChatInput {

    pub fn build(builder: &Builder) -> BariumResult<Self> {

        let inner = Self {
            poke_button: get_obj!(builder, "poke_button"),
            chat_entry: get_obj!(builder, "chat_entry"),
            send_button: get_obj!(builder, "send_button")
        };

        inner.connect_chat_entry();
        inner.connect_send_button();
        inner.connect_poke_button();

        Ok(inner)

    }

    fn connect_chat_entry(&self) {

        self.chat_entry.set_max_length(consts::MAX_MESSAGE_TEXT_LENGTH as i32);

        self.chat_entry.connect_changed(|entry: &Entry| {

            let text = utils::entry_get_text(&entry);
            let ctx = entry.get_style_context();

            if text.len() >= consts::MESSAGE_ERROR_LENGHT {

                if ctx.has_class("warning") {
                    ctx.remove_class("warning");
                }

                if !ctx.has_class("error") {
                    ctx.add_class("error");
                }

            } else if text.len() > consts::MESSAGE_WARNING_LENGHT {

                if ctx.has_class("error") {
                    ctx.remove_class("error");
                }

                if !ctx.has_class("warning") {
                    ctx.add_class("warning");
                }

            } else {

                if ctx.has_class("warning") {
                    ctx.remove_class("warning");
                }

                if ctx.has_class("error") {
                    ctx.remove_class("error");
                }

            }

            info!("{} / {} / {} / {}", text.len(), consts::MESSAGE_WARNING_LENGHT, consts::MESSAGE_ERROR_LENGHT, consts::MAX_MESSAGE_TEXT_LENGTH);

        });

    }

    fn connect_send_button(&self) {

        self.send_button.connect_clicked(|_| {

            info!("Send");

        });

    }

    fn connect_poke_button(&self) {

        self.poke_button.connect_clicked(move |_| {

            info!("Poke");

        });

    }

}
