use super::*;

#[test]
fn toolbar_styles_use_m3_color_roles() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let standard = floating_style(&theme, ColorMode::Standard);
    assert_eq!(
        standard.background,
        Some(Background::Color(colors.surface.container.base))
    );
    assert_eq!(standard.shadow.offset.y, 4.0);

    let vibrant = docked_style(&theme, ColorMode::Vibrant);
    assert_eq!(
        vibrant.background,
        Some(Background::Color(colors.primary.container))
    );
}

#[test]
fn toolbar_action_styles_follow_standard_and_vibrant_selectors() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let standard = action_style(&theme, Status::Active, ColorMode::Standard, false);
    assert_eq!(
        standard.background,
        Some(Background::Color(colors.surface.container.base))
    );
    assert_eq!(standard.text_color, colors.surface.text_variant);

    let selected_standard = action_style(&theme, Status::Active, ColorMode::Standard, true);
    assert_eq!(
        selected_standard.background,
        Some(Background::Color(colors.secondary.container))
    );
    assert_eq!(
        selected_standard.text_color,
        colors.secondary.container_text
    );

    let selected_vibrant = action_style(&theme, Status::Active, ColorMode::Vibrant, true);
    assert_eq!(
        selected_vibrant.background,
        Some(Background::Color(colors.surface.container.base))
    );
    assert_eq!(selected_vibrant.text_color, colors.surface.text);
}
