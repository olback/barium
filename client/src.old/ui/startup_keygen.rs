use gtk::{self, prelude::*};
use glib;
use crate::get_obj;
use super::utils::StackSwitcher;

#[derive(PartialEq)]
pub enum StartupKeygenStatus {
    Generating(usize, String),
    Done
}

pub struct StartupKeygen {
    label: gtk::Label,
    tx: glib::Sender<StartupKeygenStatus>
}

impl StartupKeygen {

    pub fn build(builder: &gtk::Builder, stack: &gtk::Stack) -> Self {

        let (tx, rx) = glib::MainContext::channel(glib::Priority::default());

        let inner = Self {
            tx: tx,
            label: get_obj!(builder, "generate_keys_info")
        };

        let label = inner.label.clone();
        let stack_clone = stack.clone();
        rx.attach(None, move |msg| {

            match msg {
                StartupKeygenStatus::Done => {
                    StackSwitcher::new(&stack_clone, "main").left();
                    glib::Continue(false)
                },
                StartupKeygenStatus::Generating(size, server) => {
                    label.set_text(&format!("Generating {} bit key for {}", size, server));
                    glib::Continue(true)
                }
            }

        });

        inner

    }

    pub fn get_tx(&self) -> glib::Sender<StartupKeygenStatus> {
        self.tx.clone()
    }

}
