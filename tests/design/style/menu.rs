use super::*;

#[test]
fn default_menu_uses_m3_container_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = default(&theme);

    assert_eq!(
        style.background,
        Background::Color(colors.surface.container.base)
    );
    assert_eq!(
        style.border.radius.top_left,
        tokens::component::menu::CONTAINER_SHAPE
    );
    assert_eq!(
        style.selected_background,
        Background::Color(colors.secondary.container)
    );
    assert_eq!(style.selected_text_color, colors.secondary.container_text);
    assert_eq!(style.shadow.offset.y, 2.0);
    assert_eq!(style.shadow.blur_radius, 6.0);
}

#[test]
fn outlined_select_menu_uses_m3_outlined_select_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = outlined_select(&theme);

    assert_eq!(
        style.background,
        Background::Color(colors.surface.container.base)
    );
    assert_eq!(
        style.border.radius.top_left,
        tokens::component::select::MENU_CONTAINER_SHAPE
    );
    assert_eq!(
        style.selected_background,
        Background::Color(colors.surface.container.highest)
    );
    assert_eq!(style.selected_text_color, colors.surface.text);
    assert_eq!(style.shadow.offset.y, 2.0);
    assert_eq!(style.shadow.blur_radius, 6.0);
}
