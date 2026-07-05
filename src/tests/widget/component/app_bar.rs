use super::*;

#[test]
fn top_app_bar_styles_use_surface_roles() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let resting = top_style(&theme, false);
    assert_eq!(
        resting.background,
        Some(Background::Color(colors.surface.color))
    );
    assert_eq!(resting.shadow.offset.y, 0.0);

    let scrolled = top_style(&theme, true);
    assert_eq!(
        scrolled.background,
        Some(Background::Color(colors.surface.container.base))
    );
    assert_eq!(scrolled.shadow.offset.y, 2.0);
    assert_eq!(scrolled.shadow.blur_radius, 6.0);
}

#[test]
fn bottom_app_bar_uses_surface_container_and_level2() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = bottom_style(&theme);

    assert_eq!(
        style.background,
        Some(Background::Color(colors.surface.container.base))
    );
    assert_eq!(style.shadow.offset.y, 2.0);
    assert_eq!(style.shadow.blur_radius, 6.0);
}

#[test]
fn status_bar_uses_fixed_edge_to_edge_inset() {
    let status: Container<'_, (), Theme, iced_widget::Renderer> = status_bar();

    assert_eq!(STATUS_BAR_HEIGHT, 24.0);
    assert_eq!(
        iced_widget::core::Widget::<(), Theme, iced_widget::Renderer>::size(&status).height,
        Length::Fixed(STATUS_BAR_HEIGHT)
    );
}
