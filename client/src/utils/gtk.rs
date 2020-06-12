use gtk::{Entry, prelude::*};

pub fn entry_get_text(entry: &Entry) -> String {

    entry
        .get_text()
        .map(|t| t.to_string())
        .unwrap_or("".into())

}
