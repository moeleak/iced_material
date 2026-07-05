use super::*;

#[test]
fn primary_tab_active_style_uses_primary_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = tab_style(&theme, Status::Active, Variant::Primary, true);

    assert_eq!(
        style.background,
        Some(Background::Color(colors.surface.color))
    );
    assert_eq!(style.text_color, colors.primary.color);
    assert_eq!(style.shadow.offset.y, 0.0);
}

#[test]
fn inactive_tabs_use_on_surface_variant_until_interaction() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let active = tab_style(&theme, Status::Active, Variant::Secondary, false);
    let hovered = tab_style(&theme, Status::Hovered, Variant::Secondary, false);

    assert_eq!(active.text_color, colors.surface.text_variant);
    assert_eq!(hovered.text_color, colors.surface.text);
    assert_eq!(
        hovered.background,
        Some(Background::Color(mix(
            colors.surface.color,
            colors.surface.text,
            tokens::component::secondary_tab::HOVER_STATE_LAYER_OPACITY
        )))
    );
}

#[test]
fn indicator_uses_primary_shape_and_secondary_square_shape() {
    let primary = indicator_radius(Variant::Primary);
    assert_eq!(
        primary.top_left,
        tokens::component::primary_tab::ACTIVE_INDICATOR_SHAPE_TOP
    );
    assert_eq!(primary.bottom_left, 0.0);
    assert_eq!(
        indicator_radius(Variant::Secondary),
        Radius::new(tokens::component::secondary_tab::ACTIVE_INDICATOR_SHAPE)
    );
}

#[test]
fn inactive_indicator_is_transparent_but_keeps_height() {
    let theme = Theme::Light;
    let style = indicator_style(&theme, Variant::Primary, false);

    assert_eq!(
        style.background,
        Some(Background::Color(Color::TRANSPARENT))
    );
    assert_eq!(
        Variant::Primary.indicator_height(),
        tokens::component::primary_tab::ACTIVE_INDICATOR_HEIGHT
    );
}

#[test]
fn tab_state_animates_indicator_with_material_timing() {
    let now = Instant::now();
    let mut state = State::new(0);

    state.select(2, now, Variant::Primary);

    assert_eq!(state.selected_index(), 2);
    assert_eq!(state.indicator_position.to, 2.0);

    let _ = state.advance(now + duration_ms(125));
    assert!(state.indicator_position() > 0.0);
    assert!(state.indicator_position() < 2.0);

    let _ = state.advance(now + duration_ms(250));
    assert_eq!(state.indicator_position(), 2.0);
}

#[test]
fn moving_indicator_width_matches_primary_inset_and_secondary_full_width() {
    assert_eq!(
        moving_indicator_width(Variant::Primary, 120.0),
        120.0 - tokens::component::primary_tab::HORIZONTAL_SPACE * 2.0
    );
    assert_eq!(moving_indicator_width(Variant::Secondary, 120.0), 120.0);
}
