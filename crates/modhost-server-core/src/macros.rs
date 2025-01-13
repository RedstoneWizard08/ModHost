//! Utility macros for using ModHost.

/// A small utility macro for creating [`crate::ModLoader`] objects.
#[macro_export]
macro_rules! loader {
    ($name: expr) => {
        $crate::models::ModLoader {
            id: $name.to_lowercase(),
            name: $name.into(),
        }
    };
}

/// A small utility macro for creating [`crate::Tag`] objects.
#[macro_export]
macro_rules! tag {
    ($id: expr, $name: expr, $icon: expr) => {
        $crate::models::Tag {
            id: $id.into(),
            name: $name.into(),
            icon: $icon.into(),
        }
    };
}

/// A small utility macro for creating many [`crate::ModLoader`] objects.
#[macro_export]
macro_rules! loaders {
    [$($name: expr),*$(,)?] => {
        vec![$($crate::loader!($name)),*]
    };
}

/// A small utility macro for creating many [`crate::Tag`] objects.
#[macro_export]
macro_rules! tags {
    [$($id: expr, $name: expr, $icon: expr);*$(;)?] => {
        vec![$($crate::tag!($id, $name, $icon)),*]
    };
}
