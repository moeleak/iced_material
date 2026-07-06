use super::*;

#[test]
fn default_text_input_uses_m3_outlined_field_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let active = default(&theme, Status::Active);
    assert_eq!(active.background, Background::Color(Color::TRANSPARENT));
    assert_eq!(active.border.color, colors.outline.color);
    assert_eq!(
        active.border.width,
        tokens::component::text_field::OUTLINE_WIDTH
    );
    assert_eq!(
        active.border.radius.top_left,
        tokens::component::text_field::CONTAINER_SHAPE
    );

    let focused = default(&theme, Status::Focused { is_hovered: false });
    assert_eq!(focused.border.color, colors.primary.color);
    assert_eq!(
        focused.border.width,
        tokens::component::text_field::FOCUS_OUTLINE_WIDTH
    );
    assert_eq!(focused.placeholder, colors.surface.text_variant);

    let disabled = default(&theme, Status::Disabled);
    assert_eq!(
        disabled.border.color.a,
        tokens::component::text_field::DISABLED_OUTLINE_OPACITY
    );
    assert_eq!(
        disabled.icon.a,
        tokens::component::text_field::DISABLED_LEADING_ICON_OPACITY
    );
    assert_eq!(
        disabled.placeholder.a,
        tokens::component::text_field::DISABLED_INPUT_TEXT_OPACITY
    );
    assert_eq!(
        disabled.value.a,
        tokens::component::text_field::DISABLED_INPUT_TEXT_OPACITY
    );
}
