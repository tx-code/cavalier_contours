use cavalier_contours::core::{
    math::{LineLineIntr, Vector2, line_line_intr},
    traits::FuzzyEq,
};

#[derive(Clone, Copy)]
enum ExpectedLineLineCase {
    NoIntersect,
    TrueIntersect { seg1_t: f64, seg2_t: f64 },
    FalseIntersect { seg1_t: f64, seg2_t: f64 },
    Overlapping { seg2_t0: f64, seg2_t1: f64 },
}

struct LineLineParityCase {
    name: &'static str,
    seg1_start: Vector2<f64>,
    seg1_end: Vector2<f64>,
    seg2_start: Vector2<f64>,
    seg2_end: Vector2<f64>,
    expected: ExpectedLineLineCase,
}

fn assert_line_line_case(
    actual: LineLineIntr<f64>,
    expected: ExpectedLineLineCase,
    case_name: &str,
) {
    match (actual, expected) {
        (LineLineIntr::NoIntersect, ExpectedLineLineCase::NoIntersect) => {}
        (
            LineLineIntr::TrueIntersect {
                seg1_t: actual_seg1_t,
                seg2_t: actual_seg2_t,
            },
            ExpectedLineLineCase::TrueIntersect {
                seg1_t: expected_seg1_t,
                seg2_t: expected_seg2_t,
            },
        ) if actual_seg1_t.fuzzy_eq(expected_seg1_t) && actual_seg2_t.fuzzy_eq(expected_seg2_t) => {
        }
        (
            LineLineIntr::FalseIntersect {
                seg1_t: actual_seg1_t,
                seg2_t: actual_seg2_t,
            },
            ExpectedLineLineCase::FalseIntersect {
                seg1_t: expected_seg1_t,
                seg2_t: expected_seg2_t,
            },
        ) if actual_seg1_t.fuzzy_eq(expected_seg1_t) && actual_seg2_t.fuzzy_eq(expected_seg2_t) => {
        }
        (
            LineLineIntr::Overlapping {
                seg2_t0: actual_seg2_t0,
                seg2_t1: actual_seg2_t1,
            },
            ExpectedLineLineCase::Overlapping {
                seg2_t0: expected_seg2_t0,
                seg2_t1: expected_seg2_t1,
            },
        ) if actual_seg2_t0.fuzzy_eq(expected_seg2_t0)
            && actual_seg2_t1.fuzzy_eq(expected_seg2_t1) => {}
        _ => panic!("line-line C++ parity mismatch for case {case_name}: actual={actual:?}"),
    }
}

