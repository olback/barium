use {
    crate::{
        get_obj,
        fs::{Server, Servers},
        utils::{entry_get_text, get_server_properties, verify_server_password},
        error::BariumResult,
        consts::DEFAULT_PORT
    },
    std::sync::{Arc, Mutex},
    gtk::{Builder, Button, Stack, Entry, EntryIconPosition, Label, Switch, InfoBar, prelude::*},
    glib::clone,
    barium_shared::ServerProperties,
    padlock
};

#[derive(Debug, Clone)]
pub struct InitialSetup {
    pub servers: Arc<Mutex<Servers>>,
    pub main_stack: Stack,
    pub setup_stack: Stack,
    pub setup_continue_button: Button,
    pub setup_server_name_entry: Entry,
    pub setup_address_entry: Entry,
    pub setup_port_entry: Entry,
    pub setup_allow_invalid_cert_switch: Switch,
    pub setup_connect_button: Button,
    pub setup_password_entry: Entry,
    pub setup_password_connect_button: Button,
    pub setup_infobar: InfoBar,
    pub setup_infobar_label: Label
}

impl InitialSetup {

    pub fn build(builder: &Builder, servers: Arc<Mutex<Servers>>) -> BariumResult<Self> {

        let inner = Self {
            servers: servers,
            main_stack: get_obj!(builder, "main_stack"),
            setup_stack: get_obj!(builder, "setup_stack"),
            setup_continue_button: get_obj!(builder, "setup_continue_button"),
            setup_server_name_entry: get_obj!(builder, "setup_server_name_entry"),
            setup_address_entry: get_obj!(builder, "setup_address_entry"),
            setup_port_entry: get_obj!(builder, "setup_port_entry"),
            setup_allow_invalid_cert_switch: get_obj!(builder, "setup_allow_invalid_cert_switch"),
            setup_connect_button: get_obj!(builder, "setup_connect_button"),
            setup_password_entry: get_obj!(builder, "setup_password_entry"),
            setup_password_connect_button: get_obj!(builder, "setup_password_connect_button"),
            setup_infobar: get_obj!(builder, "setup_infobar"),
            setup_infobar_label: get_obj!(builder, "setup_infobar_label")
        };

        inner.connect_setup_continue_button();
        inner.connect_setup_connect_button();
        inner.connect_setup_password_entry();
        inner.connect_setup_password_connect_button();
        inner.connect_infobar();

        Ok(inner)

    }

    fn connect_setup_continue_button(&self) {

        self.setup_continue_button.connect_clicked(clone!(@strong self.setup_stack as stack => move |_| {
            stack.set_visible_child_name("setup_connect_main_view");
        }));

    }

    fn connect_setup_connect_button(&self) {

        self.setup_connect_button.connect_clicked(clone!(
            @strong self.servers as servers,
            @strong self.main_stack as main_stack,
            @strong self.setup_stack as setup_stack,
            @strong self.setup_server_name_entry as setup_server_name_entry,
            @strong self.setup_address_entry as setup_address_entry,
            @strong self.setup_port_entry as setup_port_entry,
            @strong self.setup_infobar as setup_infobar,
            @strong self.setup_infobar_label as setup_infobar_label,
            @strong self.setup_allow_invalid_cert_switch as setup_allow_invalid_cert_switch
        => move |button: &Button| {

            setup_infobar.set_visible(false);
            button.set_sensitive(false);

            let name = entry_get_text(&setup_server_name_entry);
            if name.trim().is_empty() {
                button.set_sensitive(true);
                setup_infobar_label.set_text("Name may not be empty");
                setup_infobar.set_visible(true);
                return;
            }
            let address = entry_get_text(&setup_address_entry);
            let port_text = entry_get_text(&setup_port_entry);
            let port = match port_text.len() {
                0 => DEFAULT_PORT,
                _ => match port_text.parse::<u16>() {
                    Ok(p) => p,
                    Err(e) => {
                        button.set_sensitive(true);
                        setup_infobar_label.set_text(format!("{}", e).as_str());
                        setup_infobar.set_visible(true);
                        return
                    }
                }
            };
            let allow_invalid_cert = setup_allow_invalid_cert_switch.get_active();

            get_server_properties(address.clone(), port, allow_invalid_cert).attach(None, clone!(
                @strong servers,
                @strong button,
                @strong setup_stack,
                @strong main_stack,
                @strong setup_infobar,
                @strong setup_infobar_label
            => move |data: BariumResult<ServerProperties>| {

                button.set_sensitive(true);

                match data {
                    Ok(props) => {
                        if props.requires_password {
                            setup_stack.set_visible_child_name("setup_connect_password_view");
                        } else {
                            let server = Server {
                                name: name.clone(),
                                address: address.clone(),
                                port,
                                password: None,
                                allow_invalid_cert,
                                friends: Vec::new()
                            };
                            add_server(&servers, server).unwrap();
                            main_stack.set_visible_child_name("chat");
                        }
                    },
                    Err(e) => {
                        setup_infobar_label.set_text(format!("{}", e).as_str());
                        setup_infobar.set_visible(true);
                    }
                }

                Continue(false)

            }));

        }));

    }

