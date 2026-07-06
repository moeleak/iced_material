use super::*;
use iced_widget::core::Background;

#[test]
fn default_radio_uses_m3_selected_and_unselected_icon_colors() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let active_unselected = default(&theme, Status::Active { is_selected: false });
    assert_eq!(active_unselected.border_color, colors.surface.text_variant);

    let hovered_unselected = default(&theme, Status::Hovered { is_selected: false });
    assert_eq!(hovered_unselected.border_color, colors.surface.text);
    assert_eq!(
        hovered_unselected.background,
        Background::Color(crate::utils::state_layer(
            colors.surface.text,
            crate::tokens::state::HOVER_STATE_LAYER_OPACITY
        ))
    );

    let hovered_selected = default(&theme, Status::Hovered { is_selected: true });
    assert_eq!(hovered_selected.border_color, colors.primary.color);
    assert_eq!(hovered_selected.dot_color, colors.primary.color);
    assert_eq!(
        active_unselected.border_width,
        tokens::component::radio::OUTER_RING_WIDTH
    );
}
