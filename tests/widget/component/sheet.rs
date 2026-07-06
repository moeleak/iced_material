use iced_widget::core::time::Duration;

use super::*;

#[test]
fn bottom_sheet_radius_rounds_top_corners_only() {
    let radius = bottom_sheet_radius();

    assert_eq!(
        radius.top_left,
        tokens::component::bottom_sheet::CONTAINER_SHAPE_TOP
    );
    assert_eq!(
        radius.top_right,
        tokens::component::bottom_sheet::CONTAINER_SHAPE_TOP
    );
    assert_eq!(radius.bottom_left, 0.0);
    assert_eq!(radius.bottom_right, 0.0);
}

#[test]
fn bottom_sheet_style_uses_surface_container_low_and_level1() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = bottom_sheet_style(&theme, Kind::Modal);

    assert_eq!(
        style.background,
        Some(Background::Color(colors.surface.container.low))
    );
    assert_eq!(style.text_color, Some(colors.surface.text));
    assert_eq!(
        style.shadow.offset.y,
        tokens::elevation::shadow(1).ambient.y
    );
    assert_eq!(
        style.shadow.blur_radius,
        tokens::elevation::shadow(1).ambient.blur
    );
}

#[test]
fn side_sheet_radius_matches_docked_and_detached_edges() {
    let right = side_sheet_radius(Kind::Modal, Side::Right, false);
    assert_eq!(
        right.top_left,
        tokens::component::side_sheet::DOCKED_MODAL_CONTAINER_SHAPE
    );
    assert_eq!(right.top_right, 0.0);
    assert_eq!(right.bottom_right, 0.0);
    assert_eq!(
        right.bottom_left,
        tokens::component::side_sheet::DOCKED_MODAL_CONTAINER_SHAPE
    );

    let left = side_sheet_radius(Kind::Modal, Side::Left, false);
    assert_eq!(left.top_left, 0.0);
    assert_eq!(
        left.top_right,
        tokens::component::side_sheet::DOCKED_MODAL_CONTAINER_SHAPE
    );
    assert_eq!(
        left.bottom_right,
        tokens::component::side_sheet::DOCKED_MODAL_CONTAINER_SHAPE
    );
    assert_eq!(left.bottom_left, 0.0);

    let standard = side_sheet_radius(Kind::Standard, Side::Right, false);
    assert_eq!(standard.top_left, 0.0);
    assert_eq!(standard.top_right, 0.0);
    assert_eq!(standard.bottom_left, 0.0);
    assert_eq!(standard.bottom_right, 0.0);

    let detached = side_sheet_radius(Kind::Standard, Side::Right, true);
    assert_eq!(
        detached.top_left,
        tokens::component::side_sheet::DETACHED_CONTAINER_SHAPE
    );
    assert_eq!(
        detached.top_right,
        tokens::component::side_sheet::DETACHED_CONTAINER_SHAPE
    );
    assert_eq!(
        detached.bottom_left,
        tokens::component::side_sheet::DETACHED_CONTAINER_SHAPE
    );
    assert_eq!(
        detached.bottom_right,
        tokens::component::side_sheet::DETACHED_CONTAINER_SHAPE
    );
}

#[test]
fn side_sheet_styles_use_material_container_roles_and_elevation() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let modal = side_sheet_style(&theme, Kind::Modal, Side::Right, false);
    assert_eq!(
        modal.background,
        Some(Background::Color(colors.surface.container.low))
    );
    assert_eq!(modal.text_color, Some(colors.surface.text));
    assert_eq!(
        modal.shadow.offset.y,
        tokens::elevation::shadow(1).ambient.y
    );

    let standard = side_sheet_style(&theme, Kind::Standard, Side::Right, false);
    assert_eq!(
        standard.background,
        Some(Background::Color(colors.surface.color))
    );
    assert_eq!(
        standard.shadow.offset.y,
        tokens::elevation::shadow(0).ambient.y
    );
    assert_eq!(
        standard.shadow.blur_radius,
        tokens::elevation::shadow(0).ambient.blur
    );
}

#[test]
fn drag_handle_uses_on_surface_variant_color() {
    let theme = Theme::Light;
    let style = drag_handle_style(&theme);

    assert_eq!(
        style.background,
        Some(Background::Color(theme.colors().surface.text_variant))
    );
    assert_eq!(style.border.radius.top_left, tokens::shape::CORNER_FULL);
}

#[test]
fn sheet_content_wrappers_apply_material_fill_constraints() {
    let bottom: Container<'_, (), Theme, iced_widget::Renderer> = bottom_content(Space::new());
    let bottom_size = iced_widget::core::Widget::<(), Theme, iced_widget::Renderer>::size(&bottom);
    assert_eq!(bottom_size.width, Length::Fill);

    let side: Container<'_, (), Theme, iced_widget::Renderer> = side_content(Space::new());
    let side_size = iced_widget::core::Widget::<(), Theme, iced_widget::Renderer>::size(&side);
    assert_eq!(side_size.width, Length::Fill);
    assert_eq!(side_size.height, Length::Fill);
}

#[test]
fn scrim_uses_material_opacity_over_scrim_color() {
    let theme = Theme::Light;
    let style = scrim_style(&theme);
    let Some(Background::Color(color)) = style.background else {
        panic!("expected solid scrim background");
    };

    assert_eq!(color.a, tokens::component::bottom_sheet::SCRIM_OPACITY);
}

#[test]
fn side_scrim_uses_material_opacity_over_scrim_color() {
    let theme = Theme::Light;
    let style = side_scrim_style(&theme);
    let Some(Background::Color(color)) = style.background else {
        panic!("expected solid scrim background");
    };

    assert_eq!(color.a, tokens::component::side_sheet::SCRIM_OPACITY);
}

#[test]
fn sheet_state_animates_with_material_timing() {
    let start = Instant::now();
    let mut state = State::new(Value::Hidden);

    state.show(start);
    assert_eq!(state.target(), Value::PartiallyExpanded);
    assert_eq!(state.visibility_progress(), 0.0);
    assert!(state.is_animating());

    let _ = state.advance(start + Duration::from_millis(150));
    assert!(state.visibility_progress() > 0.0);
    assert!(state.visibility_progress() < 0.5);

    let _ = state.advance(
        start
            + Duration::from_millis(u64::from(
                tokens::component::bottom_sheet::ANIMATION_DURATION_MS,
            )),
    );
    assert_eq!(state.visibility_progress(), 0.5);
    assert!(!state.is_animating());
}

#[test]
fn side_sheet_state_animates_with_material_timing() {
    let start = Instant::now();
    let mut state = SideState::new(SideValue::Hidden);

    state.show(start);
    assert_eq!(state.target(), SideValue::Expanded);
    assert_eq!(state.visibility_progress(), 0.0);
    assert_eq!(state.translation_fraction(Side::Right), 1.0);
    assert_eq!(state.translation_fraction(Side::Left), -1.0);
    assert!(state.is_animating());

    let _ = state.advance(start + Duration::from_millis(138));
    assert!(state.visibility_progress() > 0.0);
    assert!(state.visibility_progress() < 1.0);

    let _ = state.advance(
        start
            + Duration::from_millis(u64::from(
                tokens::component::side_sheet::ANIMATION_DURATION_MS,
            )),
    );
    assert_eq!(state.visibility_progress(), 1.0);
    assert_eq!(state.translation_fraction(Side::Right), 0.0);
    assert!(!state.is_animating());

    state.hide(start);
    assert_eq!(state.target(), SideValue::Hidden);
}
