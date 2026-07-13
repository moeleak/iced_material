use super::*;
use iced_widget::core::time::{Duration, Instant};

#[test]
fn material_menu_height_uses_five_visible_options_max() {
    assert_eq!(
        material_menu_height(3),
        Length::Fixed(tokens::component::select::MENU_LIST_ITEM_CONTAINER_HEIGHT * 3.0)
    );
    assert_eq!(
        material_menu_height(8),
        Length::Fixed(
            tokens::component::select::MENU_LIST_ITEM_CONTAINER_HEIGHT * MAX_VISIBLE_OPTIONS as f32
        )
    );
}

#[test]
fn material_option_padding_produces_m3_menu_item_height() {
    let padding = menu_option_padding();

    assert_eq!(
        tokens::component::text_field::INPUT_TEXT_LINE_HEIGHT + padding.y(),
        tokens::component::select::MENU_LIST_ITEM_CONTAINER_HEIGHT
    );
}

#[test]
fn select_status_tracks_open_state_before_hover_state() {
    assert_eq!(select_status(false, false), Status::Active);
    assert_eq!(select_status(false, true), Status::Hovered);
    assert_eq!(
        select_status(true, false),
        Status::Opened { is_hovered: false }
    );
    assert_eq!(
        select_status(true, true),
        Status::Opened { is_hovered: true }
    );
}

#[test]
fn select_menu_remains_visible_during_close_animation() {
    let start = Instant::now();
    let mut menu = menu_overlay::State::new();

    menu.start_open(3, start);
    assert!(!menu.advance(start + Duration::from_secs(2)));
    menu.start_close(start + Duration::from_secs(2));

    assert!(menu.is_visible());
    assert!(menu.is_animating());

    assert!(!menu.advance(start + Duration::from_secs(4)));
    assert!(!menu.is_visible());
}

#[test]
fn default_handle_rotation_matches_compose_trailing_icon_targets() {
    assert_eq!(menu_handle_rotation_target(false), 0.0);
    assert_eq!(menu_handle_rotation_target(true), 1.0);
    assert_eq!(menu_handle_rotation_radians(0.0), 0.0);
    assert_eq!(
        menu_handle_rotation_radians(0.5),
        std::f32::consts::FRAC_PI_2
    );
    assert_eq!(menu_handle_rotation_radians(1.0), std::f32::consts::PI);
}

#[test]
fn default_handle_arrow_points_match_material_icon_viewbox() {
    assert_eq!(
        default_handle_arrow_points(MENU_HANDLE_VIEWPORT_SIZE),
        [
            Point::new(7.0, 10.0),
            Point::new(12.0, 15.0),
            Point::new(17.0, 10.0),
        ]
    );
    assert_eq!(
        default_handle_arrow_points(MENU_HANDLE_VIEWPORT_SIZE / 2.0),
        [
            Point::new(3.5, 5.0),
            Point::new(6.0, 7.5),
            Point::new(8.5, 5.0),
        ]
    );
}

#[test]
fn select_prefers_down_when_menu_fits_below() {
    let position = Point::new(0.0, 500.0);
    let target_height = 56.0;
    let viewport = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 800.0,
        height: 940.0,
    };

    let anchor = prefer_down_when_menu_fits(position, viewport, target_height, 144.0);

    assert_eq!(anchor.position.y + anchor.target_height, 556.0);
    assert!(viewport.height - (anchor.position.y + anchor.target_height) > anchor.position.y);
}

#[test]
fn select_keeps_default_anchor_when_menu_does_not_fit_below() {
    let position = Point::new(0.0, 500.0);
    let target_height = 56.0;
    let viewport = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 800.0,
        height: 620.0,
    };

    let anchor = prefer_down_when_menu_fits(position, viewport, target_height, 144.0);

    assert_eq!(anchor.position, position);
    assert_eq!(anchor.target_height, target_height);
}
