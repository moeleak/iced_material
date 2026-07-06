use iced_widget::core::Background;
use iced_widget::table::{Catalog, Style, StyleFn};

use crate::Theme;

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
    let separator = theme.colors().outline.variant;

    Style {
        separator_x: Background::Color(separator),
        separator_y: Background::Color(separator),
    }
}

#[cfg(test)]
#[path = "../../../tests/design/style/table.rs"]
mod tests;
