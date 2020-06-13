use {
    crate::{get_obj, error::BariumResult},
    gtk::Builder
};

mod initial_setup;
mod chat_input;

#[derive(Debug)]
pub struct Ui {
    pub main_window: gtk::ApplicationWindow,
    pub initial_setup: initial_setup::InitialSetup,
    pub chat_input: chat_input::ChatInput
}

impl Ui {

    pub fn build(builder: &Builder) -> BariumResult<Self> {

        let inner = Self {
            main_window: get_obj!(builder, "main_window"),
            initial_setup: initial_setup::InitialSetup::build(&builder)?,
            chat_input: chat_input::ChatInput::build(&builder)?
        };

        Ok(inner)

    }

}
