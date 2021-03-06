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
                // super::new_err!(format!("{}", err))
                $crate::error::BariumError::new_with_module(format!("{}", err), std::file!(), std::line!(), stringify!($t))
            }
        }
    };
}
