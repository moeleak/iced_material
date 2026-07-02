//! Material 3 snackbar surface constructors.

use iced_widget::button::{Status, Style};
use iced_widget::core::text as core_text;
use iced_widget::core::{Background, Border, Color, Element, Length, Padding, alignment, border};
use iced_widget::text;
use iced_widget::{Button, Container, Row, Text};

use super::absolute_line_height;
use crate::utils::{shadow_from_level, state_layer};
use crate::{Theme, fonts, tokens};

/// Creates a single-line snackbar.
pub fn single_line<'a, Message, Renderer>(
    message: impl text::IntoFragment<'a>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    surface(
        message,
        None,
        tokens::component::snackbar::WITH_SINGLE_LINE_CONTAINER_HEIGHT,
    )
}

/// Creates a single-line snackbar with one action.
pub fn single_line_with_action<'a, Message, Renderer>(
    message: impl text::IntoFragment<'a>,
    action: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    surface(
        message,
        Some(action.into()),
        tokens::component::snackbar::WITH_SINGLE_LINE_CONTAINER_HEIGHT,
    )
}

/// Creates a two-line snackbar with text wrapping enabled.
pub fn two_line<'a, Message, Renderer>(
    message: impl text::IntoFragment<'a>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    surface(
        message,
        None,
        tokens::component::snackbar::WITH_TWO_LINES_CONTAINER_HEIGHT,
    )
}

/// Creates a two-line snackbar with one action.
pub fn two_line_with_action<'a, Message, Renderer>(
    message: impl text::IntoFragment<'a>,
    action: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    surface(
        message,
        Some(action.into()),
        tokens::component::snackbar::WITH_TWO_LINES_CONTAINER_HEIGHT,
    )
}

/// Creates a snackbar text action button.
pub fn action<'a, Message, Renderer>(
    label: impl text::IntoFragment<'a>,
) -> Button<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    let label_text = tokens::component::snackbar::ACTION_LABEL_TEXT;

    Button::new(
        Container::new(
            Text::new(label)
                .size(label_text.size)
                .line_height(absolute_line_height(label_text.line_height)),
        )
        .height(Length::Fixed(tokens::component::button::CONTAINER_HEIGHT))
        .padding(Padding {
            top: 0.0,
            right: tokens::component::button::TRAILING_SPACE,
            bottom: 0.0,
            left: tokens::component::button::LEADING_SPACE,
        })
        .align_y(alignment::Vertical::Center),
    )
    .height(Length::Fixed(tokens::component::button::CONTAINER_HEIGHT))
    .padding(Padding::ZERO)
    .style(action_style)
}

/// Creates a snackbar icon action, typically used for dismiss.
pub fn icon_action<'a, Message, Renderer>(
    icon_name: impl text::IntoFragment<'a>,
) -> Button<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
    iced_widget::core::Font: Into<Renderer::Font>,
{
    Button::new(
        Container::new(fonts::icon(
            icon_name,
            tokens::component::snackbar::ICON_SIZE,
        ))
        .center_x(Length::Fixed(
            tokens::component::icon_button::CONTAINER_WIDTH,
        ))
        .center_y(Length::Fixed(
            tokens::component::icon_button::CONTAINER_HEIGHT,
        )),
    )
    .width(Length::Fixed(
        tokens::component::icon_button::CONTAINER_WIDTH,
    ))
    .height(Length::Fixed(
        tokens::component::icon_button::CONTAINER_HEIGHT,
    ))
    .padding(Padding::ZERO)
    .style(icon_action_style)
}

fn surface<'a, Message, Renderer>(
    message: impl text::IntoFragment<'a>,
    action: Option<Element<'a, Message, Theme, Renderer>>,
    height: f32,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    let supporting_text = tokens::component::snackbar::SUPPORTING_TEXT;
    let mut content = Row::new()
        .push(
            Text::new(message)
                .size(supporting_text.size)
                .line_height(absolute_line_height(supporting_text.line_height))
                .wrapping(text::Wrapping::Word)
                .width(Length::Fill),
        )
        .spacing(8)
        .align_y(alignment::Vertical::Center);

    if let Some(action) = action {
        content = content.push(action);
    }

    Container::new(content)
        .height(Length::Fixed(height))
        .width(Length::Fill)
        .padding(Padding {
            top: 0.0,
            right: 8.0,
            bottom: 0.0,
            left: 16.0,
        })
        .align_y(alignment::Vertical::Center)
        .style(container_style)
}

