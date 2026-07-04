//! Material 3 container and card constructors.

use super::*;

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

pub fn transparent<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    styled(content, container_style::transparent)
}

pub fn surface<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    styled(content, container_style::surface)
}

pub fn surface_container_lowest<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    styled(content, container_style::surface_container_lowest)
}

pub fn surface_container_low<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    styled(content, container_style::surface_container_low)
}

pub fn surface_container<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    styled(content, container_style::surface_container)
}

pub fn surface_container_high<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    styled(content, container_style::surface_container_high)
}

pub fn surface_container_highest<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    styled(content, container_style::surface_container_highest)
}

pub fn outlined<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    styled(content, container_style::outlined)
}

pub fn elevated_card<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    super::card::elevated(content)
}

pub fn filled_card<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    super::card::filled(content)
}

pub fn outlined_card<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Container<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: iced_widget::core::Renderer + 'a,
{
    super::card::outlined(content)
}
