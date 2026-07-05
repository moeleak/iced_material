use super::*;

#[test]
fn display_value_uses_typed_query_text() {
    assert_eq!(DisplayValue::<&str>::Input("xxx".into()).to_string(), "xxx");
}

#[test]
fn state_preserves_original_options() {
    let mut state = State::new(vec!["Assist", "Suggestion"]);

    state.push("Filter");

    assert_eq!(state.options(), &["Assist", "Suggestion", "Filter"]);
}

#[test]
fn combobox_option_padding_produces_m3_menu_item_height() {
    let state = State::new(vec!["Assist", "Suggestion", "Filter"]);
    let combobox: Combobox<'_, _, (), iced_widget::Renderer> =
        outlined_with_input(&state, "Search", "Suggestion", None, |_| ());
    let padding = combobox.inner.option_padding;

    assert_eq!(
        tokens::component::text_field::INPUT_TEXT_LINE_HEIGHT + padding.y(),
        tokens::component::select::MENU_LIST_ITEM_CONTAINER_HEIGHT
    );
}
