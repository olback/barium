mod error;
mod key_pair;
mod macros;
mod resources;

use error::BariumResult;
use gtk::prelude::*;
use gio::prelude::*;
use std::env::args;
use std::sync::{Arc, RwLock};
use padlock;
use tray_indicator::TrayIndicator;

pub const BASE_RESOURCE_PATH: &'static str = "/net/olback/barium";

fn main() -> BariumResult<()> {

    // Load resources
    resources::load();

    // Create application
    let application = gtk::Application::new(Some("net.olback.barium"), Default::default())?;
    gtk::Window::set_default_icon_name("net.olback.Barium");

    let builder = gtk::Builder::new_from_resource("/net/olback/barium/ui/main-window");
    let window: gtk::ApplicationWindow = get_obj!(builder, "main-window");

    application.connect_activate(move |app| {
        window.set_application(Some(app));
        window.show_all();
    });


    application.run(&args().collect::<Vec<String>>());

    Ok(())

}
