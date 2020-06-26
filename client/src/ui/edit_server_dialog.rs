use {
    crate::{ui::ServerIdentity, get_obj, error::BariumResult, servers::{Server, Servers, ComparableServer}, utils::entry_get_text},
    std::{sync::{Arc, Mutex}, convert::TryInto},
    gtk_resources::UIResource,
    gtk::{ApplicationWindow, Builder, Button, ToggleButton, Dialog, Entry,
    Label, InfoBar, Switch, EntryIconPosition, prelude::*},
    padlock,
    glib::clone,
    log::{debug, warn, error}
};

#[derive(Debug, UIResource)]
#[resource="/net/olback/barium/ui/edit-server-dialog"]
pub struct EditServerDialog {
    pub edit_server_dialog: Dialog,
    pub info_bar: InfoBar,
    pub info_bar_label: Label,
    pub header_label: Label,
    pub name_entry: Entry,
    pub address_entry: Entry,
    pub port_entry: Entry,
    pub password_check: ToggleButton,
    pub password_entry: Entry,
    pub allow_invalid_cert_switch: Switch,
    pub remove_button: Button,
    pub save_button: Button,
    _server_idnentity: Entry
}

impl EditServerDialog {

    pub fn build(builder: &Builder, fs_servers: Arc<Mutex<Servers>>) -> BariumResult<Self> {

        let inner = Self::load()?;
        let main_window: ApplicationWindow = get_obj!(builder, "main_window");

        inner.edit_server_dialog.set_transient_for(Some(&main_window));
        inner.edit_server_dialog.get_content_area().set_border_width(0);

        inner.password_check.connect_toggled(clone!(
            @strong inner.password_entry as password_entry
        => move |chk_btn| {
            if chk_btn.get_active() {
                password_entry.set_sensitive(true);
            } else {
                password_entry.set_sensitive(false);
            }
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

        inner.remove_button.connect_clicked(clone!(
            @strong inner.edit_server_dialog as edit_server_dialog,
            @strong inner.info_bar as info_bar,
            @strong inner.info_bar_label as info_bar_label,
            @strong inner._server_idnentity as _server_idnentity_entry,
            @strong fs_servers
        => move |_| {

            let server_identity = entry_get_text(&_server_idnentity_entry);
            let server_identity_parts = server_identity.split(':').collect::<Vec<&str>>();
            let server_identity_addr = server_identity_parts[0];
            let server_identity_port = server_identity_parts[1].parse::<u16>().unwrap();

            padlock::mutex_lock(&fs_servers, |lock| {

                let server = match lock.find_by_addr(server_identity_addr, &server_identity_port) {
                    Some(s) => s.clone(),
                    None => {
                        error!("Server not found");
                        info_bar_label.set_text("Server not found");
                        info_bar.set_visible(true);
                        info_bar.set_revealed(true);
                        return
                    }
                };

                match lock.remove(&server.as_comparable()) {
                    Ok(_) => drop(edit_server_dialog.emit("close", &[])),
                    Err(e) => {
                        error!("{}", e);
                        info_bar_label.set_text("Error removing server");
                        info_bar.set_visible(true);
                        info_bar.set_revealed(true);
                    }
                }

            });

        }));

        Ok(inner)

    }

    pub fn show(&self, comparable: &ComparableServer) {

        self.info_bar.set_visible(false);
        self.info_bar.set_revealed(false);
        self.header_label.set_text(comparable.name);
        self._server_idnentity.set_text(&format!("{}:{}", comparable.address, comparable.port));
        self.name_entry.set_text(comparable.name);
        self.address_entry.set_text(comparable.address);
        self.port_entry.set_text(&format!("{}", comparable.port));
        match comparable.password {
            Some(password) => {
                self.password_entry.set_text(password);
                self.password_check.set_active(true);
            },
            None => {
                self.password_entry.set_text("");
                self.password_check.set_active(false);
            }
        }
        self.allow_invalid_cert_switch.set_active(*comparable.allow_invalid_cert);

        match self.edit_server_dialog.run() {
            _ => self.edit_server_dialog.hide()
        }

    }

}
