use super::*;

#[test]
fn default_progress_bar_uses_m3_linear_indicator_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = default(&theme);

    assert_eq!(
        style.background,
        Background::Color(colors.surface.container.highest)
    );
    assert_eq!(style.bar, Background::Color(colors.primary.color));
    assert_eq!(
        style.border.radius.top_left,
        tokens::component::linear_progress::TRACK_SHAPE
    );
}
