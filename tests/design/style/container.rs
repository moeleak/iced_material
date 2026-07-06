use super::*;

#[test]
fn container_helpers_use_m3_shape_tokens() {
    assert_eq!(
        transparent(&Theme::Light).border.radius.top_left,
        tokens::shape::CORNER_EXTRA_SMALL
    );
    assert_eq!(
        surface_container(&Theme::Light).border.radius.top_left,
        tokens::shape::CORNER_SMALL
    );
}

#[test]
fn outlined_container_uses_m3_one_pixel_outline() {
    let style = outlined(&Theme::Light);

    assert_eq!(style.border.color, Theme::Light.colors().outline.color);
    assert_eq!(
        style.border.width,
        tokens::component::button::OUTLINED_OUTLINE_WIDTH
    );
}

#[test]
fn card_helpers_use_m3_container_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let elevated = elevated_card(&theme);
    assert_eq!(
        elevated.background,
        Some(Background::Color(colors.surface.container.low))
    );
    assert_eq!(
        elevated.border.radius.top_left,
        tokens::component::card::CONTAINER_SHAPE
    );
    assert_eq!(elevated.shadow.offset.y, 1.0);
    assert_eq!(elevated.shadow.blur_radius, 3.0);

    let filled = filled_card(&theme);
    assert_eq!(
        filled.background,
        Some(Background::Color(colors.surface.container.highest))
    );
    assert_eq!(filled.shadow.offset.y, 0.0);

    let outlined = outlined_card(&theme);
    assert_eq!(
        outlined.background,
        Some(Background::Color(colors.surface.color))
    );
    assert_eq!(outlined.border.color, colors.outline.variant);
    assert_eq!(
        outlined.border.width,
        tokens::component::card::OUTLINED_OUTLINE_WIDTH
    );
}
