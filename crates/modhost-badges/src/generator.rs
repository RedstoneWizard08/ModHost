//! The badge generator.

use crate::{
    logo::resolve_simpleicons_logo,
    models::{BadgeOptions, BadgeStyle},
};
use modhost_core::{AppError, Result};
use rsbadges::{Badge, Style};

/// The default blue color for the badge message.
pub const DEFAULT_BLUE: &str = "#007ec6";

/// The default gray color for the badge label.
pub const DEFAULT_GRAY: &str = "#555555";

/// Generate a badge.
pub async fn generate_badge(opts: BadgeOptions) -> Result<String> {
    let logo = if let Some(logo) = opts.logo {
        resolve_simpleicons_logo(&logo)
            .await
            .ok_or(AppError::NoLogo(logo))?
    } else {
        Default::default()
    };

    let badge = Badge {
        badge_link: opts.link.unwrap_or_default(),
        badge_title: opts.title.unwrap_or_default(),
        embed_logo: !logo.is_empty(),
        logo,
        label_text: opts.label_text,
        label_color: opts.label_color.unwrap_or(DEFAULT_GRAY.into()),
        label_link: opts.label_link.unwrap_or_default(),
        label_title: opts.label_title.unwrap_or_default(),
        msg_text: opts.msg_text,
        msg_color: opts.msg_color.unwrap_or(DEFAULT_BLUE.into()),
        msg_link: opts.msg_link.unwrap_or_default(),
        msg_title: opts.msg_title.unwrap_or_default(),
    };

    let badge = match opts.style.unwrap_or_default() {
        BadgeStyle::Plastic => Style::Plastic(badge),
        BadgeStyle::Flat => Style::Flat(badge),
        BadgeStyle::FlatSquare => Style::FlatSquare(badge),
        BadgeStyle::ForTheBadge => Style::ForTheBadge(badge),
        BadgeStyle::Social => Style::Social(badge),
    };

    Ok(badge.generate_svg()?)
}
