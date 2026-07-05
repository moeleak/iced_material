use super::*;

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 0.001,
        "expected {expected}, got {actual}",
    );
}

#[test]
fn ripple_radius_matches_android_auto_radius_for_non_round_bounds() {
    let radius = ripple_target_radius(Size::new(100.0, 40.0));

    assert_close(radius, (50.0_f32 * 50.0 + 20.0 * 20.0).sqrt());
}

#[test]
fn ripple_radius_uses_android_auto_radius_for_round_bounds() {
    let radius = ripple_target_radius(Size::new(40.0, 40.0));

    assert_close(radius, (20.0_f32 * 20.0 + 20.0 * 20.0).sqrt());
}

#[test]
fn partial_round_bounds_use_android_auto_radius() {
    let radius = ripple_target_radius(Size::new(80.0, 40.0));

    assert_close(radius, (40.0_f32 * 40.0 + 20.0 * 20.0).sqrt());
}

#[test]
fn ripple_starts_at_android_foreground_start_radius() {
    let start = Instant::now();
    let ripple = Ripple::new(Point::new(50.0, 20.0), start);
    let circle = ripple.circle(Size::new(100.0, 40.0), start);

    assert_close(circle.radius, 30.0);
}

#[test]
fn ripple_enter_opacity_is_linear() {
    let start = Instant::now();
    let ripple = Ripple::new(Point::new(20.0, 20.0), start);

    assert_close(ripple.opacity(start), 0.0);
    assert_close(ripple.opacity(start + duration_ms(75)), 1.0);
}

#[test]
fn rounded_rect_span_clips_full_round_corners() {
    let size = Size::new(40.0, 40.0);
    let radius = border::radius(9999.0);

    let top = rounded_rect_span_at_y(size, radius, 0.0).unwrap();
    assert_close(top.0, 20.0);
    assert_close(top.1, 20.0);

    let middle = rounded_rect_span_at_y(size, radius, 20.0).unwrap();
    assert_close(middle.0, 0.0);
    assert_close(middle.1, 40.0);

    let upper = rounded_rect_span_at_y(size, radius, 10.0).unwrap();
    assert_close(upper.0, 20.0 - (20.0_f32 * 20.0 - 10.0 * 10.0).sqrt());
    assert_close(upper.1, 20.0 + (20.0_f32 * 20.0 - 10.0 * 10.0).sqrt());
}

#[test]
fn rounded_rect_span_keeps_square_bounds_without_radius() {
    let span =
        rounded_rect_span_at_y(Size::new(80.0, 40.0), border::Radius::default(), 8.0).unwrap();

    assert_close(span.0, 0.0);
    assert_close(span.1, 80.0);
}

#[test]
fn ripple_clip_sampling_is_bounded_for_runtime_cost() {
    assert_eq!(ripple_clip_sample_count(1.0), RIPPLE_CLIP_MIN_SAMPLES);
    assert_eq!(ripple_clip_sample_count(100.0), RIPPLE_CLIP_MAX_SAMPLES);
}

#[test]
fn short_press_ripple_holds_before_fade_out() {
    let start = Instant::now();
    let mut ripple = Ripple::new(Point::new(20.0, 20.0), start);
    let release = start + duration_ms(50);

    ripple.exit(release);

    assert_eq!(ripple.exit_delay, duration_ms(175));
    assert_close(ripple.opacity(release + duration_ms(174)), 1.0);
    assert_close(ripple.opacity(release + duration_ms(250)), 0.5);
    assert!(ripple.has_finished_exit(release + duration_ms(325)));
}

#[test]
fn pressing_again_moves_existing_active_ripple_to_exiting() {
    let start = Instant::now();
    let mut state = ButtonState::default();

    state.press(Point::new(10.0, 10.0), start);
    state.press(Point::new(20.0, 20.0), start + duration_ms(20));

    assert!(state.active_ripple.is_some());
    assert_eq!(state.exiting_ripples.len(), 1);
    assert!(state.has_visible_ripples(start + duration_ms(75)));
}

#[test]
fn ripple_opacity_tracks_max_visible_alpha() {
    let start = Instant::now();
    let release = start + duration_ms(50);
    let mut state = ButtonState::default();

    state.press(Point::new(10.0, 10.0), start);
    state.release(release);

    assert_close(state.ripple_opacity(release + duration_ms(25)), 1.0);
    assert_close(state.ripple_opacity(release + duration_ms(250)), 0.5);
    assert_close(state.ripple_opacity(release + duration_ms(325)), 0.0);
}

#[test]
fn ripple_fade_reveals_hover_state_layer_background() {
    let theme = Theme::Light;
    let class: StyleFn<'_, Theme> = Box::new(crate::style::button::text);
    let active = button_draw_style(&theme, &class, Status::Active, 0.0);
    let hovered = button_draw_style(&theme, &class, Status::Hovered, 0.0);
    let covered = button_draw_style(&theme, &class, Status::Hovered, 1.0);
    let fading = button_draw_style(&theme, &class, Status::Hovered, 0.5);

    assert!(hovered.background.is_some());
    assert_eq!(covered.background, active.background);

    let Some(Background::Color(hovered_color)) = hovered.background else {
        panic!("expected hovered color background");
    };
    let Some(Background::Color(fading_color)) = fading.background else {
        panic!("expected fading color background");
    };

    assert!(fading_color.a > 0.0);
    assert!(fading_color.a < hovered_color.a);
}

#[test]
fn hover_state_layer_progress_inverts_ripple_opacity() {
    assert_close(hover_state_layer_progress(1.0), 0.0);
    assert_close(hover_state_layer_progress(0.25), 0.75);
    assert_close(hover_state_layer_progress(0.0), 1.0);
}

