use super::*;

#[test]
fn state_toggles_open_closed() {
    let mut state = State::new();

    assert!(!state.is_open());

    state.toggle();
    assert!(state.is_open());

    state.close();
    assert!(!state.is_open());
}

#[test]
fn bottom_margin_accounts_for_adaptive_navigation_clearance() {
    assert_eq!(
        bottom_margin_for_navigation_layout(navigation::AdaptiveLayout::NavigationBar),
        FLOATING_MARGIN + tokens::component::navigation_bar::CONTAINER_HEIGHT
    );
    assert_eq!(
        bottom_margin_for_navigation_layout(navigation::AdaptiveLayout::NavigationRail),
        FLOATING_MARGIN
    );
}

#[test]
fn controller_selects_color_and_closes_picker_from_action() {
    let mut controller = ThemeController::new(MaterialColor::Purple, true);
    let viewport = Size::new(1080.0, 980.0);
    let bottom_margin = FLOATING_MARGIN;
    let now = Instant::now();

    controller.update(ThemeAction::TogglePicker, viewport, bottom_margin, now);
    assert!(controller.is_picker_open());

    controller.update(
        ThemeAction::SelectColor(MaterialColor::Blue),
        viewport,
        bottom_margin,
        now,
    );

    let transition = controller
        .transition()
        .expect("selecting a different color should animate");

    assert_eq!(controller.selected_color(), MaterialColor::Blue);
    assert!(!controller.is_picker_open());
    assert_eq!(
        transition.origin(),
        swatch_center(viewport, bottom_margin, MaterialColor::Blue)
    );
}

#[test]
fn controller_dark_mode_action_uses_supplied_origin() {
    let mut controller = ThemeController::new(MaterialColor::Purple, true);
    let origin = Point::new(64.0, 512.0);
    let now = Instant::now();

    controller.update(
        ThemeAction::SetDarkMode {
            dark_mode: false,
            origin,
        },
        Size::new(1080.0, 980.0),
        FLOATING_MARGIN,
        now,
    );

    let transition = controller
        .transition()
        .expect("changing dark mode should animate");

    assert!(!controller.dark_mode());
    assert_eq!(transition.origin(), origin);
}

#[test]
fn material_colors_generate_distinct_primary_roles() {
    assert_eq!(MaterialColor::ALL.len(), 8);
    assert_eq!(
        MaterialColor::Purple.color_scheme(false).primary,
        Theme::Light.colors().primary
    );
    assert_ne!(
        MaterialColor::Blue.color_scheme(false).primary,
        Theme::Light.colors().primary
    );
    assert_ne!(
        MaterialColor::Green.color_scheme(true).primary,
        Theme::Dark.colors().primary
    );
    assert_ne!(
        MaterialColor::Blue
            .color_scheme(false)
            .surface
            .container
            .base,
        Theme::Light.colors().surface.container.base
    );
    assert_ne!(
        MaterialColor::Blue.color_scheme(false).secondary.container,
        Theme::Light.colors().secondary.container
    );
}

#[test]
fn material_colors_tint_menu_backgrounds() {
    let baseline = crate::style::menu::default(&Theme::Light);
    let blue = Theme::new("Blue", MaterialColor::Blue.color_scheme(false));
    let tinted = crate::style::menu::default(&blue);

    assert_ne!(tinted.background, baseline.background);
    assert_ne!(tinted.selected_background, baseline.selected_background);
}

#[test]
fn selected_swatch_uses_stronger_outline() {
    let theme = Theme::Light;
    let selected = swatch_style(&theme, Status::Active, MaterialColor::Blue, true);
    let unselected = swatch_style(&theme, Status::Active, MaterialColor::Blue, false);

    assert_eq!(selected.border.width, SELECTED_SWATCH_OUTLINE_WIDTH);
    assert_eq!(unselected.border.width, SWATCH_OUTLINE_WIDTH);
}

#[test]
fn palette_button_uses_rounded_square_without_circular_toolbar_base() {
    let style = palette_button_style(&Theme::Dark, Status::Active, false);

    assert_eq!(style.border.radius.top_left, PALETTE_BUTTON_SHAPE);
    assert_ne!(style.border.radius.top_left, tokens::shape::CORNER_FULL);
}

#[test]
fn swatch_centers_track_picker_layout() {
    let viewport = Size::new(1080.0, 980.0);
    let bottom_margin = FLOATING_MARGIN;
    let purple = swatch_center(viewport, bottom_margin, MaterialColor::Purple);
    let blue = swatch_center(viewport, bottom_margin, MaterialColor::Blue);
    let yellow = swatch_center(viewport, bottom_margin, MaterialColor::Yellow);

    assert_eq!(blue.x - purple.x, SWATCH_TARGET_SIZE + PICKER_PANEL_SPACING);
    assert_eq!(
        yellow.y - purple.y,
        SWATCH_TARGET_SIZE + PICKER_PANEL_SPACING
    );
    assert!(purple.x < palette_center(viewport, bottom_margin).x);
}

#[test]
fn reveal_overlay_uses_android_style_thresholds() {
    assert_eq!(percent_past_threshold(0.5, 0.5), 0.0);
    assert_eq!(percent_past_threshold(1.0, 0.5), 1.0);
    assert_eq!(reveal_gradient_end_alpha(1.0), 0.0);
    assert_eq!(
        reveal_gradient_end_alpha(THEME_REVEAL_EDGE_FADE_THRESHOLD),
        1.0
    );
    assert!(reveal_gradient_end_alpha(0.80) > reveal_gradient_end_alpha(0.95));
    assert_eq!(reveal_start_fill_alpha(1.0), 0.0);
    assert!(reveal_start_fill_alpha(0.40) > reveal_start_fill_alpha(0.80));
}

#[test]
fn reveal_blur_is_strongest_mid_transition() {
    assert!(reveal_blur_ratio(0.5) > reveal_blur_ratio(0.1));
    assert!(reveal_blur_ratio(0.5) > reveal_blur_ratio(0.9));
    assert!(reveal_blur_width(1000.0, 0.5) > reveal_blur_width(1000.0, 0.0));
}
