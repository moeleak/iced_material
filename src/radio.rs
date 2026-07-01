use iced_widget::core::{Background, Color};
use iced_widget::radio::{Catalog, Status, Style, StyleFn};

use super::Theme;
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
mod tests {
    use super::*;
    use iced_widget::core::Background;

    #[test]
    fn default_radio_uses_m3_selected_and_unselected_icon_colors() {
        let theme = Theme::Light;
        let colors = theme.colors();

        let active_unselected = default(&theme, Status::Active { is_selected: false });
        assert_eq!(active_unselected.border_color, colors.surface.text_variant);

        let hovered_unselected = default(&theme, Status::Hovered { is_selected: false });
        assert_eq!(hovered_unselected.border_color, colors.surface.text);
        assert_eq!(
            hovered_unselected.background,
            Background::Color(crate::utils::state_layer(
                colors.surface.text,
                crate::tokens::state::HOVER_STATE_LAYER_OPACITY
            ))
        );

        let hovered_selected = default(&theme, Status::Hovered { is_selected: true });
        assert_eq!(hovered_selected.border_color, colors.primary.color);
        assert_eq!(hovered_selected.dot_color, colors.primary.color);
        assert_eq!(
            active_unselected.border_width,
            tokens::component::radio::OUTER_RING_WIDTH
        );
    }
}
