use iced_widget::container::Style;
use iced_widget::core::{Background, Shadow, border};

use crate::Theme;
use crate::tokens;
use crate::utils::shadow_from_level;

pub fn plain(theme: &Theme) -> Style {
    let inverse = theme.colors().inverse;

    Style {
        text_color: Some(inverse.inverse_surface_text),
        background: Some(Background::Color(inverse.inverse_surface)),
        border: border::rounded(tokens::component::tooltip::PLAIN_CONTAINER_SHAPE),
        shadow: Shadow::default(),
        snap: cfg!(feature = "crisp"),
    }
}

pub fn rich(theme: &Theme) -> Style {
    let colors = theme.colors();

    Style {
        text_color: Some(colors.surface.text_variant),
        background: Some(Background::Color(colors.surface.container.base)),
        border: border::rounded(tokens::component::tooltip::RICH_CONTAINER_SHAPE),
        shadow: shadow_from_level(
            tokens::component::tooltip::RICH_CONTAINER_ELEVATION_LEVEL,
            colors.shadow,
        ),
        snap: cfg!(feature = "crisp"),
    }
}

#[cfg(test)]
#[path = "../../tests/design/style/tooltip.rs"]
mod tests;