#[test]
fn cpp_line_line_branch_matrix_parity() {
    // Source-aligned with old C++ `intrLineSeg2LineSeg2` branch families:
    // non-parallel true/false, parallel non-collinear none, point degenerates,
    // collinear touching, and clipped overlap interval.
    let cases = vec![
        LineLineParityCase {
            name: "non_parallel_true",
            seg1_start: Vector2::new(-2.0, 0.0),
            seg1_end: Vector2::new(2.0, 0.0),
            seg2_start: Vector2::new(0.0, -1.0),
            seg2_end: Vector2::new(0.0, 1.0),
            expected: ExpectedLineLineCase::TrueIntersect {
                seg1_t: 0.5,
                seg2_t: 0.5,
            },
        },
        LineLineParityCase {
            name: "non_parallel_false",
            seg1_start: Vector2::new(0.0, 0.0),
            seg1_end: Vector2::new(1.0, 0.0),
            seg2_start: Vector2::new(2.0, -1.0),
            seg2_end: Vector2::new(2.0, 1.0),
            expected: ExpectedLineLineCase::FalseIntersect {
                seg1_t: 2.0,
                seg2_t: 0.5,
            },
        },
        LineLineParityCase {
            name: "parallel_non_collinear_none",
            seg1_start: Vector2::new(0.0, 0.0),
            seg1_end: Vector2::new(1.0, 0.0),
            seg2_start: Vector2::new(0.0, 1.0),
            seg2_end: Vector2::new(1.0, 1.0),
            expected: ExpectedLineLineCase::NoIntersect,
        },
        LineLineParityCase {
            name: "both_points_same_true",
            seg1_start: Vector2::new(0.0, 0.0),
            seg1_end: Vector2::new(0.0, 0.0),
            seg2_start: Vector2::new(0.0, 0.0),
            seg2_end: Vector2::new(0.0, 0.0),
            expected: ExpectedLineLineCase::TrueIntersect {
                seg1_t: 0.0,
                seg2_t: 0.0,
            },
        },
        LineLineParityCase {
            name: "both_points_distinct_none",
            seg1_start: Vector2::new(0.0, 0.0),
            seg1_end: Vector2::new(0.0, 0.0),
            seg2_start: Vector2::new(1.0, 0.0),
            seg2_end: Vector2::new(1.0, 0.0),
            expected: ExpectedLineLineCase::NoIntersect,
        },
        LineLineParityCase {
            name: "seg1_point_on_seg2_true",
            seg1_start: Vector2::new(0.5, 0.0),
            seg1_end: Vector2::new(0.5, 0.0),
            seg2_start: Vector2::new(0.0, 0.0),
            seg2_end: Vector2::new(1.0, 0.0),
            expected: ExpectedLineLineCase::TrueIntersect {
                seg1_t: 0.0,
                seg2_t: 0.5,
            },
        },
        LineLineParityCase {
            name: "seg1_point_off_seg2_none",
            seg1_start: Vector2::new(2.0, 0.0),
            seg1_end: Vector2::new(2.0, 0.0),
            seg2_start: Vector2::new(0.0, 0.0),
            seg2_end: Vector2::new(1.0, 0.0),
            expected: ExpectedLineLineCase::NoIntersect,
        },
        LineLineParityCase {
            name: "seg2_point_on_seg1_true",
            seg1_start: Vector2::new(0.0, 0.0),
            seg1_end: Vector2::new(1.0, 0.0),
            seg2_start: Vector2::new(0.25, 0.0),
            seg2_end: Vector2::new(0.25, 0.0),
            expected: ExpectedLineLineCase::TrueIntersect {
                seg1_t: 0.25,
                seg2_t: 0.0,
            },
        },
        LineLineParityCase {
            name: "seg2_point_off_seg1_none",
            seg1_start: Vector2::new(0.0, 0.0),
            seg1_end: Vector2::new(1.0, 0.0),
            seg2_start: Vector2::new(2.0, 0.0),
            seg2_end: Vector2::new(2.0, 0.0),
            expected: ExpectedLineLineCase::NoIntersect,
        },
        LineLineParityCase {
            name: "collinear_disjoint_none",
            seg1_start: Vector2::new(2.0, 0.0),
            seg1_end: Vector2::new(3.0, 0.0),
            seg2_start: Vector2::new(0.0, 0.0),
            seg2_end: Vector2::new(1.0, 0.0),
            expected: ExpectedLineLineCase::NoIntersect,
        },
        LineLineParityCase {
            name: "collinear_touch_at_endpoint_true",
            seg1_start: Vector2::new(1.0, 0.0),
            seg1_end: Vector2::new(2.0, 0.0),
            seg2_start: Vector2::new(0.0, 0.0),
            seg2_end: Vector2::new(1.0, 0.0),
            expected: ExpectedLineLineCase::TrueIntersect {
                seg1_t: 0.0,
                seg2_t: 1.0,
            },
        },
        LineLineParityCase {
            name: "collinear_partial_overlap_clipped",
            seg1_start: Vector2::new(1.0, 0.0),
            seg1_end: Vector2::new(2.0, 0.0),
            seg2_start: Vector2::new(0.0, 0.0),
            seg2_end: Vector2::new(3.0, 0.0),
            expected: ExpectedLineLineCase::Overlapping {
                seg2_t0: 1.0 / 3.0,
                seg2_t1: 2.0 / 3.0,
            },
        },
    ];

    for case in cases {
        let result = line_line_intr(
            case.seg1_start,
            case.seg1_end,
            case.seg2_start,
            case.seg2_end,
            1e-5,
        );
        assert_line_line_case(result, case.expected, case.name);
    }
}
