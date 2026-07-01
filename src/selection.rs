use iced_selection::text::{Catalog, Style, StyleFn};

use super::Theme;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(primary)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn primary(theme: &Theme) -> Style {
    Style {
        color: Some(theme.colors().primary.text),
        selection: theme.colors().primary.container,
    }
}
