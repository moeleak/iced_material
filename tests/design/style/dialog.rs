use super::*;

#[test]
fn default_dialog_container_uses_m3_container_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = default_container(&theme);

    assert_eq!(
        style.background,
        Some(Background::Color(colors.surface.container.high))
    );
    assert_eq!(
        style.border.radius.top_left,
        tokens::component::dialog::CONTAINER_SHAPE
    );
    assert_eq!(style.shadow.offset.y, 4.0);
    assert_eq!(style.shadow.blur_radius, 8.0);
}
