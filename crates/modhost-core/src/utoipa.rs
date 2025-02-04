//! Helpers for working with [`utoipa`] types.

pub use utoipa::{openapi::schema::ComponentsBuilder, ToResponse, ToSchema};

/// Generate a function to add types to a utoipa [`ComponentBuilder`].
#[macro_export]
macro_rules! utoipa_types {
    [$($pat: ty),+ $(,)?] => {
        /// Add types to a [`$crate::utoipa::ComponentsBuilder`].
        pub fn add_types(components: $crate::utoipa::ComponentsBuilder) -> $crate::utoipa::ComponentsBuilder {
            use $crate::utoipa::{ToResponse, ToSchema};

            let mut schemas = Vec::new();
            let mut responses = Vec::new();

            $(
                <$pat>::schemas(&mut schemas);
                responses.push(<$pat>::response());
            )+

            components
                .schemas_from_iter(schemas)
                .responses_from_iter(responses)
        }
    };
}
