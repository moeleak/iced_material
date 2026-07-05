use iced_widget::container::Style;
use iced_widget::core::{Background, border};

use crate::{Theme, tokens};

pub fn default(theme: &Theme) -> Style {
    let error = theme.colors().error;

    Style {
        background: Some(Background::Color(error.color)),
        text_color: Some(error.text),
        border: border::rounded(tokens::component::badge::LARGE_CONTAINER_SHAPE),
        ..Style::default()
    }
}

#[cfg(test)]
#[path = "../../tests/design/style/badge.rs"]
mod tests;
