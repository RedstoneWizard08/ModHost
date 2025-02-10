#[macro_export]
macro_rules! client {
    ($dir: expr) => {
        #[cfg(debug_assertions)]
        mod client {
            use $crate::include_dir::Dir;

            pub const CLIENT_DIR: Option<Dir<'static>> = None;
        }

        #[cfg(not(debug_assertions))]
        mod client {
            use $crate::include_dir::{self, Dir, include_dir};

            pub const CLIENT_DIR: Option<Dir<'static>> = Some(include_dir!($dir));
        }
    };
}
