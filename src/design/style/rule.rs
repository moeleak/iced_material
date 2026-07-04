use iced_widget::core::border::Radius;
use iced_widget::rule::{Catalog, FillMode, Style, StyleFn};

use crate::Theme;
use crate::tokens;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(inset)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn inset(theme: &Theme) -> Style {
    Style {
        color: theme.colors().outline.variant,
        fill_mode: FillMode::AsymmetricPadding(
            tokens::component::divider::LIST_ITEM_LEADING_SPACE,
            tokens::component::divider::LIST_ITEM_TRAILING_SPACE,
        ),
        radius: Radius::default(),
        snap: cfg!(feature = "crisp"),
    }
}
pub fn full_width(theme: &Theme) -> Style {
    Style {
        color: theme.colors().outline.variant,
        fill_mode: FillMode::Full,
        radius: Radius::default(),
        snap: cfg!(feature = "crisp"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inset_rule_uses_m3_list_divider_spacing() {
        let style = inset(&Theme::Light);

        assert_eq!(style.color, Theme::Light.colors().outline.variant);
        assert_eq!(
            style.fill_mode,
            FillMode::AsymmetricPadding(
                tokens::component::divider::LIST_ITEM_LEADING_SPACE,
                tokens::component::divider::LIST_ITEM_TRAILING_SPACE
            )
        );
    }
}
