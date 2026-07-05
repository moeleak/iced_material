use iced_widget::core::{Background, border};
use iced_widget::overlay::menu::{Catalog, Style, StyleFn};

use crate::Theme;
use crate::tokens;
use crate::utils::shadow_from_level;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(theme: &Theme) -> Style {
    let colors = theme.colors();
    let surface = colors.surface;

    Style {
        border: border::rounded(tokens::component::menu::CONTAINER_SHAPE),
        background: Background::Color(surface.container.base),
        text_color: surface.text,
        selected_background: Background::Color(colors.secondary.container),
        selected_text_color: colors.secondary.container_text,
        shadow: shadow_from_level(
            tokens::component::menu::CONTAINER_ELEVATION_LEVEL,
            colors.shadow,
        ),
    }
}

pub fn outlined_select(theme: &Theme) -> Style {
    let colors = theme.colors();
    let surface = colors.surface;

    Style {
        border: border::rounded(tokens::component::select::MENU_CONTAINER_SHAPE),
        background: Background::Color(surface.container.base),
        text_color: surface.text,
        selected_background: Background::Color(surface.container.highest),
        selected_text_color: surface.text,
        shadow: shadow_from_level(
            tokens::component::select::MENU_CONTAINER_ELEVATION_LEVEL,
            colors.shadow,
        ),
    }
}

#[cfg(test)]
#[path = "../../tests/design/style/menu.rs"]
mod tests;
