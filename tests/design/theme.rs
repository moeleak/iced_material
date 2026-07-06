use super::{ColorScheme, color};

#[test]
fn light_scheme_matches_m3_baseline_roles() {
    let scheme = ColorScheme::LIGHT;

    assert_eq!(scheme.primary.color, color!(0x6750a4));
    assert_eq!(scheme.primary.text, color!(0xffffff));
    assert_eq!(scheme.primary.container, color!(0xeaddff));
    assert_eq!(scheme.surface.color, color!(0xfef7ff));
    assert_eq!(scheme.surface.container.low, color!(0xf7f2fa));
    assert_eq!(scheme.surface.container.highest, color!(0xe6e0e9));
    assert_eq!(scheme.outline.color, color!(0x79747e));
}

#[test]
fn dark_scheme_matches_m3_baseline_roles() {
    let scheme = ColorScheme::DARK;

    assert_eq!(scheme.primary.color, color!(0xd0bcff));
    assert_eq!(scheme.primary.text, color!(0x381e72));
    assert_eq!(scheme.primary.container, color!(0x4f378b));
    assert_eq!(scheme.surface.color, color!(0x141218));
    assert_eq!(scheme.surface.container.lowest, color!(0x0f0d13));
    assert_eq!(scheme.surface.container.highest, color!(0x36343b));
    assert_eq!(scheme.outline.color, color!(0x938f99));
}

#[test]
fn color_scheme_interpolation_clamps_to_endpoints() {
    assert_eq!(
        ColorScheme::interpolate(ColorScheme::DARK, ColorScheme::LIGHT, -1.0),
        ColorScheme::DARK
    );
    assert_eq!(
        ColorScheme::interpolate(ColorScheme::DARK, ColorScheme::LIGHT, 2.0),
        ColorScheme::LIGHT
    );
}

#[test]
fn color_scheme_interpolation_moves_all_roles() {
    let midpoint = ColorScheme::interpolate(ColorScheme::DARK, ColorScheme::LIGHT, 0.5);

    assert_ne!(midpoint.primary.color, ColorScheme::DARK.primary.color);
    assert_ne!(midpoint.primary.color, ColorScheme::LIGHT.primary.color);
    assert_ne!(midpoint.surface.color, ColorScheme::DARK.surface.color);
    assert_ne!(midpoint.surface.color, ColorScheme::LIGHT.surface.color);
    assert_ne!(midpoint.outline.color, ColorScheme::DARK.outline.color);
    assert_ne!(midpoint.outline.color, ColorScheme::LIGHT.outline.color);
}
