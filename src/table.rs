use iced_widget::core::Background;
use iced_widget::table::{Catalog, Style, StyleFn};

use super::Theme;

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
mod tests {
    use super::*;
    use crate::tokens;

    #[test]
    fn default_table_uses_m3_data_table_outline_color() {
        let theme = Theme::Light;
        let separator = Background::Color(theme.colors().outline.variant);
        let style = default(&theme);

        assert_eq!(style.separator_x, separator);
        assert_eq!(style.separator_y, separator);
        assert_eq!(tokens::component::data_table::OUTLINE_WIDTH, 1.0);
        assert_eq!(tokens::component::data_table::ROW_ITEM_OUTLINE_WIDTH, 1.0);
    }
}
