use super::*;

#[test]
fn plain_tooltip_uses_m3_plain_tooltip_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = plain(&theme);

    assert_eq!(
        style.background,
        Some(Background::Color(colors.inverse.inverse_surface))
    );
    assert_eq!(style.text_color, Some(colors.inverse.inverse_surface_text));
    assert_eq!(
        style.border.radius.top_left,
        tokens::component::tooltip::PLAIN_CONTAINER_SHAPE
    );
    assert_eq!(style.shadow, Shadow::default());
}

#[test]
fn rich_tooltip_uses_m3_rich_tooltip_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = rich(&theme);

    assert_eq!(
        style.background,
        Some(Background::Color(colors.surface.container.base))
    );
    assert_eq!(style.text_color, Some(colors.surface.text_variant));
    assert_eq!(
        style.border.radius.top_left,
        tokens::component::tooltip::RICH_CONTAINER_SHAPE
    );
    assert_eq!(style.shadow.offset.y, 2.0);
    assert_eq!(style.shadow.blur_radius, 6.0);
}
