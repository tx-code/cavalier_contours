mod test_utils;

use cavalier_contours::core::math::Vector2;
use cavalier_contours::polyline::{BooleanOp, PlineSource, PlineSourceMut, Polyline};
use cavalier_contours::{pline_closed, pline_closed_userdata, pline_open};
use test_utils::{PlineProperties, create_property_set, property_sets_match};

const EPS: f64 = PlineProperties::PROP_CMP_EPS;
const CLOSEST_QUERY_EPS: f64 = PlineProperties::POS_EQ_EPS;
const HALF_CIRCLE_RADIUS: f64 = 5.0;
const PROBE_DELTA: f64 = 0.01;

#[derive(Clone, Copy, Debug)]
struct HalfCircleCaseKey {
    center_x: f64,
    center_y: f64,
    direction: i32,
    is_x_aligned: bool,
    is_closed: bool,
}

#[derive(Clone, Copy, Debug)]
struct ClosestCase {
    query: Vector2<f64>,
    expected_point: Vector2<f64>,
    expected_distance: f64,
    expected_index: usize,
}

fn assert_near(actual: f64, expected: f64, context: &str) {
    assert!(
        (actual - expected).abs() <= EPS,
        "{context}: expected {expected}, got {actual}"
    );
}

fn case_label(case: HalfCircleCaseKey) -> String {
    let alignment = if case.is_x_aligned {
        "x_aligned"
    } else {
        "y_aligned"
    };
    let closure = if case.is_closed { "closed" } else { "open" };
    let direction = if case.direction > 0 { "ccw" } else { "cw" };
    format!(
        "{direction}/{alignment}/{closure}/center=({:.1},{:.1})",
        case.center_x, case.center_y
    )
}

fn build_half_circle_case(case: HalfCircleCaseKey) -> Polyline<f64> {
    let mut result = if case.is_closed {
        Polyline::new_closed()
    } else {
        Polyline::new()
    };

    let bulge = if case.direction > 0 { 1.0 } else { -1.0 };
    if case.is_x_aligned {
        result.add(case.center_x - HALF_CIRCLE_RADIUS, case.center_y, bulge);
        result.add(case.center_x + HALF_CIRCLE_RADIUS, case.center_y, 0.0);
    } else {
        result.add(case.center_x, case.center_y - HALF_CIRCLE_RADIUS, bulge);
        result.add(case.center_x, case.center_y + HALF_CIRCLE_RADIUS, 0.0);
    }

    result
}

fn expected_half_circle_extents(case: HalfCircleCaseKey) -> (f64, f64, f64, f64) {
    let mut min_x = case.center_x - HALF_CIRCLE_RADIUS;
    let mut min_y = case.center_y - HALF_CIRCLE_RADIUS;
    let mut max_x = case.center_x + HALF_CIRCLE_RADIUS;
    let mut max_y = case.center_y + HALF_CIRCLE_RADIUS;

    // Mirrors old C++ addHalfCircleCases extent adjustment.
    if case.direction > 0 {
        if case.is_x_aligned {
            max_y -= HALF_CIRCLE_RADIUS;
        } else {
            min_x += HALF_CIRCLE_RADIUS;
        }
    } else if case.is_x_aligned {
        min_y += HALF_CIRCLE_RADIUS;
    } else {
        max_x -= HALF_CIRCLE_RADIUS;
    }

    (min_x, min_y, max_x, max_y)
}

