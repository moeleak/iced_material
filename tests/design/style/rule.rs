use super::*;

#[test]
fn inset_rule_uses_m3_list_divider_spacing() {
    let style = inset(&Theme::Light);

    assert_eq!(style.color, Theme::Light.colors().outline.variant);
    assert_eq!(
        style.fill_mode,
        FillMode::AsymmetricPadding(
            tokens::component::divider::LIST_ITEM_LEADING_SPACE,
            tokens::component::divider::LIST_ITEM_TRAILING_SPACE
        )
    );
}
