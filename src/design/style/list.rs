use iced_widget::container::Style;
use iced_widget::core::{Background, border};

use crate::{Theme, tokens};

pub fn item(theme: &Theme) -> Style {
    let surface = theme.colors().surface;

    Style {
        background: Some(Background::Color(surface.color)),
        text_color: Some(surface.text),
        border: border::rounded(tokens::component::list::CONTAINER_SHAPE),
        ..Style::default()
    }
}

pub fn disabled_item(theme: &Theme) -> Style {
    let surface = theme.colors().surface;

    Style {
        background: Some(Background::Color(surface.color)),
        text_color: Some(iced_widget::core::Color {
            a: tokens::component::list::DISABLED_LABEL_TEXT_OPACITY,
            ..surface.text
        }),
        border: border::rounded(tokens::component::list::CONTAINER_SHAPE),
        ..Style::default()
    }
}

#[cfg(test)]
#[path = "../../../tests/design/style/list.rs"]
mod tests;
