use gio::{Notification, ThemedIcon};

pub fn new<T: Into<String>, B: Into<String>>(title: T, body: B) -> Notification {

    let themed_icon = ThemedIcon::new("net.olback.Barium");

    let notif = Notification::new(&title.into());
    notif.set_body(Some(&body.into()));
    notif.set_icon(&themed_icon);

    notif

}

pub fn new_urgent<T: Into<String>, B: Into<String>>(title: T, body: B) -> Notification {

    let notif = new(title.into(), body.into());
    notif.set_priority(gio::NotificationPriority::High);

    notif

}
