use cavalier_contours::core::math::{Vector2, angle, bulge_from_angle, delta_angle_signed};
use cavalier_contours::polyline::{
    PlineVertex, seg_arc_from_radius_center, seg_arc_radius_and_center,
};

#[test]
fn issue_76_arc_from_radius_center_round_trip_ccw() {
    // Repro/coverage for upstream issue #76:
    // https://github.com/jbuckmccready/cavalier_contours/issues/76
    let radius: f64 = 5.0;
    let center = Vector2::new(2.5f64, -3.25f64);
    let start_angle: f64 = 0.35;
    let end_angle: f64 = 1.2;
    let is_clockwise = false;

    let (v1, v2) = seg_arc_from_radius_center(radius, center, start_angle, end_angle, is_clockwise);
    let (result_radius, result_center) = seg_arc_radius_and_center(v1, v2);

    assert!(
        (result_radius - radius).abs() < 1e-10f64,
        "radius mismatch: expected {radius}, got {result_radius}"
    );
    assert!(
        result_center.fuzzy_eq(center),
        "center mismatch: expected {center:?}, got {result_center:?}"
    );
    assert!(v1.bulge > 0.0, "ccw sweep should produce positive bulge");
}

#[test]
fn issue_76_arc_from_radius_center_round_trip_cw() {
    let radius: f64 = 7.0;
    let center = Vector2::new(-4.0f64, 1.5f64);
    let start_angle: f64 = 1.1;
    let end_angle: f64 = -0.2;
    let is_clockwise = true;

    let (v1, v2) = seg_arc_from_radius_center(radius, center, start_angle, end_angle, is_clockwise);
    let (result_radius, result_center) = seg_arc_radius_and_center(v1, v2);

    assert!(
        (result_radius - radius).abs() < 1e-10f64,
        "radius mismatch: expected {radius}, got {result_radius}"
    );
    assert!(
        result_center.fuzzy_eq(center),
        "center mismatch: expected {center:?}, got {result_center:?}"
    );
    assert!(v1.bulge < 0.0, "cw sweep should produce negative bulge");
}

#[test]
fn issue_76_arc_from_radius_center_is_inverse_of_seg_arc_radius_and_center() {
    let original_v1 = PlineVertex::new(4.0f64, 0.0f64, 0.5f64);
    let original_v2 = PlineVertex::new(8.0f64, 0.0f64, 0.0f64);
    let (radius, center) = seg_arc_radius_and_center(original_v1, original_v2);
    let start_angle = angle(center, original_v1.pos());
    let end_angle = angle(center, original_v2.pos());

    let (rebuilt_v1, rebuilt_v2) = seg_arc_from_radius_center(
        radius,
        center,
        start_angle,
        end_angle,
        original_v1.bulge_is_neg(),
    );

    let expected_bulge = bulge_from_angle(delta_angle_signed(
        start_angle,
        end_angle,
        original_v1.bulge_is_neg(),
    ));

    assert!(rebuilt_v1.pos().fuzzy_eq(original_v1.pos()));
    assert!(rebuilt_v2.pos().fuzzy_eq(original_v2.pos()));
    assert!((rebuilt_v1.bulge - expected_bulge).abs() < 1e-12f64);
}
