use iced_widget::button::{Catalog, Status, Style, StyleFn};
use iced_widget::core::{Background, Border, Color, border};

use crate::Theme;
use crate::tokens;
use crate::tokens::component::button::ElevationLevels;
use crate::utils::{
    HOVERED_LAYER_OPACITY, PRESSED_LAYER_OPACITY, disabled_container, disabled_text, mix,
    shadow_from_level, state_layer,
};

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(filled)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

pub fn styled(
    background: Color,
    foreground: Color,
    disabled: Color,
    shadow_color: Color,
    elevation_level: u8,
    status: Status,
) -> Style {
    styled_with_elevations(
        background,
        foreground,
        disabled,
        shadow_color,
        ElevationLevels {
            active: elevation_level,
            hovered: elevation_level + 1,
            pressed: elevation_level,
            disabled: 0,
        },
        status,
    )
}

fn styled_with_elevations(
    background: Color,
    foreground: Color,
    disabled: Color,
    shadow_color: Color,
    elevations: ElevationLevels,
    status: Status,
) -> Style {
    styled_with_elevations_and_shape(
        background,
        foreground,
        disabled,
        shadow_color,
        elevations,
        tokens::component::button::CONTAINER_SHAPE,
        status,
    )
}

fn styled_with_elevations_and_shape(
    background: Color,
    foreground: Color,
    disabled: Color,
    shadow_color: Color,
    elevations: ElevationLevels,
    shape: f32,
    status: Status,
) -> Style {
    let active = Style {
        background: Some(Background::Color(background)),
        text_color: foreground,
        border: border::rounded(shape),
        shadow: shadow_from_level(elevations.active, shadow_color),
        snap: cfg!(feature = "crisp"),
    };

    match status {
        Status::Active => active,
        Status::Pressed => Style {
            background: Some(Background::Color(mix(
                background,
                foreground,
                PRESSED_LAYER_OPACITY,
            ))),
            shadow: shadow_from_level(elevations.pressed, shadow_color),
            ..active
        },
        Status::Hovered => Style {
            background: Some(Background::Color(mix(
                background,
                foreground,
                HOVERED_LAYER_OPACITY,
            ))),
            shadow: shadow_from_level(elevations.hovered, shadow_color),
            ..active
        },
        Status::Disabled => Style {
            background: Some(Background::Color(disabled_container(disabled))),
            text_color: disabled_text(disabled),
            border: border::rounded(shape),
            shadow: shadow_from_level(elevations.disabled, shadow_color),
            ..Default::default()
        },
    }
}

pub fn elevated(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;

    let foreground = theme.colors().primary.color;
    let background = surface.container.low;
    let disabled = surface.text;

    let shadow_color = theme.colors().shadow;

    styled_with_elevations(
        background,
        foreground,
        disabled,
        shadow_color,
        tokens::component::button::ELEVATED_ELEVATION,
        status,
    )
}

pub fn filled(theme: &Theme, status: Status) -> Style {
    let primary = theme.colors().primary;

    let foreground = primary.text;
    let background = primary.color;
    let disabled = theme.colors().surface.text;

    let shadow_color = theme.colors().shadow;

    styled_with_elevations(
        background,
        foreground,
        disabled,
        shadow_color,
        tokens::component::button::FILLED_ELEVATION,
        status,
    )
}

pub fn filled_tonal(theme: &Theme, status: Status) -> Style {
    let secondary = theme.colors().secondary;

    let foreground = secondary.container_text;
    let background = secondary.container;
    let disabled = theme.colors().surface.text;
    let shadow_color = theme.colors().shadow;

    styled_with_elevations(
        background,
        foreground,
        disabled,
        shadow_color,
        tokens::component::button::FILLED_TONAL_ELEVATION,
        status,
    )
}

pub fn outlined(theme: &Theme, status: Status) -> Style {
    let foreground = theme.colors().primary.color;
    let background = Color::TRANSPARENT;
    let disabled = theme.colors().surface.text;

    let outline = theme.colors().outline.color;

    let border = match status {
        Status::Active | Status::Pressed | Status::Hovered => Border {
            color: outline,
            width: tokens::component::button::OUTLINED_OUTLINE_WIDTH,
            radius: tokens::component::button::CONTAINER_SHAPE.into(),
        },
        Status::Disabled => Border {
            color: disabled_container(disabled),
            width: tokens::component::button::OUTLINED_OUTLINE_WIDTH,
            radius: tokens::component::button::CONTAINER_SHAPE.into(),
        },
    };

    let mut style = styled_with_elevations(
        background,
        foreground,
        disabled,
        Color::TRANSPARENT,
        tokens::component::button::FLAT_ELEVATION,
        status,
    );

    if matches!(status, Status::Disabled) {
        style.background = None;
    }

    Style { border, ..style }
}

