use super::*;
use iced_widget::core::Background;

#[test]
fn default_slider_uses_m3_track_and_handle_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = default(&theme, Status::Active);

    assert_eq!(
        style.rail.width,
        tokens::component::slider::ACTIVE_TRACK_HEIGHT
    );
    assert_eq!(
        style.rail.backgrounds,
        (
            Background::Color(colors.primary.color),
            Background::Color(colors.surface.container.highest)
        )
    );
    assert_eq!(
        style.handle.shape,
        HandleShape::Circle {
            radius: tokens::component::slider::HANDLE_RADIUS
        }
    );
    assert_eq!(
        style.handle.background,
        Background::Color(colors.primary.color)
    );
}
