use {
    crate::{get_obj, error::BariumResult},
    std::{rc::Rc, cell::RefCell},
    gtk::{Builder, Stack, prelude::*}
};

pub struct MainStack {
    view: RefCell<String>,
    stack: Stack
}

impl MainStack {

    pub fn build(builder: &Builder) -> BariumResult<Rc<Self>> {

        Ok(
            Rc::new(
                Self {
                    view: RefCell::new(String::new()),
                    stack: get_obj!(builder, "main_stack")
                }
            )
        )

    }

    pub fn show_setup(&self) {

        self.stack.set_visible_child_name("setup");

    }

    pub fn show_chat(&self) {

        self.stack.set_visible_child_name("chat");

    }

    pub fn show_settings(&self) {

        let current_view = self.stack.get_visible_child_name().map(|v| v.to_string()).unwrap();
        if current_view != "settings" && current_view != "keygen" {
            self.view.replace(current_view);
            self.stack.set_visible_child_name("settings");
        }

    }

    pub fn close_settings(&self) {

        self.stack.set_visible_child_name(&self.view.borrow());

    }

}
