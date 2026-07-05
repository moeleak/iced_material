use super::*;

fn background_color(background: Background) -> Color {
    match background {
        Background::Color(color) => color,
        Background::Gradient(_) => panic!("expected solid color"),
    }
}

#[test]
fn disabled_checkbox_uses_m3_component_opacity_tokens() {
    let checked = default(&Theme::Light, Status::Disabled { is_checked: true });
    assert_eq!(
        background_color(checked.background).a,
        tokens::component::checkbox::SELECTED_DISABLED_CONTAINER_OPACITY
    );
    assert_eq!(
        checked.text_color.unwrap().a,
        tokens::state::DISABLED_LABEL_TEXT_OPACITY
    );

    let unchecked = default(&Theme::Light, Status::Disabled { is_checked: false });
    assert_eq!(
        unchecked.border.color.a,
        tokens::component::checkbox::UNSELECTED_DISABLED_CONTAINER_OPACITY
    );
    assert_eq!(
        unchecked.border.width,
        tokens::component::checkbox::UNSELECTED_DISABLED_OUTLINE_WIDTH
    );
    assert_eq!(
        unchecked.text_color.unwrap().a,
        tokens::state::DISABLED_LABEL_TEXT_OPACITY
    );
}
