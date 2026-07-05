use super::*;

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
