use gtk::{self, StackExt};

pub struct StackSwitcher<'a> {
    stack: &'a gtk::Stack,
    time: u32,
    name: &'a str
}

impl<'a> StackSwitcher<'a> {

    pub fn new(stack: &'a gtk::Stack, name: &'a str) -> Self {

        Self {
            stack: stack,
            time: 200,
            name: name
        }

    }

    pub fn time(mut self, time: u32) -> Self {

        self.time = time;
        self

    }

    pub fn left(&self) {

        self.stack.set_transition_type(gtk::StackTransitionType::SlideLeft);
        self.stack.set_transition_duration(self.time);
        self.stack.set_visible_child_name(self.name);

    }

    pub fn right(&self) {

        self.stack.set_transition_type(gtk::StackTransitionType::SlideRight);
        self.stack.set_transition_duration(self.time);
        self.stack.set_visible_child_name(self.name);

    }

}
