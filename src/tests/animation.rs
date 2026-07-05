use super::*;
use crate::Theme;

#[test]
fn material_theme_transition_uses_m3_easing_and_duration() {
    let start = Instant::now();
    let transition =
        ColorSchemeTransition::material_theme(Theme::Dark.colors(), Theme::Light.colors(), start);

    assert_eq!(transition.duration, Duration::from_millis(400));
    assert_eq!(
        transition.easing,
        tokens::motion::EASING_EMPHASIZED_DECELERATE
    );
    assert_eq!(transition.progress_at(start), 0.0);
    assert!(!transition.is_finished_at(start + Duration::from_millis(200)));
    assert!(transition.is_finished_at(start + Duration::from_millis(400)));
}

#[test]
fn color_scheme_transition_reaches_target() {
    let start = Instant::now();
    let target = Theme::Light.colors();
    let transition = ColorSchemeTransition::material_theme(Theme::Dark.colors(), target, start);

    assert_eq!(
        transition.value_at(start + Duration::from_millis(500)),
        target
    );
}

#[test]
fn theme_reveal_transition_tracks_origin_and_radius() {
    let start = Instant::now();
    let origin = Point::new(3.0, 4.0);
    let target = Theme::Light.colors();
    let transition =
        ThemeRevealTransition::material_theme(Theme::Dark.colors(), target, origin, start);

    assert_eq!(transition.origin(), origin);
    assert_eq!(transition.reveal_radius_at(Size::new(6.0, 8.0), start), 0.0);
    assert!(
        transition.reveal_radius_at(Size::new(6.0, 8.0), start + Duration::from_millis(200)) > 0.0
    );
    assert!(!transition.is_finished_at(start + Duration::from_millis(1_000)));
    assert!(transition.is_finished_at(start + Duration::from_millis(THEME_REVEAL_DURATION_MS)));
    assert_eq!(transition.target(), target);
}