fn build_half_circle_closest_cases(case: HalfCircleCaseKey) -> Vec<ClosestCase> {
    let (min_x, min_y, max_x, max_y) = expected_half_circle_extents(case);
    let cx = case.center_x;
    let cy = case.center_y;
    let end_point_index = if case.is_closed { 1usize } else { 0usize };

    let mut result = Vec::new();

    // addClosestPointOnVertexes in old C++ source.
    if case.is_x_aligned {
        result.push(ClosestCase {
            query: Vector2::new(min_x, cy),
            expected_point: Vector2::new(min_x, cy),
            expected_distance: 0.0,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(max_x, cy),
            expected_point: Vector2::new(max_x, cy),
            expected_distance: 0.0,
            expected_index: end_point_index,
        });
    } else {
        result.push(ClosestCase {
            query: Vector2::new(cx, min_y),
            expected_point: Vector2::new(cx, min_y),
            expected_distance: 0.0,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(cx, max_y),
            expected_point: Vector2::new(cx, max_y),
            expected_distance: 0.0,
            expected_index: end_point_index,
        });
    }

    if case.is_x_aligned {
        let arc_midpoint_y = if case.direction > 0 { min_y } else { max_y };
        result.push(ClosestCase {
            query: Vector2::new(min_x - PROBE_DELTA, cy),
            expected_point: Vector2::new(min_x, cy),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(max_x + PROBE_DELTA, cy),
            expected_point: Vector2::new(max_x, cy),
            expected_distance: PROBE_DELTA,
            expected_index: end_point_index,
        });
        result.push(ClosestCase {
            query: Vector2::new(cx, arc_midpoint_y - PROBE_DELTA),
            expected_point: Vector2::new(cx, arc_midpoint_y),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(cx, arc_midpoint_y + PROBE_DELTA),
            expected_point: Vector2::new(cx, arc_midpoint_y),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        if case.is_closed {
            result.push(ClosestCase {
                query: Vector2::new(cx, cy - PROBE_DELTA),
                expected_point: Vector2::new(cx, cy),
                expected_distance: PROBE_DELTA,
                expected_index: 1,
            });
            result.push(ClosestCase {
                query: Vector2::new(cx, cy + PROBE_DELTA),
                expected_point: Vector2::new(cx, cy),
                expected_distance: PROBE_DELTA,
                expected_index: 1,
            });
        }
    } else {
        let arc_midpoint_x = if case.direction > 0 { max_x } else { min_x };
        result.push(ClosestCase {
            query: Vector2::new(cx, min_y - PROBE_DELTA),
            expected_point: Vector2::new(cx, min_y),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(cx, max_y + PROBE_DELTA),
            expected_point: Vector2::new(cx, max_y),
            expected_distance: PROBE_DELTA,
            expected_index: end_point_index,
        });
        result.push(ClosestCase {
            query: Vector2::new(arc_midpoint_x - PROBE_DELTA, cy),
            expected_point: Vector2::new(arc_midpoint_x, cy),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(arc_midpoint_x + PROBE_DELTA, cy),
            expected_point: Vector2::new(arc_midpoint_x, cy),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        if case.is_closed {
            result.push(ClosestCase {
                query: Vector2::new(cx - PROBE_DELTA, cy),
                expected_point: Vector2::new(cx, cy),
                expected_distance: PROBE_DELTA,
                expected_index: 1,
            });
            result.push(ClosestCase {
                query: Vector2::new(cx + PROBE_DELTA, cy),
                expected_point: Vector2::new(cx, cy),
                expected_distance: PROBE_DELTA,
                expected_index: 1,
            });
        }
    }

    result
}

fn half_circle_matrix_cases() -> Vec<HalfCircleCaseKey> {
    let centers = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];
    let mut result = Vec::new();

    for is_closed in [false, true] {
        for (center_x, center_y) in centers {
            for (is_x_aligned, direction) in [(true, 1), (true, -1), (false, 1), (false, -1)] {
                result.push(HalfCircleCaseKey {
                    center_x,
                    center_y,
                    direction,
                    is_x_aligned,
                    is_closed,
                });
            }
        }
    }

    result
}

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
        let result = circle.closest_point(query, CLOSEST_QUERY_EPS).unwrap();
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

#[test]
fn cpp_generated_half_circle_full_matrix_metrics_winding_parity() {
    for case in half_circle_matrix_cases() {
        let context = case_label(case);
        let pline = build_half_circle_case(case);
        let (min_x, min_y, max_x, max_y) = expected_half_circle_extents(case);

        let expected_area = if case.is_closed {
            (case.direction as f64) * std::f64::consts::PI * HALF_CIRCLE_RADIUS * HALF_CIRCLE_RADIUS
                / 2.0
        } else {
            0.0
        };
        let expected_path_length = std::f64::consts::PI * HALF_CIRCLE_RADIUS
            + if case.is_closed {
                2.0 * HALF_CIRCLE_RADIUS
            } else {
                0.0
            };

        assert_near(pline.area(), expected_area, &format!("{context}: area"));
        assert_near(
            pline.path_length(),
            expected_path_length,
            &format!("{context}: path_length"),
        );

        let ext = pline.extents().unwrap();
        assert_near(ext.min_x, min_x, &format!("{context}: extents.min_x"));
        assert_near(ext.min_y, min_y, &format!("{context}: extents.min_y"));
        assert_near(ext.max_x, max_x, &format!("{context}: extents.max_x"));
        assert_near(ext.max_y, max_y, &format!("{context}: extents.max_y"));

        let expected_inside_winding = if case.is_closed { case.direction } else { 0 };
        let outside_cases = [
            (Vector2::new(min_x - PROBE_DELTA, case.center_y), 0),
            (Vector2::new(max_x + PROBE_DELTA, case.center_y), 0),
            (Vector2::new(case.center_x, min_y - PROBE_DELTA), 0),
            (Vector2::new(case.center_x, max_y + PROBE_DELTA), 0),
        ];
        for (query, expected) in outside_cases {
            assert_eq!(
                pline.winding_number(query),
                expected,
                "{context}: winding_number outside mismatch for query={query:?}"
            );
        }

        let inside_cases = if case.is_x_aligned {
            [
                Vector2::new(case.center_x, min_y + PROBE_DELTA),
                Vector2::new(case.center_x, max_y - PROBE_DELTA),
            ]
        } else {
            [
                Vector2::new(min_x + PROBE_DELTA, case.center_y),
                Vector2::new(max_x - PROBE_DELTA, case.center_y),
            ]
        };
        for query in inside_cases {
            assert_eq!(
                pline.winding_number(query),
                expected_inside_winding,
                "{context}: winding_number inside mismatch for query={query:?}"
            );
        }
    }
}

#[test]
fn cpp_generated_half_circle_full_matrix_closest_point_strict_index_parity() {
    for case in half_circle_matrix_cases() {
        let context = case_label(case);
        let pline = build_half_circle_case(case);
        let closest_cases = build_half_circle_closest_cases(case);
        for (idx, expected) in closest_cases.iter().enumerate() {
            let result = pline
                .closest_point(expected.query, CLOSEST_QUERY_EPS)
                .unwrap();
            assert!(
                result.seg_point.fuzzy_eq_eps(expected.expected_point, EPS),
                "{context}: closest point mismatch at case #{idx} query={:?}",
                expected.query
            );
            assert_near(
                result.distance,
                expected.expected_distance,
                &format!("{context}: closest distance case #{idx}"),
            );
            assert_eq!(
                result.seg_start_index, expected.expected_index,
                "{context}: closest index mismatch at case #{idx} query={:?}",
                expected.query
            );
        }
    }
}