    fn connect_setup_password_entry(&self) {

        self.setup_password_entry.connect_icon_press(|entry: &Entry, pos: EntryIconPosition, _| {
            if pos == EntryIconPosition::Secondary {
                entry.set_visibility(true);
            }
        });

        self.setup_password_entry.connect_icon_release(|entry: &Entry, pos: EntryIconPosition, _| {
            if pos == EntryIconPosition::Secondary {
                entry.set_visibility(false);
            }
        });

    }

    fn connect_setup_password_connect_button(&self) {

        self.setup_password_connect_button.connect_clicked(clone!(
            @strong self.servers as servers,
            @strong self.main_stack as main_stack,
            @strong self.setup_server_name_entry as setup_server_name_entry,
            @strong self.setup_password_entry as setup_password_entry,
            @strong self.setup_address_entry as setup_address_entry,
            @strong self.setup_port_entry as setup_port_entry,
            @strong self.setup_infobar as setup_infobar,
            @strong self.setup_infobar_label as setup_infobar_label,
            @strong self.setup_allow_invalid_cert_switch as setup_allow_invalid_cert_switch
        => move |button: &Button| {

            button.set_sensitive(false);

            let name = entry_get_text(&setup_server_name_entry);
            let password = entry_get_text(&setup_password_entry);
            let address = entry_get_text(&setup_address_entry);
            let port = entry_get_text(&setup_port_entry).parse::<u16>().unwrap(); // Safe to unwrap
            let allow_invalid_cert = setup_allow_invalid_cert_switch.get_active();

            verify_server_password(address.clone(), port, allow_invalid_cert, password.clone()).attach(None, clone!(
                @strong servers,
                @strong button,
                @strong main_stack,
                @strong setup_infobar,
                @strong setup_infobar_label
            => move |data: BariumResult<bool>| {

                button.set_sensitive(true);

                match data {
                    Ok(valid) => {
                        if valid {
                            let server = Server {
                                name: name.clone(),
                                address: address.clone(),
                                port,
                                password: Some(password.clone()),
                                allow_invalid_cert,
                                friends: Vec::new()
                            };
                            add_server(&servers, server).unwrap();
                            main_stack.set_visible_child_name("chat");
                        } else {
                            setup_infobar_label.set_text("Invalid password");
                            setup_infobar.set_visible(true);
                        }
                    },
                    Err(e) => {
                        setup_infobar_label.set_text(&format!("{}", e));
                        setup_infobar.set_visible(true);
                    }
                }

                Continue(false)

            }));

        }));

    }

    fn connect_infobar(&self) {

        self.setup_infobar.connect_response(|infobar, _| {
            infobar.set_visible(false);
        });

    }

}

fn add_server(servers: &Arc<Mutex<Servers>>, server: Server) -> BariumResult<()> {

    padlock::mutex_lock(&servers, move |servers| servers.add(server))?;

    Ok(())

}
