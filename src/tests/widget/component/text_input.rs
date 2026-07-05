use super::*;
use crate::utils::disabled_text;

#[test]
fn outlined_text_input_alpha_scales_input_layer_colors() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = input_layer_style_alpha(&theme, iced_text_input::Status::Active, 0.5);

    assert_eq!(style.value, alpha_color(colors.surface.text, 0.5));
    assert_eq!(style.icon, alpha_color(colors.surface.text_variant, 0.5));
    assert_eq!(
        style.selection,
        alpha_color(disabled_text(colors.primary.color), 0.5)
    );
    assert_eq!(style.placeholder, Color::TRANSPARENT);
}
