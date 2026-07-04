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
mod tests {
    use super::*;

    #[test]
    fn default_text_editor_uses_m3_outlined_field_tokens() {
        let theme = Theme::Light;
        let colors = theme.colors();

        let active = default(&theme, Status::Active);
        assert_eq!(active.background, Background::Color(Color::TRANSPARENT));
        assert_eq!(active.border.color, colors.outline.color);
        assert_eq!(
            active.border.width,
            tokens::component::text_field::OUTLINE_WIDTH
        );

        let focused = default(&theme, Status::Focused { is_hovered: false });
        assert_eq!(focused.border.color, colors.primary.color);
        assert_eq!(
            focused.border.width,
            tokens::component::text_field::FOCUS_OUTLINE_WIDTH
        );
        assert_eq!(focused.placeholder, colors.surface.text_variant);

        let disabled = default(&theme, Status::Disabled);
        assert_eq!(
            disabled.border.color.a,
            tokens::component::text_field::DISABLED_OUTLINE_OPACITY
        );
        assert_eq!(
            disabled.placeholder.a,
            tokens::component::text_field::DISABLED_INPUT_TEXT_OPACITY
        );
        assert_eq!(
            disabled.value.a,
            tokens::component::text_field::DISABLED_INPUT_TEXT_OPACITY
        );
    }
}
