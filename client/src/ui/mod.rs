use gtk::prelude::*;
use crate::get_obj;

pub struct Ui {
    pub main_window: gtk::ApplicationWindow
}

impl Ui {

    pub fn build(app: &gtk::Application, builder: &gtk::Builder) -> Self {

        let main_window: gtk::ApplicationWindow = get_obj!(builder, "main_window");
        main_window.set_application(Some(app));
        main_window.show();

        Self {
            main_window: main_window
        }

    }

}
