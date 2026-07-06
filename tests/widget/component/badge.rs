use super::*;

#[test]
fn badged_box_position_matches_material_offsets() {
    let anchor = Size::new(24.0, 24.0);

    let small = badged_box_badge_position(
        anchor,
        Size::new(
            tokens::component::badge::SMALL_SIZE,
            tokens::component::badge::SMALL_SIZE,
        ),
        BadgedBoxPlacement::IconOnly,
    );
    assert_eq!(small.x, 18.0);
    assert_eq!(small.y, 0.0);

    let large = badged_box_badge_position(
        anchor,
        Size::new(
            tokens::component::badge::LARGE_CONTAINER_HEIGHT,
            tokens::component::badge::LARGE_CONTAINER_HEIGHT,
        ),
        BadgedBoxPlacement::WithContent,
    );
    assert_eq!(large.x, 12.0);
    assert_eq!(large.y, -2.0);
}