#[test]
fn ripple_origin_clamps_to_android_foreground_radius() {
    let size = Size::new(100.0, 40.0);
    let target_radius = ripple_target_radius(size);
    let start_radius = size.width.max(size.height) * RIPPLE_START_RADIUS_FACTOR;
    let clamped =
        clamped_ripple_origin(Point::new(500.0, 500.0), size, target_radius, start_radius);
    let center = Point::new(50.0, 20.0);
    let dx = clamped.x - center.x;
    let dy = clamped.y - center.y;

    assert_close(
        (dx * dx + dy * dy).sqrt(),
        (target_radius - start_radius).max(0.0),
    );
}

#[test]
fn press_origin_ignores_mouse_outside_bounds() {
    let bounds = Rectangle {
        x: 10.0,
        y: 20.0,
        width: 40.0,
        height: 30.0,
    };
    let event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));

    assert_eq!(
        press_origin(
            &event,
            bounds,
            mouse::Cursor::Available(Point::new(80.0, 80.0))
        ),
        None
    );
}

#[test]
fn press_origin_returns_mouse_position_relative_to_bounds() {
    let bounds = Rectangle {
        x: 10.0,
        y: 20.0,
        width: 40.0,
        height: 30.0,
    };
    let event = Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left));

    assert_eq!(
        press_origin(
            &event,
            bounds,
            mouse::Cursor::Available(Point::new(25.0, 35.0))
        ),
        Some(Point::new(15.0, 15.0))
    );
}

#[test]
fn touch_press_origin_prefers_translated_cursor_position() {
    let bounds = Rectangle {
        x: 10.0,
        y: 120.0,
        width: 40.0,
        height: 30.0,
    };
    let event = Event::Touch(touch::Event::FingerPressed {
        id: touch::Finger(0),
        position: Point::new(25.0, 35.0),
    });

    assert_eq!(
        press_origin(
            &event,
            bounds,
            mouse::Cursor::Available(Point::new(25.0, 135.0))
        ),
        Some(Point::new(15.0, 15.0))
    );
}

#[test]
fn touch_press_origin_does_not_fallback_to_raw_position_when_cursor_is_available() {
    let bounds = Rectangle {
        x: 10.0,
        y: 20.0,
        width: 40.0,
        height: 30.0,
    };
    let event = Event::Touch(touch::Event::FingerPressed {
        id: touch::Finger(0),
        position: Point::new(25.0, 35.0),
    });

    assert_eq!(
        press_origin(
            &event,
            bounds,
            mouse::Cursor::Available(Point::new(25.0, 135.0))
        ),
        None
    );
}

#[test]
fn touch_press_origin_does_not_fallback_to_raw_position_when_cursor_is_levitating() {
    let bounds = Rectangle {
        x: 10.0,
        y: 20.0,
        width: 40.0,
        height: 30.0,
    };
    let event = Event::Touch(touch::Event::FingerPressed {
        id: touch::Finger(0),
        position: Point::new(25.0, 35.0),
    });

    assert_eq!(
        press_origin(
            &event,
            bounds,
            mouse::Cursor::Levitating(Point::new(25.0, 135.0))
        ),
        None
    );
}

#[test]
fn touch_release_uses_translated_cursor_position() {
    let bounds = Rectangle {
        x: 10.0,
        y: 120.0,
        width: 40.0,
        height: 30.0,
    };
    let event = Event::Touch(touch::Event::FingerLifted {
        id: touch::Finger(0),
        position: Point::new(25.0, 35.0),
    });

    assert!(release_is_over(
        &event,
        bounds,
        mouse::Cursor::Available(Point::new(25.0, 135.0))
    ));
}

#[test]
fn touch_release_does_not_fallback_to_raw_position_when_cursor_is_levitating() {
    let bounds = Rectangle {
        x: 10.0,
        y: 20.0,
        width: 40.0,
        height: 30.0,
    };
    let event = Event::Touch(touch::Event::FingerLifted {
        id: touch::Finger(0),
        position: Point::new(25.0, 35.0),
    });

    assert!(!release_is_over(
        &event,
        bounds,
        mouse::Cursor::Levitating(Point::new(25.0, 135.0))
    ));
}

#[test]
fn touch_press_origin_falls_back_to_raw_position_without_cursor() {
    let bounds = Rectangle {
        x: 10.0,
        y: 20.0,
        width: 40.0,
        height: 30.0,
    };
    let event = Event::Touch(touch::Event::FingerPressed {
        id: touch::Finger(0),
        position: Point::new(25.0, 35.0),
    });

    assert_eq!(
        press_origin(&event, bounds, mouse::Cursor::Unavailable),
        Some(Point::new(15.0, 15.0))
    );
}

#[test]
fn touch_move_beyond_click_slop_cancels_click_candidate() {
    let press_position = Some(Point::new(25.0, 35.0));
    let small_move = Event::Touch(touch::Event::FingerMoved {
        id: touch::Finger(0),
        position: Point::new(25.0 + TOUCH_CLICK_SLOP / 2.0, 35.0),
    });
    let scroll_move = Event::Touch(touch::Event::FingerMoved {
        id: touch::Finger(0),
        position: Point::new(25.0 + TOUCH_CLICK_SLOP + 1.0, 35.0),
    });

    assert!(!touch_moved_beyond_click_slop(
        press_position,
        &small_move,
        mouse::Cursor::Unavailable
    ));
    assert!(touch_moved_beyond_click_slop(
        press_position,
        &scroll_move,
        mouse::Cursor::Unavailable
    ));
}