fn container_style(theme: &Theme) -> iced_widget::container::Style {
    let colors = theme.colors();

    // M3 snackbars use inverse roles, so dark themes intentionally get
    // a light snackbar surface for contrast against the app surface.
    iced_widget::container::Style {
        background: Some(Background::Color(colors.inverse.inverse_surface)),
        text_color: Some(colors.inverse.inverse_surface_text),
        border: border::rounded(tokens::component::snackbar::CONTAINER_SHAPE),
        shadow: shadow_from_level(
            tokens::component::snackbar::CONTAINER_ELEVATION_LEVEL,
            colors.shadow,
        ),
        snap: cfg!(feature = "crisp"),
    }
}

fn action_style(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();
    let foreground = colors.inverse.inverse_primary;
    let active = Style {
        background: None,
        text_color: foreground,
        border: border::rounded(tokens::component::button::CONTAINER_SHAPE),
        shadow: Default::default(),
        snap: cfg!(feature = "crisp"),
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            background: Some(Background::Color(state_layer(
                foreground,
                tokens::state::HOVER_STATE_LAYER_OPACITY,
            ))),
            ..active
        },
        Status::Pressed => Style {
            background: Some(Background::Color(state_layer(
                foreground,
                tokens::state::PRESSED_STATE_LAYER_OPACITY,
            ))),
            ..active
        },
        Status::Disabled => Style {
            text_color: Color {
                a: tokens::state::DISABLED_LABEL_TEXT_OPACITY,
                ..foreground
            },
            ..active
        },
    }
}

fn icon_action_style(theme: &Theme, status: Status) -> Style {
    let colors = theme.colors();
    let foreground = colors.inverse.inverse_surface_text;
    let active = Style {
        background: None,
        text_color: foreground,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: tokens::component::icon_button::CONTAINER_SHAPE.into(),
        },
        shadow: Default::default(),
        snap: cfg!(feature = "crisp"),
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            background: Some(Background::Color(state_layer(
                foreground,
                tokens::state::HOVER_STATE_LAYER_OPACITY,
            ))),
            ..active
        },
        Status::Pressed => Style {
            background: Some(Background::Color(state_layer(
                foreground,
                tokens::state::PRESSED_STATE_LAYER_OPACITY,
            ))),
            ..active
        },
        Status::Disabled => Style {
            text_color: Color {
                a: tokens::state::DISABLED_LABEL_TEXT_OPACITY,
                ..foreground
            },
            ..active
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snackbar_container_uses_inverse_surface_tokens_in_light_and_dark() {
        for theme in [Theme::Light, Theme::Dark] {
            let colors = theme.colors();
            let style = container_style(&theme);

            assert_eq!(
                style.background,
                Some(Background::Color(colors.inverse.inverse_surface))
            );
            assert_eq!(style.text_color, Some(colors.inverse.inverse_surface_text));
            assert_eq!(
                style.border.radius.top_left,
                tokens::component::snackbar::CONTAINER_SHAPE
            );
            assert_eq!(style.shadow.offset.y, 4.0);
            assert_eq!(style.shadow.blur_radius, 8.0);
        }
    }

    #[test]
    fn snackbar_actions_use_inverse_tokens_in_light_and_dark() {
        for theme in [Theme::Light, Theme::Dark] {
            let colors = theme.colors();
            let action = action_style(&theme, Status::Active);
            let icon = icon_action_style(&theme, Status::Active);

            assert_eq!(action.text_color, colors.inverse.inverse_primary);
            assert_eq!(action.background, None);
            assert_eq!(icon.text_color, colors.inverse.inverse_surface_text);
            assert_eq!(icon.background, None);
        }
    }
}
