//! Bundled Material typefaces and icon fonts.

use iced_widget::Text;
use iced_widget::core::Font;
use iced_widget::core::font::{Family, Stretch, Style, Weight};
use iced_widget::core::text as core_text;
use iced_widget::text::{self, LineHeight};

use crate::{Theme, tokens};

use std::borrow::Cow;

pub const ROBOTO_FAMILY: &str = "Roboto";
pub const MATERIAL_SYMBOLS_ROUNDED_FAMILY: &str = "Material Symbols Rounded";

pub const ROBOTO_REGULAR_BYTES: &[u8] = include_bytes!("fonts/Roboto-Regular.ttf");
pub const ROBOTO_MEDIUM_BYTES: &[u8] = include_bytes!("fonts/Roboto-Medium.ttf");
pub const ROBOTO_BOLD_BYTES: &[u8] = include_bytes!("fonts/Roboto-Bold.ttf");
pub const MATERIAL_SYMBOLS_ROUNDED_BYTES: &[u8] =
    include_bytes!("fonts/MaterialSymbolsRounded-Regular.ttf");

pub const ROBOTO: Font = roboto_for_weight(tokens::typography::WEIGHT_REGULAR);
pub const ROBOTO_MEDIUM: Font = roboto_for_weight(tokens::typography::WEIGHT_MEDIUM);
pub const ROBOTO_BOLD: Font = roboto_for_weight(tokens::typography::WEIGHT_BOLD);
pub const MATERIAL_SYMBOLS_ROUNDED: Font = Font {
    family: Family::Name(MATERIAL_SYMBOLS_ROUNDED_FAMILY),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

pub fn all() -> [Cow<'static, [u8]>; 4] {
    [
        Cow::Borrowed(ROBOTO_REGULAR_BYTES),
        Cow::Borrowed(ROBOTO_MEDIUM_BYTES),
        Cow::Borrowed(ROBOTO_BOLD_BYTES),
        Cow::Borrowed(MATERIAL_SYMBOLS_ROUNDED_BYTES),
    ]
}

pub const fn roboto_for_type_scale(scale: tokens::typography::TypeScale) -> Font {
    roboto_for_weight(scale.weight)
}

pub const fn roboto_for_weight(weight: u16) -> Font {
    Font {
        family: Family::Name(ROBOTO_FAMILY),
        weight: match weight {
            tokens::typography::WEIGHT_BOLD => Weight::Bold,
            tokens::typography::WEIGHT_MEDIUM => Weight::Medium,
            _ => Weight::Normal,
        },
        stretch: Stretch::Normal,
        style: Style::Normal,
    }
}

pub fn icon<'a, Renderer>(name: impl text::IntoFragment<'a>, size: f32) -> Text<'a, Theme, Renderer>
where
    Renderer: core_text::Renderer,
    Font: Into<Renderer::Font>,
{
    Text::new(name)
        .font(MATERIAL_SYMBOLS_ROUNDED)
        .size(size)
        .line_height(LineHeight::Absolute(size.into()))
        .shaping(text::Shaping::Advanced)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_truetype(bytes: &[u8]) -> bool {
        bytes.starts_with(&[0x00, 0x01, 0x00, 0x00])
    }

    #[test]
    fn bundled_fonts_are_truetype_assets() {
        assert!(is_truetype(ROBOTO_REGULAR_BYTES));
        assert!(is_truetype(ROBOTO_MEDIUM_BYTES));
        assert!(is_truetype(ROBOTO_BOLD_BYTES));
        assert!(is_truetype(MATERIAL_SYMBOLS_ROUNDED_BYTES));
        assert_eq!(all().len(), 4);
    }

    #[test]
    fn material_fonts_expose_expected_families_and_weights() {
        assert_eq!(ROBOTO.family, Family::Name(ROBOTO_FAMILY));
        assert_eq!(ROBOTO.weight, Weight::Normal);
        assert_eq!(ROBOTO_MEDIUM.weight, Weight::Medium);
        assert_eq!(ROBOTO_BOLD.weight, Weight::Bold);
        assert_eq!(
            MATERIAL_SYMBOLS_ROUNDED.family,
            Family::Name(MATERIAL_SYMBOLS_ROUNDED_FAMILY)
        );
    }

    #[test]
    fn type_scale_weights_select_roboto_faces() {
        assert_eq!(
            roboto_for_type_scale(tokens::typography::BODY_LARGE),
            ROBOTO
        );
        assert_eq!(
            roboto_for_type_scale(tokens::typography::LABEL_LARGE),
            ROBOTO_MEDIUM
        );
        assert_eq!(
            roboto_for_weight(tokens::typography::WEIGHT_BOLD),
            ROBOTO_BOLD
        );
    }
}
