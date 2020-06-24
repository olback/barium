use {
    crate::{get_obj, error::BariumResult, servers::{Servers, Friend}, utils::entry_get_text},
    std::{sync::{Arc, Mutex}, convert::TryInto},
    gtk_resources::UIResource,
    gtk::{ApplicationWindow, Builder, Button, ComboBoxText, Dialog, Entry,
    Label, InfoBar, ResponseType, prelude::*},
    barium_shared::UserHash,
    base62,
    padlock,
    glib::clone,
    log::{debug, warn}
};

#[derive(Debug, UIResource)]
#[resource="/net/olback/barium/ui/add-friend-dialog"]
pub struct AddFriendDialog {
    pub add_friend_dialog: Dialog,
    pub server_chooser: ComboBoxText,
    pub name_entry: Entry,
    pub identity_entry: Entry,
    pub info_bar: InfoBar,
    pub info_bar_label: Label,
    pub add_button: Button
}

impl AddFriendDialog {

    pub fn build(builder: &Builder, fs_servers: Arc<Mutex<Servers>>) -> BariumResult<Self> {

        let inner = Self::load()?;
        let main_window: ApplicationWindow = get_obj!(builder, "main_window");

        inner.add_friend_dialog.set_transient_for(Some(&main_window));

        inner.info_bar.connect_response(|info_bar, _| {
            info_bar.hide();
        });

        inner.add_button.connect_clicked(clone!(
            @strong inner.add_friend_dialog as add_friend_dialog,
            @strong inner.server_chooser as server_chooser,
            @strong inner.name_entry as name_entry,
            @strong inner.identity_entry as identity_entry,
            @strong inner.info_bar as info_bar,
            @strong inner.info_bar_label as info_bar_label,
            @strong inner.add_button as add_button,
            @strong fs_servers
        => move |_| {

            info_bar.set_visible(false);

            let server_idx = match server_chooser.get_active() {
                Some(idx) => idx,
                None => {
                    info_bar_label.set_text("Please select a server");
                    info_bar.set_visible(true);
                    return
                }
            };

            let name = entry_get_text(&name_entry);
            if name.trim().is_empty() {
                info_bar_label.set_text("Name may not be empty");
                info_bar.set_visible(true);
                return
            }

            let hash = match base62::decode(&entry_get_text(&identity_entry)) {
                Ok(hash_vec) => match hash_vec.as_slice().try_into() {
                    Ok(h) => h,
                    Err(e) => {
                        warn!("{}", e);
                        info_bar_label.set_text("Invalid identity");
                        info_bar.set_visible(true);
                        return
                    }
                },
                Err(e) => {
                    warn!("{}", e);
                    info_bar_label.set_text("Invalid identity");
                    info_bar.set_visible(true);
                    return
                }
            };

            padlock::mutex_lock(&fs_servers, |lock| {

                // This unwrap() should never fails as server_idx is the index
                // in the fs_servers inner vector.
                let server = lock.iter_mut().nth(server_idx as usize).unwrap();
                let friend = Friend::new(name, hash);

                match server.add_friend(friend) {

                    Ok(_) => {
                        drop(add_friend_dialog.emit("close", &[])); // Close dialog
                        drop(lock.save());
                    },

                    Err(e) => {
                        info_bar_label.set_text(&e.to_string());
                        info_bar.set_visible(true);
                    }

                }

            });

        }));

        Ok(inner)

    }

    pub fn show(&self, fs_servers: Arc<Mutex<Servers>>) {

        self.server_chooser.remove_all();
        self.name_entry.set_text("");
        self.identity_entry.set_text("");
        self.info_bar.set_visible(false);

        padlock::mutex_lock(&fs_servers, clone!(
            @strong self.server_chooser as server_chooser
        => move |sl| for s in sl.iter() {
            server_chooser.append(Some(&format!("{}:{}", s.address, s.port)), &s.name);
        }));

        match self.add_friend_dialog.run() {

            _ => self.add_friend_dialog.hide()

        }

    }

}
