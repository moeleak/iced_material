use iced_widget::core::{Background, Border, Color};
use iced_widget::text_editor::{Catalog, Status, Style, StyleFn};

use crate::Theme;
use crate::tokens;
use crate::utils::{disabled_text, state_layer};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;
    let primary = theme.colors().primary;

    let active = Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            color: theme.colors().outline.color,
            width: tokens::component::text_field::OUTLINE_WIDTH,
            radius: tokens::component::text_field::CONTAINER_SHAPE.into(),
        },
        placeholder: surface.text_variant,
        value: surface.text,
        selection: disabled_text(primary.color),
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            border: Border {
                color: surface.text,
                ..active.border
            },
            ..active
        },
        Status::Focused { .. } => Style {
            border: Border {
                color: primary.color,
                width: tokens::component::text_field::FOCUS_OUTLINE_WIDTH,
                ..active.border
            },
            ..active
        },
        Status::Disabled => Style {
            background: Color::TRANSPARENT.into(),
            border: Border {
                color: state_layer(
                    surface.text,
                    tokens::component::text_field::DISABLED_OUTLINE_OPACITY,
                ),
                ..active.border
            },
            placeholder: state_layer(
                surface.text,
                tokens::component::text_field::DISABLED_INPUT_TEXT_OPACITY,
            ),
            value: state_layer(
                surface.text,
                tokens::component::text_field::DISABLED_INPUT_TEXT_OPACITY,
            ),
            selection: disabled_text(surface.text),
        },
    }
}

#[cfg(test)]
#[path = "../../../tests/design/style/text_editor.rs"]
mod tests;
