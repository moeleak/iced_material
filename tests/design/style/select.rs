use super::*;

#[test]
fn default_select_uses_m3_outlined_select_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let active = default(&theme, Status::Active);
    assert_eq!(active.background, Background::Color(Color::TRANSPARENT));
    assert_eq!(active.border.color, colors.outline.color);
    assert_eq!(
        active.border.width,
        tokens::component::select::TEXT_FIELD_OUTLINE_WIDTH
    );

    let hovered = default(&theme, Status::Hovered);
    assert_eq!(hovered.border.color, colors.surface.text);
    assert_eq!(
        hovered.border.width,
        tokens::component::select::TEXT_FIELD_HOVER_OUTLINE_WIDTH
    );

    let opened = default(&theme, Status::Opened { is_hovered: false });
    assert_eq!(opened.border.color, colors.primary.color);
    assert_eq!(
        opened.border.width,
        tokens::component::select::TEXT_FIELD_FOCUS_OUTLINE_WIDTH
    );
    assert_eq!(opened.handle_color, colors.primary.color);
}

#[test]
fn default_select_menu_uses_m3_outlined_select_menu_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let menu_class = <Theme as Catalog>::default_menu();
    let menu = <Theme as overlay_menu::Catalog>::style(&theme, &menu_class);

    assert_eq!(
        menu.selected_background,
        Background::Color(colors.surface.container.highest)
    );
    assert_eq!(menu.selected_text_color, colors.surface.text);
}
