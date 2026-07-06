use super::{
    Color, HOVERED_LAYER_OPACITY, PRESSED_LAYER_OPACITY, mix, shadow_from_level, state_layer,
};

#[test]
fn mixing() {
    let base = Color::from_rgba(1.0, 0.0, 0.0, 0.7);
    let overlay = Color::from_rgba(0.0, 1.0, 0.0, 0.2);

    assert_eq!(
        mix(base, overlay, 0.75).into_rgba8(),
        Color::from_linear_rgba(0.53846, 0.46154, 0.0, 0.325).into_rgba8()
    );
}

#[test]
fn state_layer_opacities_match_m3_tokens() {
    assert_eq!(HOVERED_LAYER_OPACITY, 0.08);
    assert_eq!(PRESSED_LAYER_OPACITY, 0.10);
}

#[test]
fn state_layer_preserves_source_alpha() {
    let color = Color::from_rgba(1.0, 0.0, 0.0, 0.5);

    assert_eq!(state_layer(color, 0.10).a, 0.05);
}

#[test]
fn iced_shadow_uses_m3_ambient_layer() {
    let shadow = shadow_from_level(3, Color::BLACK);

    assert_eq!(shadow.offset.y, 4.0);
    assert_eq!(shadow.blur_radius, 8.0);
    assert_eq!(shadow.color.a, 0.15);
}