pub fn text(theme: &Theme, status: Status) -> Style {
    let foreground = theme.colors().primary.color;
    let background = Color::TRANSPARENT;
    let disabled = theme.colors().surface.text;

    let style = styled_with_elevations(
        background,
        foreground,
        disabled,
        Color::TRANSPARENT,
        tokens::component::button::FLAT_ELEVATION,
        status,
    );

    match status {
        Status::Hovered | Status::Pressed => style,
        Status::Active | Status::Disabled => Style {
            background: None,
            ..style
        },
    }
}

fn fab_style_with_tokens(
    theme: &Theme,
    background: Color,
    foreground: Color,
    elevations: ElevationLevels,
    shape: f32,
    status: Status,
) -> Style {
    styled_with_elevations_and_shape(
        background,
        foreground,
        theme.colors().surface.text,
        theme.colors().shadow,
        elevations,
        shape,
        status,
    )
}

fn fab_style(theme: &Theme, background: Color, foreground: Color, status: Status) -> Style {
    fab_style_with_tokens(
        theme,
        background,
        foreground,
        tokens::component::fab::ELEVATION,
        tokens::component::fab::CONTAINER_SHAPE,
        status,
    )
}

fn fab_small_style(theme: &Theme, background: Color, foreground: Color, status: Status) -> Style {
    fab_style_with_tokens(
        theme,
        background,
        foreground,
        tokens::component::fab::ELEVATION,
        tokens::component::fab::SMALL_CONTAINER_SHAPE,
        status,
    )
}

fn fab_large_style(theme: &Theme, background: Color, foreground: Color, status: Status) -> Style {
    fab_style_with_tokens(
        theme,
        background,
        foreground,
        tokens::component::fab::ELEVATION,
        tokens::component::fab::LARGE_CONTAINER_SHAPE,
        status,
    )
}

fn extended_fab_style(
    theme: &Theme,
    background: Color,
    foreground: Color,
    status: Status,
) -> Style {
    fab_style_with_tokens(
        theme,
        background,
        foreground,
        tokens::component::fab::EXTENDED_ELEVATION,
        tokens::component::fab::EXTENDED_CONTAINER_SHAPE,
        status,
    )
}

pub fn fab_primary(theme: &Theme, status: Status) -> Style {
    let primary = theme.colors().primary;

    fab_style(theme, primary.color, primary.text, status)
}

pub fn fab_primary_small(theme: &Theme, status: Status) -> Style {
    let primary = theme.colors().primary;

    fab_small_style(theme, primary.color, primary.text, status)
}

pub fn fab_primary_large(theme: &Theme, status: Status) -> Style {
    let primary = theme.colors().primary;

    fab_large_style(theme, primary.color, primary.text, status)
}

pub fn fab_secondary(theme: &Theme, status: Status) -> Style {
    let secondary = theme.colors().secondary;

    fab_style(theme, secondary.color, secondary.text, status)
}

pub fn fab_secondary_small(theme: &Theme, status: Status) -> Style {
    let secondary = theme.colors().secondary;

    fab_small_style(theme, secondary.color, secondary.text, status)
}

pub fn fab_secondary_large(theme: &Theme, status: Status) -> Style {
    let secondary = theme.colors().secondary;

    fab_large_style(theme, secondary.color, secondary.text, status)
}

pub fn fab_tertiary(theme: &Theme, status: Status) -> Style {
    let tertiary = theme.colors().tertiary;

    fab_style(theme, tertiary.color, tertiary.text, status)
}

pub fn fab_tertiary_small(theme: &Theme, status: Status) -> Style {
    let tertiary = theme.colors().tertiary;

    fab_small_style(theme, tertiary.color, tertiary.text, status)
}

pub fn fab_tertiary_large(theme: &Theme, status: Status) -> Style {
    let tertiary = theme.colors().tertiary;

    fab_large_style(theme, tertiary.color, tertiary.text, status)
}

pub fn fab_surface(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    fab_style(
        theme,
        colors.surface.container.high,
        colors.primary.color,
        status,
    )
}

pub fn fab_surface_small(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    fab_small_style(
        theme,
        colors.surface.container.high,
        colors.primary.color,
        status,
    )
}

pub fn fab_surface_large(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    fab_large_style(
        theme,
        colors.surface.container.high,
        colors.primary.color,
        status,
    )
}

pub fn extended_fab_primary(theme: &Theme, status: Status) -> Style {
    let primary = theme.colors().primary;

    extended_fab_style(theme, primary.color, primary.text, status)
}

pub fn extended_fab_secondary(theme: &Theme, status: Status) -> Style {
    let secondary = theme.colors().secondary;

    extended_fab_style(theme, secondary.color, secondary.text, status)
}

