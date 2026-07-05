use super::*;

fn background_color(background: Background) -> Color {
    match background {
        Background::Color(color) => color,
        Background::Gradient(_) => panic!("expected solid color"),
    }
}

#[test]
fn selected_switch_uses_m3_track_and_handle_colors() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let active = default(&theme, Status::Active { is_toggled: true });
    assert_eq!(background_color(active.background), colors.primary.color);
    assert_eq!(background_color(active.foreground), colors.primary.text);

    let hovered = default(&theme, Status::Hovered { is_toggled: true });
    assert_eq!(background_color(hovered.background), colors.primary.color);
    assert_eq!(
        background_color(hovered.foreground),
        colors.primary.container
    );
}

#[test]
fn disabled_switch_uses_m3_switch_opacity_tokens() {
    let theme = Theme::Light;

    let selected = default(&theme, Status::Disabled { is_toggled: true });
    assert_eq!(
        background_color(selected.background).a,
        tokens::component::switch::DISABLED_TRACK_OPACITY
    );
    assert_eq!(
        background_color(selected.foreground).a,
        tokens::component::switch::DISABLED_SELECTED_HANDLE_OPACITY
    );

    let unselected = default(&theme, Status::Disabled { is_toggled: false });
    assert_eq!(
        background_color(unselected.background).a,
        tokens::component::switch::DISABLED_TRACK_OPACITY
    );
    assert_eq!(
        background_color(unselected.foreground).a,
        tokens::component::switch::DISABLED_UNSELECTED_HANDLE_OPACITY
    );
    assert_eq!(
        unselected.background_border_color.a,
        tokens::component::switch::DISABLED_TRACK_OPACITY
    );
}
