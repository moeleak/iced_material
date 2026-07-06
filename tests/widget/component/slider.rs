use super::*;

#[test]
fn thumb_shadow_uses_material_legacy_slider_elevation() {
    let theme = Theme::Light;
    let shadow = thumb_shadow(&theme);
    let layer = tokens::elevation::shadow(2).ambient;

    assert_eq!(tokens::component::slider::HANDLE_ELEVATION, 2.0);
    assert_eq!(shadow.offset.y, layer.y);
    assert_eq!(shadow.blur_radius, layer.blur);
}

#[test]
fn thumb_bounds_follow_slider_value_fraction() {
    let bounds = Rectangle::new(Point::ORIGIN, Size::new(120.0, 40.0));
    let thumb = thumb_bounds_with_size(
        bounds,
        50.0,
        &(0.0..=100.0),
        tokens::component::slider::HANDLE_WIDTH,
        tokens::component::slider::HANDLE_HEIGHT,
    );

    assert_eq!(thumb.x, 50.0);
    assert_eq!(thumb.y, 10.0);
    assert_eq!(thumb.width, tokens::component::slider::HANDLE_WIDTH);
    assert_eq!(thumb.height, tokens::component::slider::HANDLE_HEIGHT);
}

#[test]
fn thumb_bounds_do_not_panic_for_degenerate_range() {
    let bounds = Rectangle::new(Point::ORIGIN, Size::new(120.0, 40.0));
    let thumb = thumb_bounds_with_size(
        bounds,
        50.0,
        &(100.0..=0.0),
        tokens::component::slider::HANDLE_WIDTH,
        tokens::component::slider::HANDLE_HEIGHT,
    );

    assert_eq!(thumb.x, 0.0);
    assert_eq!(thumb.y, 10.0);
}
