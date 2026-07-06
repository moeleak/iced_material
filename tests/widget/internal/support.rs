use super::*;

type TestParagraph = <iced_widget::Renderer as core_text::Renderer>::Paragraph;

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 0.001,
        "expected {expected}, got {actual}",
    );
}

#[test]
fn animated_scalar_retargets_from_last_advanced_value() {
    let start = Instant::now();
    let mut scalar = AnimatedScalar::new(0.0);

    scalar.set_target(1.0, start, duration_ms(100), tokens::motion::EASING_LINEAR);
    assert!(scalar.advance(start + duration_ms(50)));
    scalar.set_target(
        0.0,
        start + duration_ms(80),
        duration_ms(100),
        tokens::motion::EASING_LINEAR,
    );

    assert_close(scalar.value, 0.5);
    assert_eq!(scalar.to, 0.0);

    assert!(scalar.advance(start + duration_ms(130)));
    assert_close(scalar.value, 0.25);
}

#[test]
fn text_field_state_tracks_active_ime_preedit() {
    let mut state = TextFieldState::<TestParagraph>::new(false);

    assert!(!state.ime_preedit_active);
    assert!(state.set_ime_preedit("pin yin"));
    assert!(state.ime_preedit_active);
    assert!(!state.set_ime_preedit("more"));
    assert!(state.clear_ime_preedit());
    assert!(!state.ime_preedit_active);
    assert!(!state.clear_ime_preedit());
}
