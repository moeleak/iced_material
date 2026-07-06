use super::*;

#[test]
fn filled_button_uses_m3_elevation_tokens() {
    let theme = Theme::Light;

    let active = filled(&theme, Status::Active);
    assert_eq!(active.shadow.offset.y, 0.0);
    assert_eq!(active.shadow.blur_radius, 0.0);

    let hovered = filled(&theme, Status::Hovered);
    assert_eq!(hovered.shadow.offset.y, 1.0);
    assert_eq!(hovered.shadow.blur_radius, 3.0);

    let pressed = filled(&theme, Status::Pressed);
    assert_eq!(pressed.shadow.offset.y, 0.0);
    assert_eq!(pressed.shadow.blur_radius, 0.0);
    assert_eq!(pressed, active);
}

#[test]
fn elevated_button_uses_m3_elevation_tokens() {
    let theme = Theme::Light;

    let active = elevated(&theme, Status::Active);
    assert_eq!(active.shadow.offset.y, 1.0);
    assert_eq!(active.shadow.blur_radius, 3.0);

    let hovered = elevated(&theme, Status::Hovered);
    assert_eq!(hovered.shadow.offset.y, 2.0);
    assert_eq!(hovered.shadow.blur_radius, 6.0);

    let pressed = elevated(&theme, Status::Pressed);
    assert_eq!(pressed.shadow.offset.y, 1.0);
    assert_eq!(pressed.shadow.blur_radius, 3.0);
    assert_eq!(pressed, active);
}

#[test]
fn outlined_disabled_button_has_no_container_fill() {
    let theme = Theme::Light;
    let style = outlined(&theme, Status::Disabled);

    assert_eq!(style.background, None);
    assert_eq!(
        style.border.color.a,
        tokens::state::DISABLED_CONTAINER_OPACITY
    );
    assert_eq!(
        style.text_color.a,
        tokens::state::DISABLED_LABEL_TEXT_OPACITY
    );
}

#[test]
fn fab_primary_uses_m3_container_shape_and_elevation_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let active = fab_primary(&theme, Status::Active);
    assert_eq!(
        active.background,
        Some(Background::Color(colors.primary.color))
    );
    assert_eq!(active.text_color, colors.primary.text);
    assert_eq!(
        active.border.radius.top_left,
        tokens::component::fab::CONTAINER_SHAPE
    );
    assert_eq!(active.shadow.offset.y, 4.0);
    assert_eq!(active.shadow.blur_radius, 8.0);

    let hovered = fab_primary(&theme, Status::Hovered);
    assert_eq!(hovered.shadow.offset.y, 6.0);
    assert_eq!(hovered.shadow.blur_radius, 10.0);

    let pressed = fab_primary(&theme, Status::Pressed);
    assert_eq!(pressed.shadow.offset.y, 4.0);
    assert_eq!(pressed.shadow.blur_radius, 8.0);
    assert_eq!(pressed, active);
}

#[test]
fn fab_size_variants_use_m3_shape_tokens() {
    let theme = Theme::Light;

    let small = fab_primary_small(&theme, Status::Active);
    assert_eq!(
        small.border.radius.top_left,
        tokens::component::fab::SMALL_CONTAINER_SHAPE
    );

    let large = fab_primary_large(&theme, Status::Active);
    assert_eq!(
        large.border.radius.top_left,
        tokens::component::fab::LARGE_CONTAINER_SHAPE
    );
}

#[test]
fn fab_tertiary_uses_m3_filled_color_roles() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = fab_tertiary(&theme, Status::Active);

    assert_eq!(
        style.background,
        Some(Background::Color(colors.tertiary.color))
    );
    assert_eq!(style.text_color, colors.tertiary.text);
}

#[test]
fn fab_surface_uses_m3_surface_container_high_and_primary_icon() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = fab_surface(&theme, Status::Active);

    assert_eq!(
        style.background,
        Some(Background::Color(colors.surface.container.high))
    );
    assert_eq!(style.text_color, colors.primary.color);
}

#[test]
fn extended_fab_uses_m3_shape_color_and_elevation_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let active = extended_fab_primary(&theme, Status::Active);
    assert_eq!(
        active.background,
        Some(Background::Color(colors.primary.color))
    );
    assert_eq!(active.text_color, colors.primary.text);
    assert_eq!(
        active.border.radius.top_left,
        tokens::component::fab::EXTENDED_CONTAINER_SHAPE
    );
    assert_eq!(active.shadow.offset.y, 4.0);
    assert_eq!(active.shadow.blur_radius, 8.0);

    let hovered = extended_fab_primary(&theme, Status::Hovered);
    assert_eq!(hovered.shadow.offset.y, 6.0);
    assert_eq!(hovered.shadow.blur_radius, 10.0);
}

