use crate::get_obj;
use gtk::prelude::*;

pub struct InitialSetup {
    // Welcome
    welcome_continue: gtk::Button,
    // Identity
    identity_continue: gtk::Button,
    identity_username: gtk::Entry,
    identity_key_size: gtk::ComboBoxText,
    identity_error: gtk::InfoBar,
    identity_error_label: gtk::Label,
    // Connect
    connect_connect: gtk::Button,
    connect_server_address: gtk::Entry,
    connect_server_port: gtk::Entry,
    connect_server_password: gtk::Entry,
    connect_error: gtk::InfoBar,
    connect_error_label: gtk::Label
}

impl InitialSetup {

    pub fn build(builder: &gtk::Builder) -> Self {

        let setup = Self {
            // Welcome
            welcome_continue: get_obj!(builder, "welcome_continue"),
            // Identity
            identity_continue: get_obj!(builder, "identity_continue"),
            identity_username: get_obj!(builder, "identity_username"),
            identity_key_size: get_obj!(builder, "identity_key_size"),
            identity_error: get_obj!(builder, "identity_error"),
            identity_error_label: get_obj!(builder, "identity_error_label"),
            // Connect
            connect_connect: get_obj!(builder, "connect_connect"),
            connect_server_address: get_obj!(builder, "connect_server_address"),
            connect_server_port: get_obj!(builder, "connect_server_port"),
            connect_server_password: get_obj!(builder, "connect_server_password"),
            connect_error: get_obj!(builder, "connect_error"),
            connect_error_label: get_obj!(builder, "connect_error_label")
        };

        let identity_continue_clone = setup.identity_continue.clone();
        setup.identity_username.connect_changed(move |entry| {
            let input = entry.get_text().map_or(String::new(), |val| {
                val.to_string()
            });
            if input.trim().is_empty() {
                identity_continue_clone.set_sensitive(false);
            } else {
                identity_continue_clone.set_sensitive(true);
            }
        });

        let identity_key_size_clone = setup.identity_key_size.clone();
        let identity_error = setup.identity_error.clone();
        let identity_error_label = setup.identity_error_label.clone();
        setup.identity_continue.connect_clicked(move |btn| {
            let value = identity_key_size_clone.get_active_text().map_or(None, |val| {
                val.to_string().parse::<u32>().ok()
            });
            if let Some(v) = value {
                println!("{}", v);
                // continue
            } else {
                identity_error_label.set_text("Invalid key size");
                identity_error.set_visible(true);
            }
        });

        setup

    }

}
