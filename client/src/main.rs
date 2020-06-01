mod error;
mod key_pair;
mod macros;
mod resources;
mod services;

use error::BariumResult;
use gtk::prelude::*;
use gio::prelude::*;
use std::env::args;
use std::sync::{Arc, RwLock};
use padlock;
use tray_item::TrayItem;
use services::{MainWindowEvents, MainWindowEvent};

// pub const BASE_RESOURCE_PATH: &'static str = "/net/olback/barium";

fn main() -> BariumResult<()> {

    // Load resources
    resources::load();

    // Create application
    let application = gtk::Application::new(Some("net.olback.barium"), Default::default())?;
    gtk::Window::set_default_icon_name("net.olback.Barium");

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

    let builder = gtk::Builder::new_from_resource("/net/olback/barium/ui/main-window");
    let window: gtk::ApplicationWindow = get_obj!(builder, "main-window");

    let (mwe, mwe_rx) = MainWindowEvents::new();
    mwe_rx.attach(None, glib::clone!(@strong application => move |msg| {

        match msg {
            MainWindowEvent::Show => {
                let windows = application.get_windows();
                if windows.len() > 0 {
                    // Probably not ideal to assume that the first entry is the main window
                    windows[0].show();
                    windows[0].present();
                }
            },
            MainWindowEvent::Hide => {
                let windows = application.get_windows();
                if windows.len() > 0 {
                    windows[0].hide();
                }
            },
            MainWindowEvent::Quit => {
                application.quit();
            }
        }

        glib::Continue(true)

    }));

    let mut tray = TrayItem::new("Barium", "net.olback.Barium")?;
    tray.add_label("Barium")?;
    tray.add_menu_item("Show", glib::clone!(@strong mwe => move || {
        mwe.show();
    }))?;
    tray.add_menu_item("Hide", glib::clone!(@strong mwe => move || {
        mwe.hide();
    }))?;
    tray.add_menu_item("Quit", glib::clone!(@strong mwe => move || {
        mwe.quit();
    }))?;

    application.connect_activate(move |app| {
        window.set_application(Some(app));
        window.show_all();
    });

    application.run(&args().collect::<Vec<String>>());

    Ok(())

}
