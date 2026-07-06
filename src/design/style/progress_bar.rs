use iced_widget::core::{Background, border};
use iced_widget::progress_bar::{Catalog, Style, StyleFn};

use crate::Theme;
use crate::tokens;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(theme: &Theme) -> Style {
    Style {
        background: Background::Color(theme.colors().surface.container.highest),
        bar: Background::Color(theme.colors().primary.color),
        border: border::rounded(tokens::component::linear_progress::TRACK_SHAPE),
    }
}

#[cfg(test)]
#[path = "../../../tests/design/style/progress_bar.rs"]
mod tests;
