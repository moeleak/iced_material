use iced_widget::core::Background;
use iced_widget::text_input::Status;

use super::*;
use crate::tokens;

#[test]
fn default_combobox_uses_m3_outlined_autocomplete_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();

    let input_class = <Theme as Catalog>::default_input();
    let input = <Theme as iced_text_input::Catalog>::style(
        &theme,
        &input_class,
        Status::Focused { is_hovered: false },
    );

    assert_eq!(input.border.color, colors.primary.color);
    assert_eq!(
        input.border.width,
        tokens::component::text_field::FOCUS_OUTLINE_WIDTH
    );

    let menu_class = <Theme as Catalog>::default_menu();
    let menu = <Theme as overlay_menu::Catalog>::style(&theme, &menu_class);

    assert_eq!(
        menu.selected_background,
        Background::Color(colors.surface.container.highest)
    );
    assert_eq!(menu.selected_text_color, colors.surface.text);
}
