use {
    gtk::{Entry, prelude::*},
    regex::Regex,
    lazy_static::lazy_static,
};

lazy_static! {
    // Regex monstrosity
    static ref URI_RE: Regex = Regex::new("(?P<url>https?://[-a-zA-Z0-9@:%._\\+~#=]{1,256}\\.[a-zA-Z0-9()]{1,6}\\b([-a-zA-Z0-9()@:%_\\+.~#?&//=]*))").unwrap();
}

pub fn entry_get_text(entry: &Entry) -> String {

    entry
        .get_text()
        .map(|t| t.to_string())
        .unwrap_or("".into())

}

pub fn escape_markdown(body: &str) -> String {

    let mut lines = body.split(" ").map(|l| l.to_string()).collect::<Vec<_>>();

    for line in &mut lines {

        // let line_clone: String = line.clone();
        *line = line.replace('<', "&lt;");
        *line = line.replace('>', "&gt;");
        let find = URI_RE.find(&line);

        if let Some(f) = find {
            *line = line.replace(f.as_str(), &format!("<a href=\"{url}\">{url}</a>", url = f.as_str()));
        }

    }

    lines.join(" ")

}
