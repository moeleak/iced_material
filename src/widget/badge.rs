//! Material 3 badge constructors with token-backed layout defaults.

use iced_widget::core::text as core_text;
use iced_widget::core::{alignment, Length, Padding};
use iced_widget::text;
use iced_widget::{Container, Text};

use super::absolute_line_height;
use crate::{badge as badge_style, tokens, Theme};

pub fn small<'a, Message, Renderer>() -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    Container::new(Text::new(""))
        .width(Length::Fixed(tokens::component::badge::SMALL_SIZE))
        .height(Length::Fixed(tokens::component::badge::SMALL_SIZE))
        .style(badge_style::default)
}

pub fn large<'a, Message, Renderer>(
    label: impl text::IntoFragment<'a>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + core_text::Renderer + 'a,
{
    let label_text = tokens::component::badge::LABEL_TEXT;

    Container::new(
        Text::new(label)
            .size(label_text.size)
            .line_height(absolute_line_height(label_text.line_height)),
    )
    .height(Length::Fixed(tokens::component::badge::LARGE_CONTAINER_HEIGHT))
    .max_width(tokens::component::badge::LARGE_CONTAINER_MAX_WIDTH)
    .padding(Padding::from([
        0.0,
        tokens::component::badge::LARGE_HORIZONTAL_SPACE,
    ]))
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center)
    .style(badge_style::default)
}
