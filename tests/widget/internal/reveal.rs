use iced_widget::core::time::{Duration, Instant};

use super::*;

#[test]
fn reveal_uses_selection_field_open_springs() {
    let start = Instant::now();
    let mut animation = RevealAnimation::closed();

    animation.open(start);

    assert_eq!(animation.spatial.to, OPEN_TARGET);
    assert_eq!(animation.effects.to, OPEN_TARGET);
    assert_eq!(
        tokens::motion::EXPRESSIVE_SLOW_SPATIAL,
        tokens::motion::Spring {
            damping_ratio: 0.8,
            stiffness: 200.0,
        }
    );
    assert_eq!(
        tokens::motion::EXPRESSIVE_FAST_EFFECTS,
        tokens::motion::Spring {
            damping_ratio: 1.0,
            stiffness: 3800.0,
        }
    );
    assert!(animation.is_visible());
    assert!(!animation.frame().is_closing);
}

#[test]
fn reveal_close_stays_visible_until_reverse_animation_finishes() {
    let start = Instant::now();
    let mut animation = RevealAnimation::closed();

    animation.open(start);
    assert!(!animation.advance(start + Duration::from_secs(2)));
    animation.close(start + Duration::from_secs(2));

    assert!(animation.is_visible());
    assert!(animation.frame().is_closing);
    assert_eq!(animation.spatial.to, CLOSED_TARGET);
    assert_eq!(animation.effects.to, CLOSED_TARGET);

    assert!(!animation.advance(start + Duration::from_secs(4)));
    assert!(!animation.is_visible());
    assert_eq!(animation.frame().reveal, 0.0);
    assert_eq!(animation.frame().alpha, 0.0);
}

#[test]
fn reveal_reverses_from_the_current_frame() {
    let start = Instant::now();
    let mut animation = RevealAnimation::closed();

    animation.open(start);
    assert!(animation.advance(start + Duration::from_millis(100)));
    animation.close(start + Duration::from_millis(100));
    assert!(animation.advance(start + Duration::from_millis(150)));
    let closing = animation.frame();

    animation.open(start + Duration::from_millis(150));

    assert_eq!(animation.frame().reveal, closing.reveal);
    assert_eq!(animation.frame().alpha, closing.alpha);
    assert!(!animation.frame().is_closing);
}
