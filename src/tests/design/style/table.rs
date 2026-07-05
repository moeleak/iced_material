use super::*;
use crate::tokens;

#[test]
fn default_table_uses_m3_data_table_outline_color() {
    let theme = Theme::Light;
    let separator = Background::Color(theme.colors().outline.variant);
    let style = default(&theme);

    assert_eq!(style.separator_x, separator);
    assert_eq!(style.separator_y, separator);
    assert_eq!(tokens::component::data_table::OUTLINE_WIDTH, 1.0);
    assert_eq!(tokens::component::data_table::ROW_ITEM_OUTLINE_WIDTH, 1.0);
}
