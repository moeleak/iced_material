use iced_widget::core::{Background, Color};
use iced_widget::radio::{Catalog, Status, Style, StyleFn};

use crate::Theme;
use crate::tokens;
use crate::utils::{HOVERED_LAYER_OPACITY, state_layer};

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
        background: Color::TRANSPARENT.into(),
        dot_color: primary.color,
        border_width: tokens::component::radio::OUTER_RING_WIDTH,
        border_color: primary.color,
        text_color: None,
    };

    match status {
        Status::Active { is_selected } => Style {
            border_color: if is_selected {
                primary.color
            } else {
                surface.text_variant
            },
            ..active
        },
        Status::Hovered { is_selected } => Style {
            background: Background::Color(state_layer(
                if is_selected {
                    primary.color
                } else {
                    surface.text
                },
                HOVERED_LAYER_OPACITY,
            )),
            dot_color: primary.color,
            border_color: if is_selected {
                primary.color
            } else {
                surface.text
            },
            ..active
        },
    }
}

#[cfg(test)]
#[path = "../../tests/design/style/radio.rs"]
mod tests;
