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
mod fs;
mod panic_handler;

use {
    error::BariumResult,
    services::{MainWindowEvent, MainWindowEvents}
};

use {
    glib::{self, clone},
    gio::{SimpleAction, SimpleActionGroup, prelude::*},
    gtk::{
        AboutDialog, Application, ApplicationWindow, Builder, CssProvider, Stack, StyleContext,
        Window, STYLE_PROVIDER_PRIORITY_APPLICATION, prelude::*
    },
    std::{env, rc::Rc, cell::RefCell},
    tray_item::TrayItem
};

fn main() -> BariumResult<()> {

    // Attempt to show an error when someting panics
    #[cfg(not(debug_assertions))]
    std::panic::set_hook(Box::new(panic_handler::panic_handler));

    // Load resources
    resources::load();

    // Create application
    let application = Application::new(Some("net.olback.barium"), Default::default())?;
    Window::set_default_icon_name("net.olback.Barium");

    // Load CSS
    let provider = CssProvider::new();
    provider.load_from_resource(resource!("css/app.css"));
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
    let main_stack: Stack = get_obj!(main_builder, "main_stack");
    let main_stack_current_view = Rc::new(RefCell::new(String::new()));
    let open_settings_action = SimpleAction::new("open-settings", None);
    let close_settings_action = SimpleAction::new("close-settings", None);
    open_settings_action.connect_activate(clone!(@strong main_stack, @strong main_stack_current_view => move |_, _| {
        let current_view = main_stack.get_visible_child_name().unwrap().to_string();
        if current_view != "settings" {
            main_stack_current_view.replace(current_view);
            main_stack.set_visible_child_name("settings");
        }
    }));
    close_settings_action.connect_activate(clone!(@strong main_stack, @strong main_stack_current_view => move |_, _| {
        main_stack.set_visible_child_name(main_stack_current_view.borrow().as_str());
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

    // Tray item
    let mut tray = TrayItem::new("Barium", "net.olback.Barium")?;
    tray.add_label("Barium")?;
    tray.add_menu_item("Show", clone!(@strong mwe => move || mwe.show()))?;
    tray.add_menu_item("Hide", clone!(@strong mwe => move || mwe.hide()))?;
    tray.add_menu_item("Quit", clone!(@strong mwe => move || mwe.quit()))?;

    let ui_ref = ui::Ui::build(&main_builder)?;
    println!("{:#?}", ui_ref);

    // Connect on activate
    application.connect_activate(move |app| {
        main_window.set_application(Some(app));
        main_window.show_all();
    });

    // Run the application
    application.run(&env::args().collect::<Vec<String>>());

    println!("Closing...");

    Ok(())

}
