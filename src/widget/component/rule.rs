//! Material 3 divider constructors with token-backed thickness and insets.

use super::*;

pub fn horizontal_full_width<'a>() -> Rule<'a, Theme> {
    iced_rule::horizontal(tokens::component::divider::THICKNESS).style(rule_style::full_width)
}

pub fn horizontal_inset<'a>() -> Rule<'a, Theme> {
    iced_rule::horizontal(tokens::component::divider::THICKNESS).style(rule_style::inset)
}

pub fn vertical_full_height<'a>() -> Rule<'a, Theme> {
    iced_rule::vertical(tokens::component::divider::THICKNESS).style(rule_style::full_width)
}

pub fn vertical_inset<'a>() -> Rule<'a, Theme> {
    iced_rule::vertical(tokens::component::divider::THICKNESS).style(rule_style::inset)
}
