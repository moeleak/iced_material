use super::*;

#[test]
fn search_bar_uses_surface_container_high_and_level3() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = bar_container_style(&theme);

    assert_eq!(
        style.background,
        Some(Background::Color(colors.surface.container.high))
    );
    assert_eq!(
        style.border.radius.top_left,
        tokens::component::search_bar::CONTAINER_SHAPE
    );
    assert_eq!(style.shadow.offset.y, 4.0);
    assert_eq!(style.shadow.blur_radius, 8.0);
}

#[test]
fn search_input_style_uses_body_surface_roles() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = input_style(&theme, Status::Active);

    assert_eq!(style.value, colors.surface.text);
    assert_eq!(style.placeholder, colors.surface.text_variant);
    assert_eq!(style.background, Background::Color(Color::TRANSPARENT));
}