pub fn extended_fab_tertiary(theme: &Theme, status: Status) -> Style {
    let tertiary = theme.colors().tertiary;

    extended_fab_style(theme, tertiary.color, tertiary.text, status)
}

pub fn extended_fab_surface(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    extended_fab_style(
        theme,
        colors.surface.container.high,
        colors.primary.color,
        status,
    )
}

pub fn icon(theme: &Theme, status: Status) -> Style {
    let surface = theme.colors().surface;

    let active = Style {
        background: None,
        text_color: surface.text_variant,
        border: border::rounded(tokens::component::icon_button::CONTAINER_SHAPE),
        shadow: shadow_from_level(0, Color::TRANSPARENT),
        snap: cfg!(feature = "crisp"),
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            background: Some(Background::Color(mix(
                Color::TRANSPARENT,
                surface.text_variant,
                HOVERED_LAYER_OPACITY,
            ))),
            ..active
        },
        Status::Pressed => Style {
            background: Some(Background::Color(mix(
                Color::TRANSPARENT,
                surface.text_variant,
                PRESSED_LAYER_OPACITY,
            ))),
            ..active
        },
        Status::Disabled => Style {
            text_color: Color {
                a: tokens::component::icon_button::DISABLED_ICON_OPACITY,
                ..surface.text
            },
            ..active
        },
    }
}

pub fn filled_icon(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    styled_with_elevations_and_shape(
        colors.primary.color,
        colors.primary.text,
        colors.surface.text,
        Color::TRANSPARENT,
        tokens::component::button::FLAT_ELEVATION,
        tokens::component::icon_button::CONTAINER_SHAPE,
        status,
    )
}

pub fn filled_tonal_icon(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    styled_with_elevations_and_shape(
        colors.secondary.container,
        colors.secondary.container_text,
        colors.surface.text,
        Color::TRANSPARENT,
        tokens::component::button::FLAT_ELEVATION,
        tokens::component::icon_button::CONTAINER_SHAPE,
        status,
    )
}

pub fn outlined_icon(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();
    let surface = colors.surface;

    let active = Style {
        background: None,
        text_color: surface.text_variant,
        border: Border {
            color: colors.outline.color,
            width: tokens::component::icon_button::OUTLINED_OUTLINE_WIDTH,
            radius: tokens::component::icon_button::CONTAINER_SHAPE.into(),
        },
        shadow: shadow_from_level(0, Color::TRANSPARENT),
        snap: cfg!(feature = "crisp"),
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            background: Some(Background::Color(mix(
                Color::TRANSPARENT,
                surface.text_variant,
                HOVERED_LAYER_OPACITY,
            ))),
            ..active
        },
        Status::Pressed => Style {
            background: Some(Background::Color(mix(
                Color::TRANSPARENT,
                surface.text,
                PRESSED_LAYER_OPACITY,
            ))),
            text_color: surface.text,
            ..active
        },
        Status::Disabled => Style {
            text_color: Color {
                a: tokens::component::icon_button::DISABLED_ICON_OPACITY,
                ..surface.text
            },
            border: Border {
                color: Color {
                    a: tokens::component::icon_button::OUTLINED_DISABLED_OUTLINE_OPACITY,
                    ..surface.text
                },
                ..active.border
            },
            ..active
        },
    }
}

#[derive(Debug, Clone, Copy)]
struct ChipSpec {
    background: Option<Color>,
    foreground: Color,
    outline: Option<Color>,
    disabled_background: Option<Color>,
    disabled_outline: Option<Color>,
    hover_layer: Color,
    pressed_layer: Color,
    elevations: ElevationLevels,
    shadow_color: Color,
}

fn chip_style(spec: ChipSpec, status: Status) -> Style {
    let border = Border {
        color: spec.outline.unwrap_or(Color::TRANSPARENT),
        width: spec
            .outline
            .map_or(tokens::component::chip::SELECTED_OUTLINE_WIDTH, |_| {
                tokens::component::chip::OUTLINE_WIDTH
            }),
        radius: tokens::component::chip::CONTAINER_SHAPE.into(),
    };

    let active = Style {
        background: spec.background.map(Background::Color),
        text_color: spec.foreground,
        border,
        shadow: shadow_from_level(spec.elevations.active, spec.shadow_color),
        snap: cfg!(feature = "crisp"),
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            background: Some(Background::Color(chip_state_background(
                spec.background,
                spec.hover_layer,
                HOVERED_LAYER_OPACITY,
            ))),
            shadow: shadow_from_level(spec.elevations.hovered, spec.shadow_color),
            ..active
        },
        Status::Pressed => Style {
            background: Some(Background::Color(chip_state_background(
                spec.background,
                spec.pressed_layer,
                PRESSED_LAYER_OPACITY,
            ))),
            shadow: shadow_from_level(spec.elevations.pressed, spec.shadow_color),
            ..active
        },
        Status::Disabled => Style {
            background: spec.disabled_background.map(Background::Color),
            text_color: Color {
                a: tokens::component::chip::DISABLED_LABEL_TEXT_OPACITY,
                ..spec.foreground
            },
            border: Border {
                color: spec.disabled_outline.unwrap_or(Color::TRANSPARENT),
                ..border
            },
            shadow: shadow_from_level(spec.elevations.disabled, spec.shadow_color),
            snap: cfg!(feature = "crisp"),
        },
    }
}