#[test]
fn standard_icon_button_uses_m3_state_layer_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let active = icon(&theme, Status::Active);
    assert_eq!(active.background, None);
    assert_eq!(active.text_color, colors.surface.text_variant);
    assert_eq!(
        active.border.radius.top_left,
        tokens::component::icon_button::CONTAINER_SHAPE
    );

    let hovered = icon(&theme, Status::Hovered);
    assert_eq!(
        hovered.background,
        Some(Background::Color(mix(
            Color::TRANSPARENT,
            colors.surface.text_variant,
            HOVERED_LAYER_OPACITY
        )))
    );

    let pressed = icon(&theme, Status::Pressed);
    assert_eq!(pressed, active);

    let disabled = icon(&theme, Status::Disabled);
    assert_eq!(
        disabled.text_color.a,
        tokens::component::icon_button::DISABLED_ICON_OPACITY
    );
    assert_eq!(disabled.background, None);
}

#[test]
fn filled_icon_buttons_use_m3_container_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let filled = filled_icon(&theme, Status::Active);
    assert_eq!(
        filled.background,
        Some(Background::Color(colors.primary.color))
    );
    assert_eq!(filled.text_color, colors.primary.text);
    assert_eq!(
        filled.border.radius.top_left,
        tokens::component::icon_button::CONTAINER_SHAPE
    );

    let tonal = filled_tonal_icon(&theme, Status::Active);
    assert_eq!(
        tonal.background,
        Some(Background::Color(colors.secondary.container))
    );
    assert_eq!(tonal.text_color, colors.secondary.container_text);

    let tonal_pressed = filled_tonal_icon(&theme, Status::Pressed);
    assert_eq!(tonal_pressed, tonal);
}

#[test]
fn chip_helpers_use_m3_chip_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let assist = assist_chip(&theme, Status::Active);
    assert_eq!(assist.background, None);
    assert_eq!(assist.text_color, colors.surface.text);
    assert_eq!(assist.border.color, colors.outline.color);
    assert_eq!(assist.border.width, tokens::component::chip::OUTLINE_WIDTH);
    assert_eq!(
        assist.border.radius.top_left,
        tokens::component::chip::CONTAINER_SHAPE
    );
    assert_eq!(assist_chip(&theme, Status::Pressed), assist);

    let elevated = elevated_assist_chip(&theme, Status::Active);
    assert_eq!(
        elevated.background,
        Some(Background::Color(colors.surface.container.low))
    );
    assert_eq!(elevated.shadow.offset.y, 1.0);
    assert_eq!(elevated.shadow.blur_radius, 3.0);

    let hovered_elevated = elevated_assist_chip(&theme, Status::Hovered);
    assert_eq!(hovered_elevated.shadow.offset.y, 2.0);
    assert_eq!(hovered_elevated.shadow.blur_radius, 6.0);

    let suggestion = suggestion_chip(&theme, Status::Active);
    assert_eq!(suggestion.text_color, colors.surface.text_variant);

    let disabled = assist_chip(&theme, Status::Disabled);
    assert_eq!(
        disabled.text_color.a,
        tokens::component::chip::DISABLED_LABEL_TEXT_OPACITY
    );
    assert_eq!(
        disabled.border.color.a,
        tokens::component::chip::DISABLED_OUTLINE_OPACITY
    );
}

#[test]
fn selectable_chip_helpers_use_m3_selected_and_unselected_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let unselected = filter_chip(&theme, Status::Active);
    assert_eq!(unselected.background, None);
    assert_eq!(unselected.text_color, colors.surface.text_variant);
    assert_eq!(unselected.border.color, colors.outline.color);

    let selected = selected_filter_chip(&theme, Status::Active);
    assert_eq!(
        selected.background,
        Some(Background::Color(colors.secondary.container))
    );
    assert_eq!(selected.text_color, colors.secondary.container_text);
    assert_eq!(
        selected.border.width,
        tokens::component::chip::SELECTED_OUTLINE_WIDTH
    );
    assert_eq!(selected_filter_chip(&theme, Status::Pressed), selected);

    let hovered = selected_filter_chip(&theme, Status::Hovered);
    assert_eq!(hovered.shadow.offset.y, 1.0);
    assert_eq!(hovered.shadow.blur_radius, 3.0);

    let input = input_chip(&theme, Status::Active);
    assert_eq!(input.text_color, colors.surface.text_variant);

    let selected_input = selected_input_chip(&theme, Status::Active);
    assert_eq!(
        selected_input.background,
        Some(Background::Color(colors.secondary.container))
    );
    assert_eq!(selected_input.shadow.blur_radius, 0.0);
}

#[test]
fn outlined_icon_button_uses_m3_outline_tokens_without_pressed_overlay() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let active = outlined_icon(&theme, Status::Active);
    assert_eq!(active.border.color, colors.outline.color);
    assert_eq!(
        active.border.width,
        tokens::component::icon_button::OUTLINED_OUTLINE_WIDTH
    );
    assert_eq!(active.text_color, colors.surface.text_variant);

    let pressed = outlined_icon(&theme, Status::Pressed);
    assert_eq!(pressed, active);

    let disabled = outlined_icon(&theme, Status::Disabled);
    assert_eq!(
        disabled.border.color.a,
        tokens::component::icon_button::OUTLINED_DISABLED_OUTLINE_OPACITY
    );
    assert_eq!(
        disabled.text_color.a,
        tokens::component::icon_button::DISABLED_ICON_OPACITY
    );
}
