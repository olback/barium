// Disable console on windows
// #![windows_subsystem = "windows"] // TODO: Enable

mod error;
mod key_pair;
mod macros;
mod resources;
mod services;
mod ui;
mod utils;
mod consts;
mod panic_handler;
mod servers;
mod logger;
mod config;

use {
    error::BariumResult,
    servers::Servers,
    key_pair::KeyPair,
    services::{MainWindowEvent, MainWindowEvents, MainStack},
    glib::{self, clone, MainContext, Priority},
    gio::{SimpleAction, SimpleActionGroup, prelude::*},
    gtk::{
        AboutDialog, Application, ApplicationWindow, Builder, CssProvider, Stack, StyleContext,
        Window, STYLE_PROVIDER_PRIORITY_APPLICATION, prelude::*
    },
    std::{env, rc::Rc, cell::RefCell, sync::{Arc, Mutex}},
    tray_item::TrayItem,
    lazy_static::lazy_static,
    padlock,
    log::{debug, info},
    config::Config
};

lazy_static! {
    pub static ref KEY_PAIR: KeyPair = KeyPair::new(consts::KEY_SIZE).unwrap();
    pub static ref CONFIG: Config = Config::load().unwrap();
}

fn run_app() -> BariumResult<()> {

    info!("Starting...");

    // Load resources
    resources::load();

    // Create application
    let application = Application::new(Some("net.olback.barium"), Default::default())?;
    Window::set_default_icon_name("net.olback.Barium");

    // Load CSS
    let provider = CssProvider::new();
    #[cfg(not(target_os = "windows"))]
    provider.load_from_resource(resource!("css/app.css"));
    #[cfg(target_os = "windows")]
    provider.load_from_resource(resource!("css/windows.css"));
    StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    glib::set_application_name("barium");
    glib::set_prgname(Some("barium"));

    let main_builder = Builder::new_from_resource(resource!("ui/main-window"));
    let about_builder = Builder::new_from_resource(resource!("ui/about-dialog"));
    let main_window: ApplicationWindow = get_obj!(main_builder, "main_window");
    let about_dialog: AboutDialog = get_obj!(about_builder, "about_dialog");
    about_dialog.set_transient_for(Some(&main_window));

    let (mwe, mwe_rx) = MainWindowEvents::new();
    mwe_rx.attach(None, clone!(@strong application => move |msg| {

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

    // Top-level actions
    let actions = SimpleActionGroup::new();
    main_window.insert_action_group("app", Some(&actions));

    // Settings
    let main_stack = MainStack::build(&main_builder)?;
    let open_settings_action = SimpleAction::new("open-settings", None);
    let close_settings_action = SimpleAction::new("close-settings", None);
    open_settings_action.connect_activate(clone!(
        @strong main_stack
    => move |_, _| {
        main_stack.show_settings();
    }));
    close_settings_action.connect_activate(clone!(
        @strong main_stack
    => move |_, _| {
        main_stack.close_settings();
    }));
    actions.add_action(&open_settings_action);
    actions.add_action(&close_settings_action);

    // About dialog
    let open_about_action = SimpleAction::new("open-about", None);
    open_about_action.connect_activate(clone!(@strong about_dialog => move |_, _| {
        match about_dialog.run() {
            _ => about_dialog.hide()
        }
    }));
    actions.add_action(&open_about_action);

    // Quit action
    let quit_action = SimpleAction::new("quit", None);
    quit_action.connect_activate(clone!(@strong application => move |_, _| application.quit()));
    actions.add_action(&quit_action);
    application.set_accels_for_action("app.quit", &["<CTRL>Q", "<CTRL>W"]);

    // Load server list
    let servers = Arc::new(Mutex::new(Servers::load()?));

    // Generate key pair
    let (tx, rx) = MainContext::channel::<()>(Priority::default());
    let keys_ready = Rc::new(RefCell::new(false));
    rx.attach(None, clone!(@strong main_stack, @strong servers, @strong keys_ready => move |_| {
        keys_ready.replace(true);
        let len = padlock::mutex_lock(&servers, |s| s.len());
        if len > 0 {
            main_stack.show_chat()
        } else {
            main_stack.show_setup()
        }
        Continue(false)
    }));
    std::thread::spawn(move || {
        let _ = &KEY_PAIR.public_key();
        tx.send(()).unwrap();
    });

    // Tray item
    let mut tray = TrayItem::new("Barium", "net.olback.Barium")?;
    tray.add_label("Barium")?;
    tray.add_menu_item("Show", clone!(@strong mwe => move || mwe.show()))?;
    tray.add_menu_item("Hide", clone!(@strong mwe => move || mwe.hide()))?;
    tray.add_menu_item("Quit", clone!(@strong mwe => move || mwe.quit()))?;

    let ui_ref = ui::Ui::build(
        &main_builder,
        keys_ready,
        Arc::clone(&servers)
    )?;
    info!("{:#?}", ui_ref);

    // Connect on activate
    application.connect_activate(move |app| {
        main_window.set_application(Some(app));
        main_window.show_all();
    });

    // Run the application
    application.run(&env::args().collect::<Vec<String>>());

    padlock::mutex_lock(&servers, |s| s.save())?;

    info!("Closing...");

    Ok(())

}

fn main() {

    // Attempt to show an error when someting panics
    #[cfg(not(debug_assertions))]
    std::panic::set_hook(Box::new(panic_handler::panic_handler));

    logger::configure(Some(CONFIG.log_level)).unwrap();

    run_app().unwrap();

}
