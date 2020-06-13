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
        match $builder.get_object($id) {
            Some(o) => o,
            None => panic!("could not get {}", $id)
        }
    };
}
