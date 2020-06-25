use {
    crate::{
        get_obj,
        error::BariumResult,
        servers::{Server, Servers},
        utils::{entry_get_text, new_user_id},
        services::{get_server_properties, verify_server_password}
    },
    std::sync::{Arc, Mutex},
    gtk_resources::UIResource,
    gtk::{
        ApplicationWindow, Builder, Button, Label, Stack, InfoBar,
        Entry, Switch, EntryIconPosition, Dialog, prelude::*
    },
    glib::{clone, Continue},
    padlock
};

#[derive(Debug, UIResource)]
#[resource="/net/olback/barium/ui/add-server-dialog"]
pub struct AddServerDialog {
    pub add_server_dialog: Dialog,
    pub info_bar: InfoBar,
    pub info_bar_label: Label,
    pub stack: Stack,
    pub name_entry: Entry,
    pub address_entry: Entry,
    pub port_entry: Entry,
    pub allow_invalid_cert: Switch,
    pub connect_button: Button,
    pub connect_button_stack: Stack,
    pub password_entry: Entry,
    pub authenticate_button: Button,
    pub authenticate_button_stack: Stack
}

impl AddServerDialog {

    pub fn build(builder: &Builder, servers: Arc<Mutex<Servers>>) -> BariumResult<Self> {

        let inner = Self::load()?;

        let main_window: ApplicationWindow = get_obj!(builder, "main_window");
        inner.add_server_dialog.set_transient_for(Some(&main_window));
        inner.add_server_dialog.get_content_area().set_border_width(0);

        inner.connect_button.connect_clicked(clone!(
            @strong servers,
            @strong inner.add_server_dialog as add_server_dialog,
            @strong inner.info_bar as info_bar,
            @strong inner.info_bar_label as info_bar_label,
            @strong inner.stack as stack,
            @strong inner.name_entry as name_entry,
            @strong inner.address_entry as address_entry,
            @strong inner.port_entry as port_entry,
            @strong inner.allow_invalid_cert as allow_invalid_cert_switch,
            @strong inner.connect_button_stack as connect_button_stack
        => move |connect_button| {

            let name = entry_get_text(&name_entry);
            if name.trim().is_empty() {
                info_bar_label.set_text("Name may not be empty");
                info_bar.set_visible(true);
                info_bar.set_revealed(true);
                return;
            }

            let address = entry_get_text(&address_entry);
            if address.trim().is_empty() {
                info_bar_label.set_text("Address may not be empty");
                info_bar.set_visible(true);
                info_bar.set_revealed(true);
                return;
            }

            let port = match entry_get_text(&port_entry).parse::<u16>() {
                Ok(p) => p,
                Err(_) => {
                    info_bar_label.set_text("Invalid port number");
                    info_bar.set_visible(true);
                    info_bar.set_revealed(true);
                    return
                }
            };

            let allow_invalid_cert = allow_invalid_cert_switch.get_active();

            let exists = padlock::mutex_lock(&servers, |lock| lock.find(&address, &port).is_some());

            if exists {
                info_bar_label.set_text("Server already added");
                info_bar.set_visible(true);
                info_bar.set_revealed(true);
                return
            }

            connect_button.set_sensitive(false);
            connect_button_stack.set_visible_child_name("spinner");

            get_server_properties(address.clone(), port, allow_invalid_cert).attach(None, clone!(
                @strong servers,
                @strong add_server_dialog,
                @strong info_bar,
                @strong info_bar_label,
                @strong stack,
                @strong allow_invalid_cert,
                @strong connect_button,
                @strong connect_button_stack
            => move |props| {

                let password_required = match props {
                    Ok(p) => p.requires_password,
                    Err(_) => {
                        info_bar_label.set_text("Error fetching server properties");
                        info_bar.set_visible(true);
                        info_bar.set_revealed(true);
                        connect_button.set_sensitive(true);
                        connect_button_stack.set_visible_child_name("label");
                        return Continue(false)
                    }
                };

                if password_required {

                    info_bar.set_visible(false);
                    info_bar.set_revealed(false);
                    stack.set_visible_child_name("password");

                } else {

                    // Add server
                    padlock::mutex_lock(&servers, |lock| {
                        drop(lock.add(Server::new(
                            new_user_id(),
                            name.clone(),
                            address.clone(),
                            port,
                            None,
                            allow_invalid_cert
                        )));
                    });

                    drop(add_server_dialog.emit("close", &[]));

                }

                Continue(false)

            }));

        }));

        inner.authenticate_button.connect_clicked(clone!(
            @strong servers,
            @strong inner.add_server_dialog as add_server_dialog,
            @strong inner.info_bar as info_bar,
            @strong inner.info_bar_label as info_bar_label,
            @strong inner.name_entry as name_entry,
            @strong inner.address_entry as address_entry,
            @strong inner.port_entry as port_entry,
            @strong inner.allow_invalid_cert as allow_invalid_cert_switch,
            @strong inner.password_entry as password_entry,
            @strong inner.authenticate_button as authenticate_button,
            @strong inner.authenticate_button_stack as authenticate_button_stack
        => move |_| {

            let name = entry_get_text(&name_entry);
            if name.trim().is_empty() {
                info_bar_label.set_text("Name may not be empty");
                info_bar.set_visible(true);
                info_bar.set_revealed(true);
                return;
            }

            let address = entry_get_text(&address_entry);
            if address.trim().is_empty() {
                info_bar_label.set_text("Address may not be empty");
                info_bar.set_visible(true);
                info_bar.set_revealed(true);
                return;
            }

            let port = match entry_get_text(&port_entry).parse::<u16>() {
                Ok(p) => p,
                Err(_) => {
                    info_bar_label.set_text("Invalid port number");
                    info_bar.set_visible(true);
                    info_bar.set_revealed(true);
                    return
                }
            };

            let allow_invalid_cert = allow_invalid_cert_switch.get_active();

            let password = entry_get_text(&password_entry);

            authenticate_button.set_sensitive(false);
            authenticate_button_stack.set_visible_child_name("spinner");

            verify_server_password(address.clone(), port, allow_invalid_cert, password.clone()).attach(None, clone!(
                @strong servers,
                @strong add_server_dialog,
                @strong info_bar,
                @strong info_bar_label,
                @strong authenticate_button,
                @strong authenticate_button_stack
            => move |password_ok| {

                match password_ok {

                    Ok(is_ok) => {

                        if is_ok {

                            // Add server
                            padlock::mutex_lock(&servers, |lock| {
                                drop(lock.add(Server::new(
                                    new_user_id(),
                                    name.clone(),
                                    address.clone(),
                                    port,
                                    Some(password.clone()),
                                    allow_invalid_cert
                                )));
                            });

                            drop(add_server_dialog.emit("close", &[]));

                        } else {

                            info_bar_label.set_text("Password incorrect");
                            info_bar.set_visible(true);
                            info_bar.set_revealed(true);
                            authenticate_button.set_sensitive(true);
                            authenticate_button_stack.set_visible_child_name("label");

                        }

                    },

                    Err(_) => {

                        info_bar_label.set_text("Error validating password");
                        info_bar.set_visible(true);
                        info_bar.set_revealed(true);
                        authenticate_button.set_sensitive(true);
                        authenticate_button_stack.set_visible_child_name("label");

                    }

                }

                Continue(false)

            }));

        }));

        inner.password_entry.connect_icon_press(|entry: &Entry, pos: EntryIconPosition, _| {
            if pos == EntryIconPosition::Secondary {
                entry.set_visibility(true);
            }
        });

        inner.password_entry.connect_icon_release(|entry: &Entry, pos: EntryIconPosition, _| {
            if pos == EntryIconPosition::Secondary {
                entry.set_visibility(false);
            }
        });

        Ok(inner)

    }

    pub fn show(&self) {

        self.info_bar.set_visible(false);
        self.info_bar.set_revealed(false);
        self.stack.set_visible_child_name("server");
        self.name_entry.set_text("");
        self.address_entry.set_text("");
        self.port_entry.set_text("13337");
        self.allow_invalid_cert.set_active(false);
        self.connect_button.set_sensitive(true);
        self.connect_button_stack.set_visible_child_name("label");
        self.password_entry.set_text("");
        self.authenticate_button.set_sensitive(true);
        self.authenticate_button_stack.set_visible_child_name("label");

        match self.add_server_dialog.run() {
            _ => self.add_server_dialog.hide()
        }

    }

}
