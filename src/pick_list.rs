use iced_widget::core::{Background, Border, Color};
use iced_widget::overlay::menu as overlay_menu;
use iced_widget::pick_list::{Catalog, Status, Style, StyleFn};

use super::Theme;
use crate::tokens;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style {
        class(self, status)
    }

    fn default_menu<'a>() -> <Self as overlay_menu::Catalog>::Class<'a> {
        Box::new(crate::menu::outlined_select)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();
    let surface = colors.surface;

    let active = Style {
        text_color: surface.text,
        placeholder_color: surface.text_variant,
        handle_color: surface.text_variant,
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            color: colors.outline.color,
            width: tokens::component::select::TEXT_FIELD_OUTLINE_WIDTH,
            radius: tokens::component::select::TEXT_FIELD_CONTAINER_SHAPE.into(),
        },
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            border: Border {
                color: surface.text,
                width: tokens::component::select::TEXT_FIELD_HOVER_OUTLINE_WIDTH,
                ..active.border
            },
            ..active
        },
        Status::Opened { .. } => Style {
            border: Border {
                color: colors.primary.color,
                width: tokens::component::select::TEXT_FIELD_FOCUS_OUTLINE_WIDTH,
                ..active.border
            },
            handle_color: colors.primary.color,
            ..active
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pick_list_uses_m3_outlined_select_tokens() {
        let theme = Theme::Light;
        let colors = theme.colors();

        let active = default(&theme, Status::Active);
        assert_eq!(active.background, Background::Color(Color::TRANSPARENT));
        assert_eq!(active.border.color, colors.outline.color);
        assert_eq!(
            active.border.width,
            tokens::component::select::TEXT_FIELD_OUTLINE_WIDTH
        );

        let hovered = default(&theme, Status::Hovered);
        assert_eq!(hovered.border.color, colors.surface.text);
        assert_eq!(
            hovered.border.width,
            tokens::component::select::TEXT_FIELD_HOVER_OUTLINE_WIDTH
        );

        let opened = default(&theme, Status::Opened { is_hovered: false });
        assert_eq!(opened.border.color, colors.primary.color);
        assert_eq!(
            opened.border.width,
            tokens::component::select::TEXT_FIELD_FOCUS_OUTLINE_WIDTH
        );
        assert_eq!(opened.handle_color, colors.primary.color);
    }

    #[test]
    fn default_pick_list_menu_uses_m3_outlined_select_menu_tokens() {
        let theme = Theme::Light;
        let colors = theme.colors();
        let menu_class = <Theme as Catalog>::default_menu();
        let menu = <Theme as overlay_menu::Catalog>::style(&theme, &menu_class);

        assert_eq!(
            menu.selected_background,
            Background::Color(colors.surface.container.highest)
        );
        assert_eq!(menu.selected_text_color, colors.surface.text);
    }
}
