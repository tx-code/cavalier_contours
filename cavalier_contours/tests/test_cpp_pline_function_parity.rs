mod test_utils;

use cavalier_contours::core::math::Vector2;
use cavalier_contours::polyline::{BooleanOp, PlineSource, Polyline};
use cavalier_contours::{pline_closed, pline_closed_userdata, pline_open};
use test_utils::{create_property_set, property_sets_match};

const EPS: f64 = 1e-4;

fn cpp_ccw_circle_x_aligned() -> Polyline<f64> {
    pline_closed![(-4.0, 1.0, 1.0), (6.0, 1.0, 1.0)]
}

fn cpp_cw_circle_x_aligned() -> Polyline<f64> {
    pline_closed![(-4.0, 1.0, -1.0), (6.0, 1.0, -1.0)]
}

#[test]
fn cpp_circle_function_metrics_parity() {
    let ccw = cpp_ccw_circle_x_aligned();
    let cw = cpp_cw_circle_x_aligned();

    assert!((ccw.area() - std::f64::consts::PI * 25.0).abs() <= EPS);
    assert!((cw.area() + std::f64::consts::PI * 25.0).abs() <= EPS);
    assert!((ccw.path_length() - std::f64::consts::PI * 10.0).abs() <= EPS);
    assert!((cw.path_length() - std::f64::consts::PI * 10.0).abs() <= EPS);

    let ccw_extents = ccw.extents().unwrap();
    let cw_extents = cw.extents().unwrap();
    let expected = (-4.0, -4.0, 6.0, 6.0);

    assert!((ccw_extents.min_x - expected.0).abs() <= EPS);
    assert!((ccw_extents.min_y - expected.1).abs() <= EPS);
    assert!((ccw_extents.max_x - expected.2).abs() <= EPS);
    assert!((ccw_extents.max_y - expected.3).abs() <= EPS);

    assert!((cw_extents.min_x - expected.0).abs() <= EPS);
    assert!((cw_extents.min_y - expected.1).abs() <= EPS);
    assert!((cw_extents.max_x - expected.2).abs() <= EPS);
    assert!((cw_extents.max_y - expected.3).abs() <= EPS);
}

#[test]
fn cpp_circle_winding_number_parity() {
    let ccw = cpp_ccw_circle_x_aligned();
    let cw = cpp_cw_circle_x_aligned();

    let outside = [
        Vector2::new(-4.01, 1.0),
        Vector2::new(6.01, 1.0),
        Vector2::new(1.0, -4.01),
        Vector2::new(1.0, 6.01),
    ];
    let inside = [
        Vector2::new(1.0, 1.0),
        Vector2::new(-3.99, 1.0),
        Vector2::new(5.99, 1.0),
        Vector2::new(1.0, -3.99),
        Vector2::new(1.0, 5.99),
    ];

    for pt in outside {
        assert_eq!(ccw.winding_number(pt), 0);
        assert_eq!(cw.winding_number(pt), 0);
    }

    for pt in inside {
        assert_eq!(ccw.winding_number(pt), 1);
        assert_eq!(cw.winding_number(pt), -1);
    }
}

fn assert_combine_with_self_invariants(input: &Polyline<f64>) {
    let expected = create_property_set([input], false);

    for op in [BooleanOp::Or, BooleanOp::And] {
        let result = input.boolean(input, op);
        assert_eq!(result.pos_plines.len(), 1);
        assert!(result.neg_plines.is_empty());
        let actual = create_property_set(result.pos_plines.iter().map(|p| &p.pline), false);
        assert!(
            property_sets_match(&actual, &expected),
            "combine-with-self parity mismatch for op={op:?}"
        );
    }

    for op in [BooleanOp::Not, BooleanOp::Xor] {
        let result = input.boolean(input, op);
        assert!(result.pos_plines.is_empty());
        assert!(result.neg_plines.is_empty());
    }
}

#[test]
fn cpp_combine_with_self_invariants_parity() {
    let circle = cpp_ccw_circle_x_aligned();
    let rect = pline_closed_userdata![
        [7],
        (0.0, 0.0, 0.0),
        (20.0, 0.0, 0.0),
        (20.0, 10.0, 0.0),
        (0.0, 10.0, 0.0)
    ];

    assert_combine_with_self_invariants(&circle);
    assert_combine_with_self_invariants(&rect);
}

#[test]
fn cpp_circle_closest_point_parity() {
    let circle = cpp_ccw_circle_x_aligned();

    let cases = [
        // Matches old C++ `addClosestPointTestPt` center +/- 0.1 on Y axis.
        (Vector2::new(1.0, 1.1), Vector2::new(1.0, 6.0), 4.9, None),
        (Vector2::new(1.0, 0.9), Vector2::new(1.0, -4.0), 4.9, None),
    ];

    for (query, expected_point, expected_distance, expected_index) in cases {
        let result = circle.closest_point(query, 1e-5).unwrap();
        assert!(
            result.seg_point.fuzzy_eq_eps(expected_point, EPS),
            "closest point mismatch for query={query:?}"
        );
        assert!(
            (result.distance - expected_distance).abs() <= EPS,
            "closest distance mismatch for query={query:?}"
        );
        if let Some(i) = expected_index {
            assert_eq!(result.seg_start_index, i, "closest index mismatch");
        }
    }
}

fn cpp_ccw_half_circle_x_aligned_open() -> Polyline<f64> {
    pline_open![(-4.0, 1.0, 1.0), (6.0, 1.0, 0.0)]
}

fn cpp_ccw_half_circle_x_aligned_closed() -> Polyline<f64> {
    pline_closed![(-4.0, 1.0, 1.0), (6.0, 1.0, 0.0)]
}

#[test]
fn cpp_generated_half_circle_matrix_subset_parity() {
    // These expectations follow the generated formulas used in old C++
    // TEST_cavc_pline_function.cpp::addHalfCircleCases for radius=5, center=(1,1),
    // direction=+1, x-aligned.
    let open = cpp_ccw_half_circle_x_aligned_open();
    let closed = cpp_ccw_half_circle_x_aligned_closed();

    let expected_open_length = std::f64::consts::PI * 5.0;
    let expected_closed_length = std::f64::consts::PI * 5.0 + 10.0;
    let expected_closed_area = std::f64::consts::PI * 25.0 / 2.0;

    assert!(open.area().abs() <= EPS);
    assert!((closed.area() - expected_closed_area).abs() <= EPS);
    assert!((open.path_length() - expected_open_length).abs() <= EPS);
    assert!((closed.path_length() - expected_closed_length).abs() <= EPS);

    let open_ext = open.extents().unwrap();
    let closed_ext = closed.extents().unwrap();
    let expected_ext = (-4.0, -4.0, 6.0, 1.0);

    assert!((open_ext.min_x - expected_ext.0).abs() <= EPS);
    assert!((open_ext.min_y - expected_ext.1).abs() <= EPS);
    assert!((open_ext.max_x - expected_ext.2).abs() <= EPS);
    assert!((open_ext.max_y - expected_ext.3).abs() <= EPS);

    assert!((closed_ext.min_x - expected_ext.0).abs() <= EPS);
    assert!((closed_ext.min_y - expected_ext.1).abs() <= EPS);
    assert!((closed_ext.max_x - expected_ext.2).abs() <= EPS);
    assert!((closed_ext.max_y - expected_ext.3).abs() <= EPS);

    assert_eq!(open.winding_number(Vector2::new(1.0, 0.0)), 0);
    assert_eq!(closed.winding_number(Vector2::new(1.0, 0.0)), 1);
}
