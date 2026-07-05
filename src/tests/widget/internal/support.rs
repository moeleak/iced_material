use super::*;

type TestParagraph = <iced_widget::Renderer as core_text::Renderer>::Paragraph;

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
