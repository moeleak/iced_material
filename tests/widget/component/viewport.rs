use super::*;

#[test]
fn fixed_height_viewport_separates_visible_and_layout_height() {
    let viewport: Viewport<'_, (), iced_widget::Renderer> =
        Viewport::fixed_height(iced_widget::Space::new(), 40.0, 120.0);

    assert_eq!(viewport.height, Length::Fixed(40.0));
    assert_eq!(viewport.layout_height, Some(120.0));
}
