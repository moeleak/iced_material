use iced_widget::core::{Background, Color, border};
use iced_widget::slider::{Catalog, Handle, HandleShape, Rail, Status, Style, StyleFn};

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
}

pub fn styled(left: Color, right: Color, handle_radius: f32) -> Style {
    Style {
        rail: Rail {
            backgrounds: (left.into(), right.into()),
            width: tokens::component::slider::ACTIVE_TRACK_HEIGHT,
            border: border::rounded(tokens::component::slider::TRACK_SHAPE),
        },
        handle: Handle {
            shape: HandleShape::Circle {
                radius: handle_radius,
            },
            background: Background::Color(left),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
        },
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;
    let primary = theme.colors().primary;
    let active = primary.color;
    let inactive = surface.container.highest;

    match status {
        Status::Active | Status::Hovered | Status::Dragged => {
            styled(active, inactive, tokens::component::slider::HANDLE_RADIUS)
        }
    }
}

#[cfg(test)]
#[path = "../../tests/design/style/slider.rs"]
mod tests;
