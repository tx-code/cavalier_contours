use cavalier_contours::core::{
    math::{LineCircleIntr, Vector2, line_circle_intr},
    traits::FuzzyEq,
};

#[derive(Clone, Copy)]
enum ExpectedLineCircleCase {
    NoIntersect,
    TangentIntersect { t0: f64 },
    TwoIntersects { t0: f64, t1: f64 },
}

struct LineCircleParityCase {
    name: &'static str,
    p0: Vector2<f64>,
    p1: Vector2<f64>,
    radius: f64,
    center: Vector2<f64>,
    expected: ExpectedLineCircleCase,
}

fn assert_line_circle_case(
    actual: LineCircleIntr<f64>,
    expected: ExpectedLineCircleCase,
    case_name: &str,
) {
    match (actual, expected) {
        (LineCircleIntr::NoIntersect, ExpectedLineCircleCase::NoIntersect) => {}
        (
            LineCircleIntr::TangentIntersect { t0: actual_t0 },
            ExpectedLineCircleCase::TangentIntersect { t0: expected_t0 },
        ) if actual_t0.fuzzy_eq(expected_t0) => {}
        (
            LineCircleIntr::TwoIntersects {
                t0: actual_t0,
                t1: actual_t1,
            },
            ExpectedLineCircleCase::TwoIntersects {
                t0: expected_t0,
                t1: expected_t1,
            },
        ) if (actual_t0.fuzzy_eq(expected_t0) && actual_t1.fuzzy_eq(expected_t1))
            || (actual_t0.fuzzy_eq(expected_t1) && actual_t1.fuzzy_eq(expected_t0)) => {}
        _ => panic!("line-circle C++ parity mismatch for case {case_name}: actual={actual:?}"),
    }
}

#[test]
fn cpp_line_circle_branch_matrix_parity() {
    // Source-aligned with old C++ `intrLineSeg2Circle2` branch families:
    // degenerate segment, tangent (discriminant ~= 0), no intersect, and two-intersect paths.
    let cases = vec![
        LineCircleParityCase {
            name: "degenerate_point_on_circle_tangent",
            p0: Vector2::new(1.0, 0.0),
            p1: Vector2::new(1.0, 0.0),
            radius: 1.0,
            center: Vector2::new(0.0, 0.0),
            expected: ExpectedLineCircleCase::TangentIntersect { t0: 0.0 },
        },
        LineCircleParityCase {
            name: "degenerate_point_off_circle_none",
            p0: Vector2::new(2.0, 0.0),
            p1: Vector2::new(2.0, 0.0),
            radius: 1.0,
            center: Vector2::new(0.0, 0.0),
            expected: ExpectedLineCircleCase::NoIntersect,
        },
        LineCircleParityCase {
            name: "tangent_inside_segment",
            p0: Vector2::new(-2.0, 1.0),
            p1: Vector2::new(2.0, 1.0),
            radius: 1.0,
            center: Vector2::new(0.0, 0.0),
            expected: ExpectedLineCircleCase::TangentIntersect { t0: 0.5 },
        },
        LineCircleParityCase {
            name: "tangent_outside_segment",
            p0: Vector2::new(2.0, 1.0),
            p1: Vector2::new(3.0, 1.0),
            radius: 1.0,
            center: Vector2::new(0.0, 0.0),
            expected: ExpectedLineCircleCase::TangentIntersect { t0: -2.0 },
        },
        LineCircleParityCase {
            name: "no_intersect_discriminant_negative",
            p0: Vector2::new(-2.0, 2.0),
            p1: Vector2::new(2.0, 2.0),
            radius: 1.0,
            center: Vector2::new(0.0, 0.0),
            expected: ExpectedLineCircleCase::NoIntersect,
        },
        LineCircleParityCase {
            name: "two_intersects_inside_horizontal",
            p0: Vector2::new(-2.0, 0.0),
            p1: Vector2::new(2.0, 0.0),
            radius: 1.0,
            center: Vector2::new(0.0, 0.0),
            expected: ExpectedLineCircleCase::TwoIntersects { t0: 0.25, t1: 0.75 },
        },
        LineCircleParityCase {
            name: "two_intersects_inside_vertical",
            p0: Vector2::new(0.0, -2.0),
            p1: Vector2::new(0.0, 2.0),
            radius: 1.0,
            center: Vector2::new(0.0, 0.0),
            expected: ExpectedLineCircleCase::TwoIntersects { t0: 0.25, t1: 0.75 },
        },
        LineCircleParityCase {
            name: "two_intersects_outside_segment_b_positive",
            p0: Vector2::new(2.0, 0.0),
            p1: Vector2::new(3.0, 0.0),
            radius: 1.0,
            center: Vector2::new(0.0, 0.0),
            expected: ExpectedLineCircleCase::TwoIntersects { t0: -3.0, t1: -1.0 },
        },
        LineCircleParityCase {
            name: "two_intersects_outside_segment_b_negative",
            p0: Vector2::new(-3.0, 0.0),
            p1: Vector2::new(-2.0, 0.0),
            radius: 1.0,
            center: Vector2::new(0.0, 0.0),
            expected: ExpectedLineCircleCase::TwoIntersects { t0: 2.0, t1: 4.0 },
        },
        LineCircleParityCase {
            name: "two_intersects_offset_center",
            p0: Vector2::new(-5.0, 1.0),
            p1: Vector2::new(5.0, 1.0),
            radius: 2.0,
            center: Vector2::new(1.0, 1.0),
            expected: ExpectedLineCircleCase::TwoIntersects { t0: 0.4, t1: 0.8 },
        },
    ];

    for case in cases {
        let result = line_circle_intr(case.p0, case.p1, case.radius, case.center, 1e-5);
        assert_line_circle_case(result, case.expected, case.name);
    }
}
