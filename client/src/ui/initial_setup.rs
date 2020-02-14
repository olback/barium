use crate::get_obj;
use gtk::prelude::*;
use gio::prelude::*;

use crate::notification;

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

    pub fn build(app: &gtk::Application, builder: &gtk::Builder) -> Self {

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

        let app_clone = app.clone();
        setup.welcome_continue.connect_clicked(move |_| {
            let notif = notification::new("This is a title", "This is a little longer body...");
            app_clone.send_notification(Some("Barium: <from user>"), &notif);
        });

        let identity_continue_clone = setup.identity_continue.clone();
        let identity_key_size = setup.identity_key_size.clone();
        let identity_error = setup.identity_error.clone();
        let identity_error_label = setup.identity_error_label.clone();
        setup.identity_username.connect_changed(move |entry| {

            let valid_username = validate_username(entry);
            let valid_key_size = validate_key_size(&identity_key_size);

            if valid_username && valid_key_size {
                identity_error.set_visible(false);
                identity_continue_clone.set_sensitive(true);
            } else if !valid_username {
                identity_continue_clone.set_sensitive(false);
            } else if !valid_key_size {
                identity_continue_clone.set_sensitive(false);
                identity_error_label.set_text("Invalid key size");
                identity_error.set_visible(true);
            }

        });

        let identity_continue_clone = setup.identity_continue.clone();
        let identity_username = setup.identity_username.clone();
        let identity_error = setup.identity_error.clone();
        let identity_error_label = setup.identity_error_label.clone();
        setup.identity_key_size.connect_changed(move |key_size| {

            let valid_username = validate_username(&identity_username);
            let valid_key_size = validate_key_size(key_size);

            if valid_username && valid_key_size {
                identity_error.set_visible(false);
                identity_continue_clone.set_sensitive(true);
            } else if !valid_username {
                identity_continue_clone.set_sensitive(false);
            } else if !valid_key_size {
                identity_continue_clone.set_sensitive(false);
                identity_error_label.set_text("Invalid key size");
                identity_error.set_visible(true);
            }

        });

        setup.identity_continue.connect_clicked(move |btn| {
        });

        setup

    }

}

fn validate_username(entry: &gtk::Entry) -> bool {

    !entry.get_text().map_or(String::new(), |val| {
        val.to_string().trim().to_string()
    }).is_empty()

}

fn validate_key_size(combo_box: &gtk::ComboBoxText) -> bool {

    combo_box.get_active_text().map_or(None, |val| {
        val.to_string().parse::<u32>().ok()
    }).is_some()

}
