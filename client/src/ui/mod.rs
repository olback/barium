use gtk::prelude::*;
use glib;
use crate::get_obj;

mod initial_setup;
use initial_setup::InitialSetup;

pub struct Ui {
    pub main_window: gtk::ApplicationWindow,
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

        let main_window: gtk::ApplicationWindow = get_obj!(builder, "main_window");
        main_window.set_application(Some(app));
        main_window.show();

        Self {
            main_window: main_window,
            setup: InitialSetup::build(builder)
        }

    }

}
