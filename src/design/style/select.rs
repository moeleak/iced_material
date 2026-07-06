use iced_widget::core::{Background, Border, Color};
use iced_widget::overlay::menu as overlay_menu;
use iced_widget::pick_list::{Catalog, Status, Style, StyleFn};

use crate::Theme;
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
        Box::new(crate::style::menu::outlined_select)
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
#[path = "../../../tests/design/style/select.rs"]
mod tests;
