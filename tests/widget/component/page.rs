use iced_widget::core::Element;

use super::*;

#[derive(Debug, Clone)]
enum Message {}

type TestElement<'a> = Element<'a, Message, Theme, iced_widget::Renderer>;

#[test]
fn page_helpers_compile_to_elements() {
    let header = header("Title", "Subtitle");
    let body = section("Section", Text::new("Body"));
    let _: TestElement<'_> = surface(header, body).into();
    let _: TestElement<'_> = sections([
        section("First", Text::new("Body")).into(),
        section("Second", Text::new("Body")).into(),
    ])
    .into();
    let _: TestElement<'_> = stack([Text::new("One").into(), Text::new("Two").into()]).into();
    let _: TestElement<'_> =
        compact_stack([Text::new("One").into(), Text::new("Two").into()]).into();
    let _: TestElement<'_> =
        component_stack([Text::new("One").into(), Text::new("Two").into()]).into();
    let _: TestElement<'_> = dense_stack([Text::new("One").into(), Text::new("Two").into()]).into();
    let _: TestElement<'_> =
        spacious_stack([Text::new("One").into(), Text::new("Two").into()]).into();
    let _: TestElement<'_> = row([Text::new("One").into(), Text::new("Two").into()]).into();
    let _: TestElement<'_> =
        indicator_row([Text::new("One").into(), Text::new("Two").into()]).into();
    let _: TestElement<'_> = compact_row([Text::new("One").into(), Text::new("Two").into()]).into();
    let _: TestElement<'_> = labeled_value_row("Label", "Value").into();
    let _: TestElement<'_> = divider_row([Text::new("One").into(), Text::new("Two").into()]).into();
    let _: TestElement<'_> = card(super::super::card::elevated, "Card", "Subtitle").into();
    let _: TestElement<'_> = centered_preview(320.0, Text::new("Preview")).into();
    let _: TestElement<'_> = preview_pane(Text::new("Preview")).into();
    let _: TestElement<'_> =
        aligned_preview_pane(alignment::Horizontal::Right, Text::new("Preview")).into();
}

#[test]
fn preview_width_caps_to_material_preview_bounds() {
    assert_eq!(preview_width(1920.0), PREVIEW_MAX_WIDTH);
    assert_eq!(preview_width(420.0), PREVIEW_MIN_WIDTH);
    assert_eq!(PREVIEW_HEIGHT, 260.0);
}
