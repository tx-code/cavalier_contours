use cavalier_contours::core::math::{CircleCircleIntr, Vector2, circle_circle_intr};

const EPS: f64 = 1e-8;

#[derive(Clone, Copy)]
enum ExpectedCircleCircleCase {
    NoIntersect,
    OneIntersect {
        point: Vector2<f64>,
    },
    TwoIntersects {
        point1: Vector2<f64>,
        point2: Vector2<f64>,
    },
    Coincident,
}

struct CircleCircleParityCase {
    name: &'static str,
    radius1: f64,
    center1: Vector2<f64>,
    radius2: f64,
    center2: Vector2<f64>,
    expected: ExpectedCircleCircleCase,
}

fn assert_circle_circle_case(
    actual: CircleCircleIntr<f64>,
    expected: ExpectedCircleCircleCase,
    case_name: &str,
) {
    match (actual, expected) {
        (CircleCircleIntr::NoIntersect, ExpectedCircleCircleCase::NoIntersect) => {}
        (
            CircleCircleIntr::TangentIntersect {
                point: actual_point,
            },
            ExpectedCircleCircleCase::OneIntersect {
                point: expected_point,
            },
        ) if actual_point.fuzzy_eq(expected_point) => {}
        (
            CircleCircleIntr::TwoIntersects {
                point1: actual_point1,
                point2: actual_point2,
            },
            ExpectedCircleCircleCase::TwoIntersects {
                point1: expected_point1,
                point2: expected_point2,
            },
        ) if (actual_point1.fuzzy_eq(expected_point1)
            && actual_point2.fuzzy_eq(expected_point2))
            || (actual_point1.fuzzy_eq(expected_point2)
                && actual_point2.fuzzy_eq(expected_point1)) => {}
        (CircleCircleIntr::Overlapping, ExpectedCircleCircleCase::Coincident) => {}
        _ => panic!("circle-circle C++ parity mismatch for case {case_name}: actual={actual:?}"),
    }
}

#[test]
fn cpp_circle_circle_branch_matrix_parity() {
    // Source-aligned with old C++ `intrCircle2Circle2` branch families:
    // same-center coincident/none, outside none, inside none, tangent, two-intersects,
    // and near-tangent midpoint path where `diff < 0`.
    let cases = vec![
        CircleCircleParityCase {
            name: "same_center_coincident",
            radius1: 2.0,
            center1: Vector2::new(1.0, 1.0),
            radius2: 2.0,
            center2: Vector2::new(1.0, 1.0),
            expected: ExpectedCircleCircleCase::Coincident,
        },
        CircleCircleParityCase {
            name: "same_center_different_radius_none",
            radius1: 2.0,
            center1: Vector2::new(1.0, 1.0),
            radius2: 1.0,
            center2: Vector2::new(1.0, 1.0),
            expected: ExpectedCircleCircleCase::NoIntersect,
        },
        CircleCircleParityCase {
            name: "outside_none",
            radius1: 1.0,
            center1: Vector2::new(0.0, 0.0),
            radius2: 1.0,
            center2: Vector2::new(3.0, 0.0),
            expected: ExpectedCircleCircleCase::NoIntersect,
        },
        CircleCircleParityCase {
            name: "inside_none",
            radius1: 5.0,
            center1: Vector2::new(0.0, 0.0),
            radius2: 1.0,
            center2: Vector2::new(1.0, 0.0),
            expected: ExpectedCircleCircleCase::NoIntersect,
        },
        CircleCircleParityCase {
            name: "tangent_outside",
            radius1: 1.0,
            center1: Vector2::new(0.0, 0.0),
            radius2: 1.0,
            center2: Vector2::new(2.0, 0.0),
            expected: ExpectedCircleCircleCase::OneIntersect {
                point: Vector2::new(1.0, 0.0),
            },
        },
        CircleCircleParityCase {
            name: "tangent_inside",
            radius1: 3.0,
            center1: Vector2::new(0.0, 0.0),
            radius2: 2.0,
            center2: Vector2::new(1.0, 0.0),
            expected: ExpectedCircleCircleCase::OneIntersect {
                point: Vector2::new(3.0, 0.0),
            },
        },
        CircleCircleParityCase {
            name: "two_intersects_exact",
            radius1: 5.0,
            center1: Vector2::new(0.0, 0.0),
            radius2: 3.0,
            center2: Vector2::new(4.0, 0.0),
            expected: ExpectedCircleCircleCase::TwoIntersects {
                point1: Vector2::new(4.0, -3.0),
                point2: Vector2::new(4.0, 3.0),
            },
        },
        CircleCircleParityCase {
            name: "near_tangent_diff_negative_midpoint",
            radius1: 1.0,
            center1: Vector2::new(0.0, 0.0),
            radius2: 1.0,
            center2: Vector2::new(2.000000001, 0.0),
            expected: ExpectedCircleCircleCase::OneIntersect {
                point: Vector2::new(1.0000000005, 0.0),
            },
        },
    ];

    for case in cases {
        let result =
            circle_circle_intr(case.radius1, case.center1, case.radius2, case.center2, EPS);
        assert_circle_circle_case(result, case.expected, case.name);
    }
}
