use super::*;

#[test]
fn menu_reveal_uses_expressive_motion_scheme_springs() {
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
    assert_eq!(CLOSED_ALPHA_TARGET, 0.0);
    assert_eq!(EXPANDED_ALPHA_TARGET, 1.0);
}

#[test]
fn menu_open_targets_full_reveal_and_alpha() {
    let mut animation = MenuAnimation::closed();
    animation.start_open(Instant::now());

    assert_eq!(animation.reveal.to, EXPANDED_ALPHA_TARGET);
    assert_eq!(animation.alpha.to, EXPANDED_ALPHA_TARGET);
}

#[test]
fn selected_option_background_uses_square_corners() {
    let border = selected_option_border();

    assert_eq!(border.radius.top_left, 0.0);
    assert_eq!(border.radius.top_right, 0.0);
    assert_eq!(border.radius.bottom_right, 0.0);
    assert_eq!(border.radius.bottom_left, 0.0);
}

#[test]
fn reveal_bounds_expand_from_anchor_edge() {
    let bounds = Rectangle::new(Point::new(8.0, 16.0), Size::new(200.0, 300.0));
    let down = MenuAnimationFrame {
        reveal: 0.25,
        alpha: 1.0,
        opens_down: true,
    };
    let up = MenuAnimationFrame {
        reveal: 0.25,
        alpha: 1.0,
        opens_down: false,
    };

    assert_eq!(down.reveal_bounds(bounds).y, 16.0);
    assert_eq!(down.reveal_bounds(bounds).height, 75.0);
    assert_eq!(up.reveal_bounds(bounds).y, 241.0);
    assert_eq!(up.reveal_bounds(bounds).height, 75.0);
}
