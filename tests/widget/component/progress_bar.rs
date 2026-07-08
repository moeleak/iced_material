use super::*;

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < 0.001,
        "actual={actual} expected={expected}"
    );
}

#[test]
fn indeterminate_keyframes_match_material3_head_tail_timing() {
    let start = indeterminate_bars(0.0);
    assert_close(start[0].head, 0.0);
    assert_close(start[0].tail, 0.0);
    assert_close(start[1].head, 0.0);
    assert_close(start[1].tail, 0.0);

    let first_head_done = indeterminate_bars(1000.0 / 1750.0);
    assert_close(first_head_done[0].head, 1.0);
    assert!(first_head_done[0].tail > 0.0);

    let second_head_started = indeterminate_bars(700.0 / 1750.0);
    assert!(second_head_started[1].head > 0.0);
    assert_close(second_head_started[1].tail, 0.0);
}

#[test]
fn determinate_wavy_amplitude_flattens_near_edges() {
    assert_eq!(determinate_wave_amplitude(0.0), 0.0);
    assert_eq!(determinate_wave_amplitude(0.1), 0.0);
    assert_eq!(determinate_wave_amplitude(0.5), 1.0);
    assert_eq!(determinate_wave_amplitude(0.95), 0.0);
}

#[test]
fn four_color_indicator_uses_material_color_windows() {
    let primary = Color::from_rgb(1.0, 0.0, 0.0);
    let primary_container = Color::from_rgb(0.0, 1.0, 0.0);
    let tertiary = Color::from_rgb(0.0, 0.0, 1.0);
    let tertiary_container = Color::from_rgb(1.0, 1.0, 0.0);

    assert_eq!(
        four_color_indicator(
            primary,
            primary_container,
            tertiary,
            tertiary_container,
            0.10
        ),
        primary
    );
    assert_eq!(
        four_color_indicator(
            primary,
            primary_container,
            tertiary,
            tertiary_container,
            0.30
        ),
        primary_container
    );
    assert_eq!(
        four_color_indicator(
            primary,
            primary_container,
            tertiary,
            tertiary_container,
            0.55
        ),
        tertiary
    );
    assert_eq!(
        four_color_indicator(
            primary,
            primary_container,
            tertiary,
            tertiary_container,
            0.80
        ),
        tertiary_container
    );
}

#[test]
fn loading_indicator_uses_androidx_material_shape_sequence() {
    let polygons = indeterminate_loading_polygons();

    assert_eq!(
        polygons.len(),
        tokens::component::loading_indicator::INDETERMINATE_SHAPE_COUNT
    );
    assert_eq!(
        polygons.iter().map(corner_count).collect::<Vec<_>>(),
        vec![20, 18, 5, 10, 16, 8, 8]
    );
    assert!(polygons.iter().all(|polygon| !polygon.cubics.is_empty()));
}

#[test]
fn cookie4_uses_androidx_material_shape_definition() {
    let cookie = material_cookie4();
    let repeated = repeat_material_vertices(
        &[
            ShapeVertex::new(1.237, 1.236, CornerRounding::new(0.258)),
            ShapeVertex::new(0.500, 0.918, CornerRounding::new(0.233)),
        ],
        4,
        Point::new(0.5, 0.5),
        false,
    );

    let corners: Vec<_> = cookie
        .features
        .iter()
        .filter(|feature| feature.is_corner())
        .collect();

    assert_eq!(repeated.len(), 8);
    assert_eq!(corners.len(), 8);
    assert!(corners.iter().all(|feature| matches!(
        feature,
        Feature::Corner { cubics, .. } if !cubics.is_empty()
    )));
}

#[test]
fn loading_shape_sequence_wraps_after_seven_shapes() {
    let polygons = indeterminate_loading_polygons();
    let morphs = morph_sequence(&polygons, true);
    let repeated = Morph::new(
        polygons[polygons.len() - 1].normalized(),
        polygons[0].normalized(),
    );

    assert_eq!(morphs.len(), polygons.len());
    assert_eq!(morphs[morphs.len() - 1].pairs, repeated.pairs);
}

#[test]
fn determinate_loading_shape_sequence_uses_circle_to_soft_burst() {
    let polygons = determinate_loading_polygons();

    assert_eq!(polygons.len(), 2);
    assert_eq!(corner_count(&polygons[0]), 10);
    assert_eq!(polygons[1], material_soft_burst());
}

#[test]
fn loading_morphs_use_androidx_feature_mapping() {
    let polygons = indeterminate_loading_polygons();
    let morphs = morph_sequence(&polygons, true);

    for (index, morph) in morphs.iter().enumerate() {
        let from = polygons[index].normalized();
        let to = polygons[(index + 1) % polygons.len()].normalized();

        assert!(!morph.pairs.is_empty());
        assert!(
            morph.pairs.len() >= from.cubics.len().max(to.cubics.len()),
            "morph {index} lost cubic segments"
        );
    }
}

#[test]
fn loading_morphs_are_centered_after_androidx_path_processing() {
    let polygons = indeterminate_loading_polygons();
    let morphs = morph_sequence(&polygons, true);
    let scale = loading_shape_scale(&polygons);
    let target = Point::new(40.0, 72.0);

    for (index, morph) in morphs.iter().enumerate() {
        for progress in [0.0, 0.125, 0.25, 0.5, 0.75, 0.875, 1.0] {
            let cubics = morph.as_cubics(progress);
            let processed = processed_cubics(&cubics, target, 100.0, scale, 0.0);
            let center = bounds_center(cubics_bounds(&processed, false));

            assert_point_close(center, target, index, progress);
        }
    }
}

#[test]
fn loading_morph_spring_reaches_target_before_interval_end() {
    assert_close(loading_spring_progress(0.0), 0.0);
    assert!(loading_spring_progress(0.5) > 0.8);
    assert!(loading_spring_progress(1.0) > 0.99);
}

#[test]
fn loading_shape_scale_accounts_for_rotation_bounds() {
    let polygons = indeterminate_loading_polygons();
    let scale = loading_shape_scale(&polygons);

    assert!(scale > 0.0);
    assert!(scale < tokens::component::loading_indicator::ACTIVE_INDICATOR_SCALE);
}

#[test]
fn degenerate_polygon_normalization_does_not_emit_nan() {
    let polygon = RoundedPolygon::from_vertices(
        &[Point::ORIGIN, Point::ORIGIN, Point::ORIGIN],
        &[CornerRounding::UNROUNDED; 3],
        None,
    );

    let normalized = polygon.normalized();

    assert!(normalized.cubics.iter().all(|cubic| {
        [
            cubic.anchor0_x(),
            cubic.anchor0_y(),
            cubic.control0_x(),
            cubic.control0_y(),
            cubic.control1_x(),
            cubic.control1_y(),
            cubic.anchor1_x(),
            cubic.anchor1_y(),
        ]
        .into_iter()
        .all(f32::is_finite)
    }));
}

fn corner_count(polygon: &RoundedPolygon) -> usize {
    polygon
        .features
        .iter()
        .filter(|feature| feature.is_corner())
        .count()
}

fn assert_point_close(actual: Point, expected: Point, morph_index: usize, progress: f32) {
    assert!(
        (actual.x - expected.x).abs() < 0.001 && (actual.y - expected.y).abs() < 0.001,
        "morph {morph_index} progress {progress} is off-center: actual={actual:?} expected={expected:?}"
    );
}