fn chip_state_background(background: Option<Color>, layer: Color, opacity: f32) -> Color {
    background.map_or_else(
        || state_layer(layer, opacity),
        |background| mix(background, layer, opacity),
    )
}

fn outlined_chip_spec(
    foreground: Color,
    outline: Color,
    disabled_color: Color,
    pressed_layer: Color,
) -> ChipSpec {
    ChipSpec {
        background: None,
        foreground,
        outline: Some(outline),
        disabled_background: None,
        disabled_outline: Some(Color {
            a: tokens::component::chip::DISABLED_OUTLINE_OPACITY,
            ..disabled_color
        }),
        hover_layer: foreground,
        pressed_layer,
        elevations: tokens::component::chip::FLAT_ELEVATION,
        shadow_color: Color::TRANSPARENT,
    }
}

fn elevated_chip_spec(
    background: Color,
    foreground: Color,
    disabled_color: Color,
    shadow_color: Color,
) -> ChipSpec {
    ChipSpec {
        background: Some(background),
        foreground,
        outline: None,
        disabled_background: Some(Color {
            a: tokens::component::chip::DISABLED_CONTAINER_OPACITY,
            ..disabled_color
        }),
        disabled_outline: None,
        hover_layer: foreground,
        pressed_layer: foreground,
        elevations: tokens::component::chip::ELEVATED_ELEVATION,
        shadow_color,
    }
}

pub fn assist_chip(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    chip_style(
        outlined_chip_spec(
            colors.surface.text,
            colors.outline.color,
            colors.surface.text,
            colors.surface.text,
        ),
        status,
    )
}

pub fn elevated_assist_chip(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    chip_style(
        elevated_chip_spec(
            colors.surface.container.low,
            colors.surface.text,
            colors.surface.text,
            colors.shadow,
        ),
        status,
    )
}

pub fn suggestion_chip(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    chip_style(
        outlined_chip_spec(
            colors.surface.text_variant,
            colors.outline.color,
            colors.surface.text,
            colors.surface.text_variant,
        ),
        status,
    )
}

pub fn elevated_suggestion_chip(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    chip_style(
        elevated_chip_spec(
            colors.surface.container.low,
            colors.surface.text_variant,
            colors.surface.text,
            colors.shadow,
        ),
        status,
    )
}

pub fn filter_chip(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    chip_style(
        outlined_chip_spec(
            colors.surface.text_variant,
            colors.outline.color,
            colors.surface.text,
            colors.secondary.container_text,
        ),
        status,
    )
}

pub fn selected_filter_chip(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    chip_style(
        ChipSpec {
            background: Some(colors.secondary.container),
            foreground: colors.secondary.container_text,
            outline: None,
            disabled_background: Some(Color {
                a: tokens::component::chip::DISABLED_CONTAINER_OPACITY,
                ..colors.surface.text
            }),
            disabled_outline: None,
            hover_layer: colors.secondary.container_text,
            pressed_layer: colors.surface.text_variant,
            elevations: tokens::component::chip::SELECTED_FLAT_ELEVATION,
            shadow_color: colors.shadow,
        },
        status,
    )
}

pub fn input_chip(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    chip_style(
        outlined_chip_spec(
            colors.surface.text_variant,
            colors.outline.color,
            colors.surface.text,
            colors.surface.text_variant,
        ),
        status,
    )
}

pub fn selected_input_chip(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();

    chip_style(
        ChipSpec {
            background: Some(colors.secondary.container),
            foreground: colors.secondary.container_text,
            outline: None,
            disabled_background: Some(Color {
                a: tokens::component::chip::DISABLED_CONTAINER_OPACITY,
                ..colors.surface.text
            }),
            disabled_outline: None,
            hover_layer: colors.secondary.container_text,
            pressed_layer: colors.secondary.container_text,
            elevations: tokens::component::chip::FLAT_ELEVATION,
            shadow_color: Color::TRANSPARENT,
        },
        status,
    )
}

#[cfg(test)]
#[path = "../../tests/design/style/button.rs"]
mod tests;
