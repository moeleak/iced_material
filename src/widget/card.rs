//! Material 3 card constructors backed by the container style catalog.

use iced_widget::Container;
use iced_widget::core::Element;

use crate::{Theme, container as container_style};

fn styled<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    style: fn(&Theme) -> iced_widget::container::Style,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    Container::new(content).style(style)
}

pub fn elevated<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    styled(content, container_style::elevated_card)
}

pub fn filled<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    styled(content, container_style::filled_card)
}

pub fn outlined<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    styled(content, container_style::outlined_card)
}
