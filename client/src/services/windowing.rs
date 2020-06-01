use glib;

pub enum MainWindowEvent {
    Show,
    Hide,
    Quit
}

#[derive(Clone)]
pub struct MainWindowEvents {
    tx: glib::Sender<MainWindowEvent>
}

impl MainWindowEvents {

    pub fn new() -> (Self, glib::Receiver<MainWindowEvent>) {

        let (tx, rx) = glib::MainContext::channel(glib::Priority::default());

        (Self { tx }, rx)

    }

    pub fn show(&self) {

        self._send(MainWindowEvent::Show);

    }

    pub fn hide(&self) {

        self._send(MainWindowEvent::Hide);

    }

    pub fn quit(&self) {

        self._send(MainWindowEvent::Quit);

    }

    fn _send(&self, value: MainWindowEvent) {

        let _ = self.tx.send(value);

    }

}
