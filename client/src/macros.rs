#[macro_export]
macro_rules! new_err {
    ($e:expr) => {
        $crate::error::BariumError::new(format!("{}", $e), std::file!(), std::line!())
    };
}

#[macro_export]
macro_rules! is_debug {
    () => {
        if cfg!(debug_assertions) {
            true
        } else {
            std::env::var("BARIUM_DEBUG").is_ok()
        }
    };
}

#[macro_export]
macro_rules! impl_from {
    ($t:ty) => {
        impl From<$t> for BariumError {
            fn from(err: $t) -> BariumError {
                super::new_err!(format!("{}", err))
            }
        }
    };
}

#[macro_export]
macro_rules! get_obj {
    ($builder:expr, $id:expr) => {
        // Catch and panic manually to get useful file and line info
        {
            use gtk::prelude::BuilderExtManual;
            match $builder.get_object($id) {
                Some(o) => o,
                None => panic!("could not get {}", $id)
            }
        }

    };
}

#[macro_export]
macro_rules! resource {
    ($res:expr) => {
        concat!("/net/olback/barium/", $res)
    };
}

#[macro_export]
macro_rules! extract_cert_field_value {
    ($input:expr, $part:expr) => {
        $input.entries_by_nid($part)
        .nth(0)
        .map(|d| d.data())
        .map(|d| d.as_utf8())
        .map(|d| d.unwrap())
        .map(|d| d.to_string())
        .unwrap_or("<Not Part Of Certificate>".into())
    };
}
