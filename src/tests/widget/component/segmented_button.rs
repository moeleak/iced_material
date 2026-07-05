use super::*;

#[test]
fn segment_position_sets_outer_radii_only() {
    let full = tokens::component::segmented_button::CONTAINER_SHAPE;

    assert_eq!(SegmentPosition::for_index(0, 1), SegmentPosition::Only);
    assert_eq!(SegmentPosition::for_index(0, 3), SegmentPosition::First);
    assert_eq!(SegmentPosition::for_index(1, 3), SegmentPosition::Middle);
    assert_eq!(SegmentPosition::for_index(2, 3), SegmentPosition::Last);
    assert_eq!(SegmentPosition::Only.radius(), Radius::new(full));
    assert_eq!(SegmentPosition::First.radius().top_left, full);
    assert_eq!(SegmentPosition::First.radius().top_right, 0.0);
    assert_eq!(SegmentPosition::Middle.radius(), Radius::default());
    assert_eq!(SegmentPosition::Last.radius().top_right, full);
    assert_eq!(SegmentPosition::Last.radius().bottom_left, 0.0);
}

#[test]
fn group_overlaps_adjacent_outlines_by_border_width() {
    assert_eq!(
        segment_overlap_spacing(),
        -tokens::component::segmented_button::OUTLINE_WIDTH
    );
}

#[test]
fn selected_segment_uses_secondary_container_tokens() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = segmented_style(&theme, Status::Active, true, SegmentPosition::Only);

    assert_eq!(
        style.background,
        Some(Background::Color(colors.secondary.container))
    );
    assert_eq!(style.text_color, colors.secondary.container_text);
    assert_eq!(
        style.border.width,
        tokens::component::segmented_button::OUTLINE_WIDTH
    );
}

#[test]
fn selection_state_crossfades_previous_and_selected_segments() {
    let now = Instant::now();
    let mut state = State::new(0);

    state.select(2, now);

    assert_eq!(state.selected_index(), 2);
    assert_eq!(state.progress_for(0), 1.0);
    assert_eq!(state.progress_for(2), 0.0);

    let _ = state.advance(now + duration_ms(100));
    assert!(state.progress_for(0) < 1.0);
    assert!(state.progress_for(2) > 0.0);
}

#[test]
fn segmented_style_progress_interpolates_selected_fill_and_text() {
    let theme = Theme::Light;
    let colors = theme.colors();
    let style = segmented_style_progress(&theme, Status::Active, 0.5, SegmentPosition::Only);

    assert_eq!(
        style.background,
        Some(Background::Color(Color {
            a: colors.secondary.container.a * 0.5,
            ..colors.secondary.container
        }))
    );
    assert_eq!(
        style.text_color,
        mix(colors.surface.text, colors.secondary.container_text, 0.5)
    );
}
