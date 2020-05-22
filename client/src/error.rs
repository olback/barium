use crate::{impl_from, is_debug};

pub type BariumResult<T> = Result<T, BariumError>;

#[derive(Debug)]
pub struct BariumError {
    cause: String,
    file: String,
    line: u32
}

impl BariumError {

    pub fn new<C: Into<String>>(cause: C, file: &str, line: u32) -> Self {

        Self {
            cause: cause.into(),
            file: String::from(file),
            line: line
        }

    }

}

impl std::fmt::Display for BariumError {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {

        if is_debug!() {
            write!(f, "{}#{}: {}", self.file, self.line, self.cause)
        } else {
            write!(f, "{}", self.cause)
        }

    }

}

// impl_from!(std::option::NoneError);
impl_from!(std::io::Error);
impl_from!(serde_json::error::Error);
impl_from!(base64::DecodeError);
// impl_from!(Box<bincode::ErrorKind>);
impl_from!(rsa::errors::Error);
impl_from!(glib::error::BoolError);
impl_from!(tray_item::TIError);
