//! ModHost's setup for [`jsglue`].

use crate::Result;
use app_config::AppConfig;
use jsglue::{config::GlueConfig, framework::Framework, glue::Glue};

/// Create a new [`Glue`] instance.
/// This will build the UI (see [`crate::ui::build_ui`]).
#[cfg(debug_assertions)]
pub async fn make_glue(config: &AppConfig) -> Result<Glue> {
    use std::path::PathBuf;

    let dir = format!("{}/../../ui", env!("CARGO_MANIFEST_DIR"));

    crate::ui::build_ui(config, &PathBuf::from(&dir)).await?;

    Ok(Glue::new(
        GlueConfig::builder()
            .base("http://localhost:4001")
            .project(dir)
            .cmd("bun")
            .arg("run")
            .arg("dev")
            .framework(Framework::Vite("/vite-hmr"))
            .env(config.ui.env())
            .build()?,
    ))
}

/// Create a new [`Glue`] instance.
/// This will build the UI (see [`crate::ui::build_ui`]).
#[cfg(not(debug_assertions))]
pub async fn make_glue(config: &AppConfig) -> Result<Glue> {
    Ok(Glue::new(
        GlueConfig::builder()
            .dir(crate::ui::build_ui(config).await?)
            .base("http://localhost:4001")
            .project(format!("{}/../../ui", env!("CARGO_MANIFEST_DIR")))
            .cmd("bun")
            .arg("run")
            .arg("dev")
            .framework(Framework::Vite("/vite-hmr"))
            .env(config.ui.env())
            .build()?,
    ))
}
