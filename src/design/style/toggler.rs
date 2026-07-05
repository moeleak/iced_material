use iced_widget::core::{Background, Color};
use iced_widget::toggler::{Catalog, Status, Style, StyleFn};

use crate::Theme;
use crate::tokens;
use crate::utils::state_layer;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn styled(
    background: Background,
    foreground: Background,
    text_color: Color,
    border: Option<Color>,
) -> Style {
    Style {
        background,
        background_border_width: if border.is_some() {
            tokens::component::switch::TRACK_OUTLINE_WIDTH
        } else {
            0.0
        },
        background_border_color: border.unwrap_or(Color::TRANSPARENT),
        foreground,
        foreground_border_width: 0.0,
        foreground_border_color: Color::TRANSPARENT,
        text_color: Some(text_color),
        border_radius: None,
        padding_ratio: 0.2,
    }
}

pub fn default(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;
    let primary = theme.colors().primary;

    match status {
        Status::Active { is_toggled } => {
            if is_toggled {
                styled(
                    primary.color.into(),
                    primary.text.into(),
                    surface.text,
                    None,
                )
            } else {
                styled(
                    surface.container.highest.into(),
                    theme.colors().outline.color.into(),
                    surface.text,
                    Some(theme.colors().outline.color),
                )
            }
        }
        Status::Hovered { is_toggled } => {
            if is_toggled {
                styled(
                    primary.color.into(),
                    primary.container.into(),
                    surface.text,
                    None,
                )
            } else {
                styled(
                    surface.container.highest.into(),
                    surface.text_variant.into(),
                    surface.text,
                    Some(theme.colors().outline.color),
                )
            }
        }
        Status::Disabled { is_toggled } => {
            if is_toggled {
                styled(
                    state_layer(
                        surface.text,
                        tokens::component::switch::DISABLED_TRACK_OPACITY,
                    )
                    .into(),
                    state_layer(
                        surface.color,
                        tokens::component::switch::DISABLED_SELECTED_HANDLE_OPACITY,
                    )
                    .into(),
                    surface.text,
                    None,
                )
            } else {
                styled(
                    state_layer(
                        surface.container.highest,
                        tokens::component::switch::DISABLED_TRACK_OPACITY,
                    )
                    .into(),
                    state_layer(
                        surface.text,
                        tokens::component::switch::DISABLED_UNSELECTED_HANDLE_OPACITY,
                    )
                    .into(),
                    surface.text,
                    Some(state_layer(
                        surface.text,
                        tokens::component::switch::DISABLED_TRACK_OPACITY,
                    )),
                )
            }
        }
    }
}

#[cfg(test)]
#[path = "../../tests/design/style/toggler.rs"]
mod tests;
