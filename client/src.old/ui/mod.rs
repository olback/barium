use gtk::prelude::*;
use gio::prelude::*;
use glib;
use crate::get_obj;

mod utils;

pub mod startup_keygen;
use startup_keygen::StartupKeygen;

pub mod initial_setup;
use initial_setup::InitialSetup;

pub struct Ui {
    pub main_window: gtk::ApplicationWindow,
    pub stack: gtk::Stack,
    pub startup_keygen: StartupKeygen,
    pub about_dialog: gtk::AboutDialog,
    setup: InitialSetup
}

impl Ui {

    pub fn build(app: &gtk::Application, builder: &gtk::Builder) -> Self {

        // Load CSS
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/net/olback/barium/css/app.css");
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        glib::set_application_name("barium");
        glib::set_prgname(Some("barium"));

        // Main window
        let main_window: gtk::ApplicationWindow = get_obj!(builder, "main_window");
        main_window.set_application(Some(app));
        main_window.hide_on_delete();
        main_window.show();
        // Clicking the exit/close button (X) should hide the window, not destroy it
        main_window.connect_delete_event(move |window, _| {
            window.hide();
            glib::signal::Inhibit(true)
        });

        // Stack of views
        let stack: gtk::Stack = get_obj!(builder, "stack1");

        // About dialog
        let about_dialog: gtk::AboutDialog = get_obj!(builder, "about_dialog");

        let ui = Self {
            main_window: main_window,
            stack: stack.clone(),
            startup_keygen: StartupKeygen::build(builder, &stack),
            about_dialog: about_dialog,
            setup: InitialSetup::build(app, builder, &stack)
        };

        Self::connect_actions(app, &ui);

        ui

    }

    fn connect_actions(app: &gtk::Application, ui: &Self) {

        // Top-level actions
        let actions = gio::SimpleActionGroup::new();
        ui.main_window.insert_action_group("app", Some(&actions));

        // About dialog
        let about_dialog_clone = ui.about_dialog.clone();
        let open_about_action = gio::SimpleAction::new("open-about", None);
        open_about_action.connect_activate(move |_, _| {
            match about_dialog_clone.run() {
                _ => about_dialog_clone.hide()
            }
        });
        actions.add_action(&open_about_action);

        // Quit action
        let app_clone = app.clone();
        let quit_action = gio::SimpleAction::new("quit", None);
        quit_action.connect_activate(move |_, _| {
            app_clone.quit();
        });
        actions.add_action(&quit_action);

    }

}
