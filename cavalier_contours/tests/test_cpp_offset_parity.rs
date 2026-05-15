mod test_utils;

use cavalier_contours::core::math::bulge_from_angle;
use cavalier_contours::polyline::{
    FindIntersectsOptions, PlineOffsetOptions, PlineSource, PlineSourceMut, Polyline,
};
use cavalier_contours::{pline_closed, pline_open};
use std::f64::consts::FRAC_PI_2;
use test_utils::{PlineProperties, create_property_set, property_sets_match};

const EPS: f64 = PlineProperties::POS_EQ_EPS;

#[derive(Clone)]
struct OffsetParityCase {
    name: &'static str,
    delta: f64,
    input: Polyline<f64>,
    expected: Vec<PlineProperties>,
}

fn simple_cases() -> Vec<OffsetParityCase> {
    vec![
        OffsetParityCase {
            name: "closed_rectangle_inward",
            delta: 2.0,
            input: pline_closed![
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0)
            ],
            expected: vec![PlineProperties::new(
                4,
                96.0,
                44.0,
                2.0,
                2.0,
                18.0,
                8.0,
                vec![],
            )],
        },
        OffsetParityCase {
            name: "open_rectangle_inward",
            delta: 2.0,
            input: pline_open![
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0),
                (0.0, 0.0, 0.0)
            ],
            expected: vec![PlineProperties::new(
                5,
                0.0,
                44.0,
                2.0,
                2.0,
                18.0,
                8.0,
                vec![],
            )],
        },
        OffsetParityCase {
            name: "closed_rectangle_outward",
            delta: -2.0,
            input: pline_closed![
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0)
            ],
            expected: vec![PlineProperties::new(
                8,
                332.56637061436,
                72.566370614359,
                -2.0,
                -2.0,
                22.0,
                12.0,
                vec![],
            )],
        },
        OffsetParityCase {
            name: "open_rectangle_outward",
            delta: -2.0,
            input: pline_open![
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0),
                (0.0, 0.0, 0.0)
            ],
            expected: vec![PlineProperties::new(
                8,
                0.0,
                69.424777960769,
                -2.0,
                -2.0,
                22.0,
                12.0,
                vec![],
            )],
        },
        OffsetParityCase {
            name: "closed_rectangle_coincident",
            delta: 5.0,
            input: pline_closed![
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0)
            ],
            expected: vec![PlineProperties::new(
                2,
                0.0,
                20.0,
                5.0,
                5.0,
                15.0,
                5.0,
                vec![],
            )],
        },
        OffsetParityCase {
            name: "closed_diamond_inward",
            delta: -5.0,
            input: pline_closed![
                (-10.0, 0.0, 0.0),
                (0.0, 10.0, 0.0),
                (10.0, 0.0, 0.0),
                (0.0, -10.0, 0.0)
            ],
            expected: vec![PlineProperties::new(
                4,
                -17.157287525381,
                16.568542494924,
                -2.9289321881345,
                -2.9289321881345,
                2.9289321881345,
                2.9289321881345,
                vec![],
            )],
        },
        OffsetParityCase {
            name: "open_diamond_inward",
            delta: -5.0,
            input: pline_open![
                (-10.0, 0.0, 0.0),
                (0.0, 10.0, 0.0),
                (10.0, 0.0, 0.0),
                (0.0, -10.0, 0.0),
                (-10.0, 0.0, 0.0)
            ],
            expected: vec![PlineProperties::new(
                5,
                0.0,
                16.568542494924,
                -2.9289321881345,
                -2.9289321881345,
                2.9289321881345,
                2.9289321881345,
                vec![],
            )],
        },
        OffsetParityCase {
            name: "closed_diamond_outward",
            delta: 5.0,
            input: pline_closed![
                (-10.0, 0.0, 0.0),
                (0.0, 10.0, 0.0),
                (10.0, 0.0, 0.0),
                (0.0, -10.0, 0.0)
            ],
            expected: vec![PlineProperties::new(
                8,
                -561.38252881436,
                87.984469030822,
                -15.0,
                -15.0,
                15.0,
                15.0,
                vec![],
            )],
        },
        OffsetParityCase {
            name: "open_diamond_outward",
            delta: 5.0,
            input: pline_open![
                (-10.0, 0.0, 0.0),
                (0.0, 10.0, 0.0),
                (10.0, 0.0, 0.0),
                (0.0, -10.0, 0.0),
                (-10.0, 0.0, 0.0)
            ],
            expected: vec![PlineProperties::new(
                8,
                0.0,
                80.130487396847,
                -13.535533905933,
                -15.0,
                15.0,
                15.0,
                vec![],
            )],
        },
    ]
}

fn specific_cases() -> Vec<OffsetParityCase> {
    vec![
        OffsetParityCase {
            name: "offset_arc_just_past_line1",
            delta: 0.1,
            input: pline_closed![
                (27.804688, 1.0, 0.0),
                (28.46842055794889, 0.3429054695163245, 0.0),
                (32.34577133994935, 0.9269762697003898, 0.0),
                (32.38116957207762, 1.451312562563487, 0.0),
                (31.5, 1.0, -0.31783751349740424),
                (30.79289310940682, 1.5, 0.0),
                (29.20710689059337, 1.5, -0.31783754777018053),
                (28.49999981323106, 1.00000000000007, 0.0)
            ],
            expected: vec![
                PlineProperties::new(
                    4,
                    0.094833810726263,
                    1.8213211761499,
                    31.533345690439,
                    0.90572346564886,
                    32.26949555256,
                    1.2817628453883,
                    vec![],
                ),
                PlineProperties::new(
                    6,
                    1.7197931450343,
                    7.5140262005179,
                    28.047835685678,
                    0.44926177903859,
                    31.495431966272,
                    1.4,
                    vec![],
                ),
            ],
        },
        OffsetParityCase {
            name: "intersect_ontop_first_vertex",
            delta: 0.25,
            input: pline_closed![
                (27.804688, 1.0, 0.0),
                (27.804688, 0.75, 0.0),
                (32.195313, 0.75, 0.0),
                (32.195313, 1.0, 0.0),
                (31.5, 1.0, -0.3178375134974),
                (30.792893109407, 1.5, 0.0),
                (29.207106890593, 1.5, -0.31783754777018),
                (28.499999813231, 1.0000000000001, 0.0)
            ],
            expected: vec![PlineProperties::new(
                4,
                0.36247092523069,
                3.593999211522,
                29.16143806012,
                1.0,
                30.838561906052,
                1.25,
                vec![],
            )],
        },
        OffsetParityCase {
            name: "collapsed_rectangle",
            delta: 30.0,
            input: pline_closed![
                (0.0, 0.0, 0.0),
                (120.0, 0.0, 0.0),
                (120.0, 40.0, 0.0),
                (0.0, 40.0, 0.0)
            ],
            expected: vec![],
        },
    ]
}

#[test]
fn cpp_parallel_offset_simple_matrix_parity() {
    for case in simple_cases() {
        let actual = create_property_set(&case.input.parallel_offset(case.delta), false);
        assert!(
            property_sets_match(&actual, &case.expected),
            "C++ parity mismatch for {}",
            case.name
        );
    }
}

#[test]
fn cpp_parallel_offset_specific_matrix_parity() {
    for case in specific_cases() {
        let actual = create_property_set(&case.input.parallel_offset(case.delta), false);
        assert!(
            property_sets_match(&actual, &case.expected),
            "C++ parity mismatch for {}",
            case.name
        );
    }
}

#[test]
fn cpp_parallel_offset_reversed_matrix_parity() {
    for case in simple_cases().into_iter().chain(specific_cases()) {
        let mut reversed = case.input.clone();
        reversed.invert_direction_mut();
        let delta = -case.delta;
        let expected: Vec<_> = case
            .expected
            .into_iter()
            .map(|mut p| {
                p.area = -p.area;
                p
            })
            .collect();
        let actual = create_property_set(&reversed.parallel_offset(delta), false);
        assert!(
            property_sets_match(&actual, &expected),
            "C++ reversed parity mismatch for {}",
            case.name
        );
    }
}

#[test]
fn cpp_parallel_offset_does_not_modify_input() {
    for case in simple_cases().into_iter().chain(specific_cases()) {
        let before: Vec<_> = case.input.iter_vertexes().collect();

        let _ = case.input.parallel_offset(case.delta);

        let after: Vec<_> = case.input.iter_vertexes().collect();
        assert_eq!(
            before.len(),
            after.len(),
            "{}: input vertex count changed after offset",
            case.name
        );
        for (idx, (v0, v1)) in before.iter().zip(after.iter()).enumerate() {
            assert!(
                (v0.x - v1.x).abs() <= EPS
                    && (v0.y - v1.y).abs() <= EPS
                    && (v0.bulge - v1.bulge).abs() <= EPS,
                "{}: input vertex changed at index {idx}: before={v0:?}, after={v1:?}",
                case.name
            );
        }
    }
}

#[test]
fn cpp_parallel_offset_reversed_does_not_modify_input() {
    for case in simple_cases().into_iter().chain(specific_cases()) {
        let mut reversed = case.input.clone();
        reversed.invert_direction_mut();
        let before: Vec<_> = reversed.iter_vertexes().collect();
        let delta = -case.delta;

        let _ = reversed.parallel_offset(delta);

        let after: Vec<_> = reversed.iter_vertexes().collect();
        assert_eq!(
            before.len(),
            after.len(),
            "{}: reversed input vertex count changed after offset",
            case.name
        );
        for (idx, (v0, v1)) in before.iter().zip(after.iter()).enumerate() {
            assert!(
                (v0.x - v1.x).abs() <= EPS
                    && (v0.y - v1.y).abs() <= EPS
                    && (v0.bulge - v1.bulge).abs() <= EPS,
                "{}: reversed input vertex changed at index {idx}: before={v0:?}, after={v1:?}",
                case.name
            );
        }
    }
}

#[test]
fn cpp_parallel_offset_options_does_not_modify_input() {
    for case in simple_cases().into_iter().chain(specific_cases()) {
        let input_aabb = case.input.create_approx_aabb_index();
        let options = PlineOffsetOptions {
            aabb_index: Some(&input_aabb),
            ..Default::default()
        };

        let before: Vec<_> = case.input.iter_vertexes().collect();

        let _ = case.input.parallel_offset_opt(case.delta, &options);

        let after: Vec<_> = case.input.iter_vertexes().collect();
        assert_eq!(
            before.len(),
            after.len(),
            "{}: options-path input vertex count changed after offset",
            case.name
        );
        for (idx, (v0, v1)) in before.iter().zip(after.iter()).enumerate() {
            assert!(
                (v0.x - v1.x).abs() <= EPS
                    && (v0.y - v1.y).abs() <= EPS
                    && (v0.bulge - v1.bulge).abs() <= EPS,
                "{}: options-path input vertex changed at index {idx}: before={v0:?}, after={v1:?}",
                case.name
            );
        }
    }
}

#[test]
fn cpp_parallel_offset_options_reversed_does_not_modify_input() {
    for case in simple_cases().into_iter().chain(specific_cases()) {
        let mut reversed = case.input.clone();
        reversed.invert_direction_mut();
        let input_aabb = reversed.create_approx_aabb_index();
        let options = PlineOffsetOptions {
            aabb_index: Some(&input_aabb),
            ..Default::default()
        };
        let before: Vec<_> = reversed.iter_vertexes().collect();
        let delta = -case.delta;

        let _ = reversed.parallel_offset_opt(delta, &options);

        let after: Vec<_> = reversed.iter_vertexes().collect();
        assert_eq!(
            before.len(),
            after.len(),
            "{}: reversed options-path input vertex count changed after offset",
            case.name
        );
        for (idx, (v0, v1)) in before.iter().zip(after.iter()).enumerate() {
            assert!(
                (v0.x - v1.x).abs() <= EPS
                    && (v0.y - v1.y).abs() <= EPS
                    && (v0.bulge - v1.bulge).abs() <= EPS,
                "{}: reversed options-path input vertex changed at index {idx}: before={v0:?}, after={v1:?}",
                case.name
            );
        }
    }
}

#[test]
fn cpp_collapsed_rectangle_parallel_offset_parity() {
    let input = pline_closed![
        (0.0, 0.0, 0.0),
        (120.0, 0.0, 0.0),
        (120.0, 40.0, 0.0),
        (0.0, 40.0, 0.0)
    ];
    let actual = input.parallel_offset(30.0);
    assert!(
        actual.is_empty(),
        "C++ parity mismatch for collapsed_rectangle, expected empty result"
    );
}

#[test]
fn cpp_circle_rectangle_intersection_snapshot() {
    fn assert_intersects(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
        expected: &[(usize, usize, f64, f64)],
    ) {
        assert_eq!(intersects.basic_intersects.len(), expected.len());
        for &(start_index1, start_index2, x, y) in expected {
            let matched = intersects.basic_intersects.iter().any(|intr| {
                intr.start_index1 == start_index1
                    && intr.start_index2 == start_index2
                    && (intr.point.x - x).abs() <= EPS
                    && (intr.point.y - y).abs() <= EPS
            });
            assert!(
                matched,
                "missing expected circle/rectangle intersect (start_index1={start_index1}, start_index2={start_index2}, point=({x}, {y})), actual={:?}",
                intersects.basic_intersects
            );
        }
        assert!(
            intersects.overlapping_intersects.is_empty(),
            "circle/rectangle parity snapshot expected no overlapping intersections"
        );
    }

    fn assert_points_only(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
        expected_points: &[(f64, f64)],
    ) {
        assert_eq!(intersects.basic_intersects.len(), expected_points.len());
        for &(x, y) in expected_points {
            let matched = intersects
                .basic_intersects
                .iter()
                .any(|intr| (intr.point.x - x).abs() <= EPS && (intr.point.y - y).abs() <= EPS);
            assert!(
                matched,
                "missing expected point ({x}, {y}), actual={:?}",
                intersects.basic_intersects
            );
        }
        assert!(
            intersects.overlapping_intersects.is_empty(),
            "circle/rectangle parity snapshot expected no overlapping intersections"
        );
    }

    let subject = pline_closed![(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)];
    let clip = pline_closed![
        (3.0, -10.0, 0.0),
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0)
    ];

    let expected_base: [(usize, usize, f64, f64); 4] = [
        (0usize, 1usize, 6.0, -3.898979485566356),
        (1usize, 1usize, 6.0, 5.898979485566356),
        (0usize, 3usize, 3.0, -3.58257569495584),
        (1usize, 3usize, 3.0, 5.58257569495584),
    ];
    let expected_swapped: [(usize, usize, f64, f64); 4] = [
        (1usize, 0usize, 6.0, -3.898979485566356),
        (1usize, 1usize, 6.0, 5.898979485566356),
        (3usize, 0usize, 3.0, -3.58257569495584),
        (3usize, 1usize, 3.0, 5.58257569495584),
    ];
    let expected_points: [(f64, f64); 4] = [
        (6.0, -3.898979485566356),
        (6.0, 5.898979485566356),
        (3.0, -3.58257569495584),
        (3.0, 5.58257569495584),
    ];

    let intersects = subject.find_intersects(&clip);
    assert_intersects(&intersects, &expected_base);

    let intersects_swapped = clip.find_intersects(&subject);
    assert_intersects(&intersects_swapped, &expected_swapped);

    let mut subject_reversed = subject.clone();
    subject_reversed.invert_direction_mut();
    let mut clip_reversed = clip.clone();
    clip_reversed.invert_direction_mut();
    let intersects_reversed = subject_reversed.find_intersects(&clip_reversed);
    assert_points_only(&intersects_reversed, &expected_points);

    let intersects_swapped_reversed = clip_reversed.find_intersects(&subject_reversed);
    assert_points_only(&intersects_swapped_reversed, &expected_points);
}

#[test]
fn cpp_circle_rectangle_intersection_matrix_parity() {
    let subject = pline_closed![(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)];
    let clip = pline_closed![
        (3.0, -10.0, 0.0),
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0)
    ];
    let expected_points: [(f64, f64); 4] = [
        (6.0, -3.898979485566356),
        (6.0, 5.898979485566356),
        (3.0, -3.58257569495584),
        (3.0, 5.58257569495584),
    ];

    let mut subject_reversed = subject.clone();
    subject_reversed.invert_direction_mut();
    let mut clip_reversed = clip.clone();
    clip_reversed.invert_direction_mut();

    let variants = [
        subject.find_intersects(&clip),
        subject.find_intersects(&clip_reversed),
        subject_reversed.find_intersects(&clip),
        subject_reversed.find_intersects(&clip_reversed),
        clip.find_intersects(&subject),
        clip.find_intersects(&subject_reversed),
        clip_reversed.find_intersects(&subject),
        clip_reversed.find_intersects(&subject_reversed),
    ];

    for intersects in variants {
        assert_eq!(intersects.basic_intersects.len(), expected_points.len());
        for (x, y) in expected_points {
            let matched = intersects
                .basic_intersects
                .iter()
                .any(|intr| (intr.point.x - x).abs() <= EPS && (intr.point.y - y).abs() <= EPS);
            assert!(
                matched,
                "missing expected point ({x}, {y}), actual={:?}",
                intersects.basic_intersects
            );
        }
        assert!(
            intersects.overlapping_intersects.is_empty(),
            "expected no overlapping intersections, actual={:?}",
            intersects.overlapping_intersects
        );
    }
}

#[test]
fn cpp_circle_rectangle_intersection_role_flip_symmetry_matrix_parity() {
    fn assert_role_flip_symmetry(
        ab: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
        ba: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
        expected_points: &[(f64, f64)],
    ) {
        assert_eq!(ab.basic_intersects.len(), expected_points.len());
        assert_eq!(ba.basic_intersects.len(), expected_points.len());
        assert!(
            ab.overlapping_intersects.is_empty(),
            "expected no overlap in AB, got {:?}",
            ab.overlapping_intersects
        );
        assert!(
            ba.overlapping_intersects.is_empty(),
            "expected no overlap in BA, got {:?}",
            ba.overlapping_intersects
        );

        for &(x, y) in expected_points {
            let ab_has = ab
                .basic_intersects
                .iter()
                .any(|intr| (intr.point.x - x).abs() <= EPS && (intr.point.y - y).abs() <= EPS);
            let ba_has = ba
                .basic_intersects
                .iter()
                .any(|intr| (intr.point.x - x).abs() <= EPS && (intr.point.y - y).abs() <= EPS);
            assert!(ab_has, "AB missing expected point ({x}, {y})");
            assert!(ba_has, "BA missing expected point ({x}, {y})");
        }

        for intr_ab in &ab.basic_intersects {
            let role_flip_match = ba.basic_intersects.iter().any(|intr_ba| {
                intr_ab.start_index1 == intr_ba.start_index2
                    && intr_ab.start_index2 == intr_ba.start_index1
                    && (intr_ab.point.x - intr_ba.point.x).abs() <= EPS
                    && (intr_ab.point.y - intr_ba.point.y).abs() <= EPS
            });

            assert!(
                role_flip_match,
                "missing AB->BA role-flip counterpart for intr_ab={intr_ab:?}, BA={:?}",
                ba.basic_intersects
            );
        }
    }

    let subject = pline_closed![(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)];
    let clip = pline_closed![
        (3.0, -10.0, 0.0),
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0)
    ];
    let expected_points: [(f64, f64); 4] = [
        (6.0, -3.898979485566356),
        (6.0, 5.898979485566356),
        (3.0, -3.58257569495584),
        (3.0, 5.58257569495584),
    ];

    let mut subject_reversed = subject.clone();
    subject_reversed.invert_direction_mut();
    let mut clip_reversed = clip.clone();
    clip_reversed.invert_direction_mut();

    let orientation_pairs = [
        (&subject, &clip),
        (&subject, &clip_reversed),
        (&subject_reversed, &clip),
        (&subject_reversed, &clip_reversed),
    ];

    for (lhs, rhs) in orientation_pairs {
        let ab = lhs.find_intersects(rhs);
        let ba = rhs.find_intersects(lhs);
        assert_role_flip_symmetry(&ab, &ba, &expected_points);
    }
}

#[test]
fn cpp_circle_rectangle_intersection_start_index_rotation_parity() {
    fn assert_points_only(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
        expected_points: &[(f64, f64)],
    ) {
        assert_eq!(intersects.basic_intersects.len(), expected_points.len());
        assert!(
            intersects.overlapping_intersects.is_empty(),
            "expected no overlapping intersections, actual={:?}",
            intersects.overlapping_intersects
        );

        for &(x, y) in expected_points {
            let matched = intersects
                .basic_intersects
                .iter()
                .any(|intr| (intr.point.x - x).abs() <= EPS && (intr.point.y - y).abs() <= EPS);
            assert!(
                matched,
                "missing expected point ({x}, {y}), actual={:?}",
                intersects.basic_intersects
            );
        }
    }

    fn assert_role_flip_pairs(
        ab: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
        ba: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) {
        for intr_ab in &ab.basic_intersects {
            let role_flip_match = ba.basic_intersects.iter().any(|intr_ba| {
                intr_ab.start_index1 == intr_ba.start_index2
                    && intr_ab.start_index2 == intr_ba.start_index1
                    && (intr_ab.point.x - intr_ba.point.x).abs() <= EPS
                    && (intr_ab.point.y - intr_ba.point.y).abs() <= EPS
            });
            assert!(
                role_flip_match,
                "missing AB->BA role-flip counterpart for intr_ab={intr_ab:?}, BA={:?}",
                ba.basic_intersects
            );
        }
    }

    // Same source geometry as the circle/rectangle parity matrix, but with rotated
    // closed-polyline start vertices to stress index re-parameterization invariance.
    let subject = pline_closed![(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)];
    let subject_rotated = pline_closed![(10.0, 1.0, 1.0), (0.0, 1.0, 1.0)];

    let clip = pline_closed![
        (3.0, -10.0, 0.0),
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0)
    ];
    let clip_rotated = pline_closed![
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0),
        (3.0, -10.0, 0.0)
    ];

    let expected_points = [
        (6.0, -3.898979485566356),
        (6.0, 5.898979485566356),
        (3.0, -3.58257569495584),
        (3.0, 5.58257569495584),
    ];

    let variants = [
        (&subject, &clip_rotated),
        (&subject_rotated, &clip),
        (&subject_rotated, &clip_rotated),
    ];

    for (lhs, rhs) in variants {
        let ab = lhs.find_intersects(rhs);
        let ba = rhs.find_intersects(lhs);
        assert_points_only(&ab, &expected_points);
        assert_points_only(&ba, &expected_points);
        assert_role_flip_pairs(&ab, &ba);
    }
}

#[test]
fn cpp_circle_rectangle_intersection_start_index_rotation_full_matrix_parity() {
    fn assert_points_only(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
        expected_points: &[(f64, f64)],
    ) {
        assert_eq!(intersects.basic_intersects.len(), expected_points.len());
        assert!(
            intersects.overlapping_intersects.is_empty(),
            "expected no overlapping intersections, actual={:?}",
            intersects.overlapping_intersects
        );

        for &(x, y) in expected_points {
            let matched = intersects
                .basic_intersects
                .iter()
                .any(|intr| (intr.point.x - x).abs() <= EPS && (intr.point.y - y).abs() <= EPS);
            assert!(
                matched,
                "missing expected point ({x}, {y}), actual={:?}",
                intersects.basic_intersects
            );
        }
    }

    fn assert_role_flip_pairs(
        ab: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
        ba: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) {
        for intr_ab in &ab.basic_intersects {
            let role_flip_match = ba.basic_intersects.iter().any(|intr_ba| {
                intr_ab.start_index1 == intr_ba.start_index2
                    && intr_ab.start_index2 == intr_ba.start_index1
                    && (intr_ab.point.x - intr_ba.point.x).abs() <= EPS
                    && (intr_ab.point.y - intr_ba.point.y).abs() <= EPS
            });
            assert!(
                role_flip_match,
                "missing AB->BA role-flip counterpart for intr_ab={intr_ab:?}, BA={:?}",
                ba.basic_intersects
            );
        }
    }

    fn reversed(mut pline: Polyline<f64>) -> Polyline<f64> {
        pline.invert_direction_mut();
        pline
    }

    let subject = pline_closed![(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)];
    let subject_rotated = pline_closed![(10.0, 1.0, 1.0), (0.0, 1.0, 1.0)];
    let clip = pline_closed![
        (3.0, -10.0, 0.0),
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0)
    ];
    let clip_rotated = pline_closed![
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0),
        (3.0, -10.0, 0.0)
    ];

    let subject_variants = [
        subject.clone(),
        subject_rotated.clone(),
        reversed(subject.clone()),
        reversed(subject_rotated),
    ];
    let clip_variants = [
        clip.clone(),
        clip_rotated.clone(),
        reversed(clip.clone()),
        reversed(clip_rotated),
    ];

    let expected_points = [
        (6.0, -3.898979485566356),
        (6.0, 5.898979485566356),
        (3.0, -3.58257569495584),
        (3.0, 5.58257569495584),
    ];

    for lhs in &subject_variants {
        for rhs in &clip_variants {
            let ab = lhs.find_intersects(rhs);
            let ba = rhs.find_intersects(lhs);
            assert_points_only(&ab, &expected_points);
            assert_points_only(&ba, &expected_points);
            assert_role_flip_pairs(&ab, &ba);
        }
    }
}

#[test]
fn cpp_circle_rectangle_intersection_options_full_matrix_parity() {
    fn assert_points_only(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
        expected_points: &[(f64, f64)],
    ) {
        assert_eq!(intersects.basic_intersects.len(), expected_points.len());
        assert!(
            intersects.overlapping_intersects.is_empty(),
            "expected no overlapping intersections, actual={:?}",
            intersects.overlapping_intersects
        );

        for &(x, y) in expected_points {
            let matched = intersects
                .basic_intersects
                .iter()
                .any(|intr| (intr.point.x - x).abs() <= EPS && (intr.point.y - y).abs() <= EPS);
            assert!(
                matched,
                "missing expected point ({x}, {y}), actual={:?}",
                intersects.basic_intersects
            );
        }
    }

    fn assert_role_flip_pairs(
        ab: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
        ba: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) {
        for intr_ab in &ab.basic_intersects {
            let role_flip_match = ba.basic_intersects.iter().any(|intr_ba| {
                intr_ab.start_index1 == intr_ba.start_index2
                    && intr_ab.start_index2 == intr_ba.start_index1
                    && (intr_ab.point.x - intr_ba.point.x).abs() <= EPS
                    && (intr_ab.point.y - intr_ba.point.y).abs() <= EPS
            });
            assert!(
                role_flip_match,
                "missing AB->BA role-flip counterpart for intr_ab={intr_ab:?}, BA={:?}",
                ba.basic_intersects
            );
        }
    }

    fn reversed(mut pline: Polyline<f64>) -> Polyline<f64> {
        pline.invert_direction_mut();
        pline
    }

    let subject = pline_closed![(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)];
    let subject_rotated = pline_closed![(10.0, 1.0, 1.0), (0.0, 1.0, 1.0)];
    let clip = pline_closed![
        (3.0, -10.0, 0.0),
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0)
    ];
    let clip_rotated = pline_closed![
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0),
        (3.0, -10.0, 0.0)
    ];

    let subject_variants = [
        subject.clone(),
        subject_rotated.clone(),
        reversed(subject.clone()),
        reversed(subject_rotated),
    ];
    let clip_variants = [
        clip.clone(),
        clip_rotated.clone(),
        reversed(clip.clone()),
        reversed(clip_rotated),
    ];

    let expected_points = [
        (6.0, -3.898979485566356),
        (6.0, 5.898979485566356),
        (3.0, -3.58257569495584),
        (3.0, 5.58257569495584),
    ];

    for (s_idx, lhs) in subject_variants.iter().enumerate() {
        for (c_idx, rhs) in clip_variants.iter().enumerate() {
            let lhs_before: Vec<_> = lhs.iter_vertexes().collect();
            let rhs_before: Vec<_> = rhs.iter_vertexes().collect();

            let lhs_aabb = lhs.create_approx_aabb_index();
            let rhs_aabb = rhs.create_approx_aabb_index();
            let options_ab = FindIntersectsOptions {
                pline1_aabb_index: Some(&lhs_aabb),
                pos_equal_eps: EPS,
            };
            let options_ba = FindIntersectsOptions {
                pline1_aabb_index: Some(&rhs_aabb),
                pos_equal_eps: EPS,
            };

            let ab = lhs.find_intersects_opt(rhs, &options_ab);
            let ba = rhs.find_intersects_opt(lhs, &options_ba);
            assert_points_only(&ab, &expected_points);
            assert_points_only(&ba, &expected_points);
            assert_role_flip_pairs(&ab, &ba);

            let lhs_after: Vec<_> = lhs.iter_vertexes().collect();
            let rhs_after: Vec<_> = rhs.iter_vertexes().collect();
            assert_eq!(
                lhs_after, lhs_before,
                "subject mutated in options intersection matrix variant s_idx={s_idx} c_idx={c_idx}"
            );
            assert_eq!(
                rhs_after, rhs_before,
                "clip mutated in options intersection matrix variant s_idx={s_idx} c_idx={c_idx}"
            );
        }
    }
}

#[test]
fn cpp_circle_rectangle_intersection_full_matrix_does_not_modify_input() {
    fn reversed(mut pline: Polyline<f64>) -> Polyline<f64> {
        pline.invert_direction_mut();
        pline
    }

    let subject = pline_closed![(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)];
    let subject_rotated = pline_closed![(10.0, 1.0, 1.0), (0.0, 1.0, 1.0)];
    let clip = pline_closed![
        (3.0, -10.0, 0.0),
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0)
    ];
    let clip_rotated = pline_closed![
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0),
        (3.0, -10.0, 0.0)
    ];

    let subject_variants = [
        subject.clone(),
        subject_rotated.clone(),
        reversed(subject.clone()),
        reversed(subject_rotated),
    ];
    let clip_variants = [
        clip.clone(),
        clip_rotated.clone(),
        reversed(clip.clone()),
        reversed(clip_rotated),
    ];

    for (s_idx, lhs) in subject_variants.iter().enumerate() {
        for (c_idx, rhs) in clip_variants.iter().enumerate() {
            let lhs_before: Vec<_> = lhs.iter_vertexes().collect();
            let rhs_before: Vec<_> = rhs.iter_vertexes().collect();

            let _ = lhs.find_intersects(rhs);
            let _ = rhs.find_intersects(lhs);

            let lhs_after: Vec<_> = lhs.iter_vertexes().collect();
            let rhs_after: Vec<_> = rhs.iter_vertexes().collect();
            assert_eq!(
                lhs_after, lhs_before,
                "subject mutated in intersection matrix variant s_idx={s_idx} c_idx={c_idx}"
            );
            assert_eq!(
                rhs_after, rhs_before,
                "clip mutated in intersection matrix variant s_idx={s_idx} c_idx={c_idx}"
            );
        }
    }
}

#[test]
fn cpp_overlap_and_basic_intersection_options_role_flip_parity_nonzero_open_index() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed_rotated = Polyline::new_closed();
    closed_side_reversed_rotated.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&closed_side_reversed_rotated, &options_ab);
    let ba =
        closed_side_reversed_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&closed_side_reversed_rotated);
    let default_ba = closed_side_reversed_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_overlap_endpoint_order_options_role_flip_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut pline1 = Polyline::new();
    pline1.add(0.0, 0.0, 0.0);
    pline1.add(4.0, 0.0, 0.0);

    let mut same_dir = Polyline::new();
    same_dir.add(1.0, 0.0, 0.0);
    same_dir.add(3.0, 0.0, 0.0);

    let mut opposite_dir = Polyline::new();
    opposite_dir.add(3.0, 0.0, 0.0);
    opposite_dir.add(1.0, 0.0, 0.0);

    for (label, pline2, expected_p1, expected_p2) in [
        ("same_dir", same_dir, (1.0_f64, 0.0_f64), (3.0_f64, 0.0_f64)),
        (
            "opposite_dir",
            opposite_dir,
            (3.0_f64, 0.0_f64),
            (1.0_f64, 0.0_f64),
        ),
    ] {
        let pline1_before: Vec<_> = pline1.iter_vertexes().collect();
        let pline2_before: Vec<_> = pline2.iter_vertexes().collect();

        let pline1_aabb = pline1.create_approx_aabb_index();
        let pline2_aabb = pline2.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&pline1_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&pline2_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = pline1.find_intersects(&pline2);
        let ab = pline1.find_intersects_opt(&pline2, &options_ab);
        let ba = pline2.find_intersects_opt(&pline1, &options_ba);

        assert!(
            default_ab.basic_intersects.is_empty(),
            "{label}: expected no default basic intersects, got {:?}",
            default_ab.basic_intersects
        );
        assert!(
            default_ab.overlapping_intersects.len() == 1,
            "{label}: expected one default overlap, got {:?}",
            default_ab.overlapping_intersects
        );
        assert!(
            ab.basic_intersects.is_empty(),
            "{label}: expected no options AB basic intersects, got {:?}",
            ab.basic_intersects
        );
        assert!(
            ab.overlapping_intersects.len() == 1,
            "{label}: expected one options AB overlap, got {:?}",
            ab.overlapping_intersects
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{label}: expected no options BA basic intersects, got {:?}",
            ba.basic_intersects
        );
        assert!(
            ba.overlapping_intersects.len() == 1,
            "{label}: expected one options BA overlap, got {:?}",
            ba.overlapping_intersects
        );

        let default_intr = default_ab.overlapping_intersects[0];
        let intr_ab = ab.overlapping_intersects[0];
        let intr_ba = ba.overlapping_intersects[0];

        assert_eq!(intr_ab.start_index1, intr_ba.start_index2);
        assert_eq!(intr_ab.start_index2, intr_ba.start_index1);

        assert_eq!(intr_ab.start_index1, default_intr.start_index1);
        assert_eq!(intr_ab.start_index2, default_intr.start_index2);
        assert_point_close(
            intr_ab.point1.x,
            intr_ab.point1.y,
            default_intr.point1.x,
            default_intr.point1.y,
        );
        assert_point_close(
            intr_ab.point2.x,
            intr_ab.point2.y,
            default_intr.point2.x,
            default_intr.point2.y,
        );

        assert_point_close(
            intr_ab.point1.x,
            intr_ab.point1.y,
            expected_p1.0,
            expected_p1.1,
        );
        assert_point_close(
            intr_ab.point2.x,
            intr_ab.point2.y,
            expected_p2.0,
            expected_p2.1,
        );

        let start_ab = pline2.at(intr_ab.start_index2).pos();
        let dist1_ab = (start_ab.x - intr_ab.point1.x) * (start_ab.x - intr_ab.point1.x)
            + (start_ab.y - intr_ab.point1.y) * (start_ab.y - intr_ab.point1.y);
        let dist2_ab = (start_ab.x - intr_ab.point2.x) * (start_ab.x - intr_ab.point2.x)
            + (start_ab.y - intr_ab.point2.y) * (start_ab.y - intr_ab.point2.y);
        assert!(
            dist1_ab <= dist2_ab + 1e-12,
            "{label}: expected AB overlap point1 closest to second segment start, dist1={dist1_ab}, dist2={dist2_ab}, intr={intr_ab:?}"
        );

        let start_ba = pline1.at(intr_ba.start_index2).pos();
        let dist1_ba = (start_ba.x - intr_ba.point1.x) * (start_ba.x - intr_ba.point1.x)
            + (start_ba.y - intr_ba.point1.y) * (start_ba.y - intr_ba.point1.y);
        let dist2_ba = (start_ba.x - intr_ba.point2.x) * (start_ba.x - intr_ba.point2.x)
            + (start_ba.y - intr_ba.point2.y) * (start_ba.y - intr_ba.point2.y);
        assert!(
            dist1_ba <= dist2_ba + 1e-12,
            "{label}: expected BA overlap point1 closest to second segment start, dist1={dist1_ba}, dist2={dist2_ba}, intr={intr_ba:?}"
        );

        let pline1_after: Vec<_> = pline1.iter_vertexes().collect();
        let pline2_after: Vec<_> = pline2.iter_vertexes().collect();
        assert_eq!(
            pline1_after, pline1_before,
            "{label}: pline1 mutated by options path"
        );
        assert_eq!(
            pline2_after, pline2_before,
            "{label}: pline2 mutated by options path"
        );
    }
}

#[test]
fn cpp_intersection_options_pos_equal_eps_controls_detection() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let gap = 5e-4;

    let mut near_touch_vertical = Polyline::new();
    near_touch_vertical.add(0.5, 0.0, 0.0);
    near_touch_vertical.add(0.5, 1.0 - gap, 0.0);

    let mut horizontal = Polyline::new();
    horizontal.add(0.0, 1.0, 0.0);
    horizontal.add(1.0, 1.0, 0.0);

    let lhs_before: Vec<_> = near_touch_vertical.iter_vertexes().collect();
    let rhs_before: Vec<_> = horizontal.iter_vertexes().collect();

    let lhs_aabb = near_touch_vertical.create_approx_aabb_index();
    let rhs_aabb = horizontal.create_approx_aabb_index();

    let strict_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&lhs_aabb),
        pos_equal_eps: 1e-6,
    };
    let strict_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&rhs_aabb),
        pos_equal_eps: 1e-6,
    };
    let loose_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&lhs_aabb),
        pos_equal_eps: 1e-3,
    };
    let loose_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&rhs_aabb),
        pos_equal_eps: 1e-3,
    };

    let strict_ab_result = near_touch_vertical.find_intersects_opt(&horizontal, &strict_ab);
    let strict_ba_result = horizontal.find_intersects_opt(&near_touch_vertical, &strict_ba);
    assert!(
        strict_ab_result.basic_intersects.is_empty(),
        "strict AB should have no basic intersects, got {:?}",
        strict_ab_result.basic_intersects
    );
    assert!(
        strict_ba_result.basic_intersects.is_empty(),
        "strict BA should have no basic intersects, got {:?}",
        strict_ba_result.basic_intersects
    );
    assert!(
        strict_ab_result.overlapping_intersects.is_empty(),
        "strict AB should have no overlaps, got {:?}",
        strict_ab_result.overlapping_intersects
    );
    assert!(
        strict_ba_result.overlapping_intersects.is_empty(),
        "strict BA should have no overlaps, got {:?}",
        strict_ba_result.overlapping_intersects
    );

    let loose_ab_result = near_touch_vertical.find_intersects_opt(&horizontal, &loose_ab);
    let loose_ba_result = horizontal.find_intersects_opt(&near_touch_vertical, &loose_ba);
    assert_eq!(loose_ab_result.basic_intersects.len(), 1);
    assert_eq!(loose_ba_result.basic_intersects.len(), 1);
    assert!(loose_ab_result.overlapping_intersects.is_empty());
    assert!(loose_ba_result.overlapping_intersects.is_empty());

    let intr_ab = loose_ab_result.basic_intersects[0];
    let intr_ba = loose_ba_result.basic_intersects[0];
    assert_eq!(intr_ab.start_index1, intr_ba.start_index2);
    assert_eq!(intr_ab.start_index2, intr_ba.start_index1);
    assert_point_close(intr_ab.point.x, intr_ab.point.y, 0.5, 1.0);
    assert_point_close(
        intr_ab.point.x,
        intr_ab.point.y,
        intr_ba.point.x,
        intr_ba.point.y,
    );

    let lhs_after: Vec<_> = near_touch_vertical.iter_vertexes().collect();
    let rhs_after: Vec<_> = horizontal.iter_vertexes().collect();
    assert_eq!(
        lhs_after, lhs_before,
        "near-touch vertical polyline mutated by find_intersects_opt"
    );
    assert_eq!(
        rhs_after, rhs_before,
        "horizontal polyline mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_overlap_and_basic_intersection_options_matrix_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn open_side_reversed() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_reversed_nonzero() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn closed_side_reversed() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(1.0, 3.0, 0.0);
        pline.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
        pline
    }

    fn closed_side_reversed_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 3.0, 0.0);
        pline.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
        pline.add(2.0, 0.0, 0.0);
        pline
    }

    let open_variants = [
        ("open_reversed", open_side_reversed()),
        ("open_reversed_nonzero", open_side_reversed_nonzero()),
    ];
    let closed_variants = [
        ("closed_reversed", closed_side_reversed()),
        ("closed_reversed_rotated", closed_side_reversed_rotated()),
    ];

    for (open_name, lhs) in &open_variants {
        for (closed_name, rhs) in &closed_variants {
            let lhs_before: Vec<_> = lhs.iter_vertexes().collect();
            let rhs_before: Vec<_> = rhs.iter_vertexes().collect();

            let lhs_aabb = lhs.create_approx_aabb_index();
            let rhs_aabb = rhs.create_approx_aabb_index();
            let options_ab = FindIntersectsOptions {
                pline1_aabb_index: Some(&lhs_aabb),
                pos_equal_eps: EPS,
            };
            let options_ba = FindIntersectsOptions {
                pline1_aabb_index: Some(&rhs_aabb),
                pos_equal_eps: EPS,
            };

            let ab = lhs.find_intersects_opt(rhs, &options_ab);
            let ba = rhs.find_intersects_opt(lhs, &options_ba);
            let default_ab = lhs.find_intersects(rhs);
            let default_ba = rhs.find_intersects(lhs);

            assert_eq!(
                ab.basic_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one AB basic intersect"
            );
            assert_eq!(
                ab.overlapping_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one AB overlap"
            );
            assert_eq!(
                ba.basic_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one BA basic intersect"
            );
            assert_eq!(
                ba.overlapping_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one BA overlap"
            );
            assert_eq!(
                default_ab.basic_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one default AB basic intersect"
            );
            assert_eq!(
                default_ab.overlapping_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one default AB overlap"
            );
            assert_eq!(
                default_ba.basic_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one default BA basic intersect"
            );
            assert_eq!(
                default_ba.overlapping_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one default BA overlap"
            );

            let basic_ab = ab.basic_intersects[0];
            let basic_ba = ba.basic_intersects[0];
            let default_basic_ab = default_ab.basic_intersects[0];
            let default_basic_ba = default_ba.basic_intersects[0];
            assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
            assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
            assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
            assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
            assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
            assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
            assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
            assert_point_close(
                basic_ab.point.x,
                basic_ab.point.y,
                basic_ba.point.x,
                basic_ba.point.y,
            );
            assert_point_close(
                basic_ab.point.x,
                basic_ab.point.y,
                default_basic_ab.point.x,
                default_basic_ab.point.y,
            );
            assert_point_close(
                basic_ba.point.x,
                basic_ba.point.y,
                default_basic_ba.point.x,
                default_basic_ba.point.y,
            );

            let overlap_ab = ab.overlapping_intersects[0];
            let overlap_ba = ba.overlapping_intersects[0];
            let default_overlap_ab = default_ab.overlapping_intersects[0];
            let default_overlap_ba = default_ba.overlapping_intersects[0];
            assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
            assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
            assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
            assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
            assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
            assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
            assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
            assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
            assert_point_close(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ba.point1.x,
                overlap_ba.point1.y,
            );
            assert_point_close(
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
            );
            assert_point_close(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                default_overlap_ab.point1.x,
                default_overlap_ab.point1.y,
            );
            assert_point_close(
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                default_overlap_ab.point2.x,
                default_overlap_ab.point2.y,
            );
            assert_point_close(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                default_overlap_ba.point1.x,
                default_overlap_ba.point1.y,
            );
            assert_point_close(
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                default_overlap_ba.point2.x,
                default_overlap_ba.point2.y,
            );

            if *open_name == "open_reversed_nonzero" {
                assert_ne!(basic_ab.start_index1, 0);
                assert_ne!(basic_ba.start_index2, 0);
                assert_ne!(overlap_ab.start_index1, 0);
                assert_ne!(overlap_ba.start_index2, 0);
            }

            let lhs_after: Vec<_> = lhs.iter_vertexes().collect();
            let rhs_after: Vec<_> = rhs.iter_vertexes().collect();
            assert_eq!(
                lhs_after, lhs_before,
                "{open_name}/{closed_name}: open-side input mutated by find_intersects_opt"
            );
            assert_eq!(
                rhs_after, rhs_before,
                "{open_name}/{closed_name}: closed-side input mutated by find_intersects_opt"
            );
        }
    }
}

#[test]
fn cpp_overlap_and_basic_intersection_options_normal_closed_side_matrix_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn open_side_reversed() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_reversed_nonzero() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn normal_closed_side() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 4.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn normal_closed_side_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 4.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    let open_variants = [
        ("open_reversed", open_side_reversed()),
        ("open_reversed_nonzero", open_side_reversed_nonzero()),
    ];
    let closed_variants = [
        ("normal_closed_side", normal_closed_side()),
        ("normal_closed_side_rotated", normal_closed_side_rotated()),
    ];

    for (open_name, lhs) in &open_variants {
        for (closed_name, rhs) in &closed_variants {
            let lhs_before: Vec<_> = lhs.iter_vertexes().collect();
            let rhs_before: Vec<_> = rhs.iter_vertexes().collect();

            let lhs_aabb = lhs.create_approx_aabb_index();
            let rhs_aabb = rhs.create_approx_aabb_index();
            let options_ab = FindIntersectsOptions {
                pline1_aabb_index: Some(&lhs_aabb),
                pos_equal_eps: EPS,
            };
            let options_ba = FindIntersectsOptions {
                pline1_aabb_index: Some(&rhs_aabb),
                pos_equal_eps: EPS,
            };

            let ab = lhs.find_intersects_opt(rhs, &options_ab);
            let ba = rhs.find_intersects_opt(lhs, &options_ba);
            let default_ab = lhs.find_intersects(rhs);
            let default_ba = rhs.find_intersects(lhs);

            assert_eq!(
                ab.basic_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one AB basic intersect"
            );
            assert_eq!(
                ab.overlapping_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one AB overlap"
            );
            assert_eq!(
                ba.basic_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one BA basic intersect"
            );
            assert_eq!(
                ba.overlapping_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one BA overlap"
            );
            assert_eq!(
                default_ab.basic_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one default AB basic intersect"
            );
            assert_eq!(
                default_ab.overlapping_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one default AB overlap"
            );
            assert_eq!(
                default_ba.basic_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one default BA basic intersect"
            );
            assert_eq!(
                default_ba.overlapping_intersects.len(),
                1,
                "{open_name}/{closed_name}: expected one default BA overlap"
            );

            let basic_ab = ab.basic_intersects[0];
            let basic_ba = ba.basic_intersects[0];
            let default_basic_ab = default_ab.basic_intersects[0];
            let default_basic_ba = default_ba.basic_intersects[0];
            assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
            assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
            assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
            assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
            assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
            assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
            assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
            assert_point_close(
                basic_ab.point.x,
                basic_ab.point.y,
                basic_ba.point.x,
                basic_ba.point.y,
            );
            assert_point_close(
                basic_ab.point.x,
                basic_ab.point.y,
                default_basic_ab.point.x,
                default_basic_ab.point.y,
            );
            assert_point_close(
                basic_ba.point.x,
                basic_ba.point.y,
                default_basic_ba.point.x,
                default_basic_ba.point.y,
            );

            let overlap_ab = ab.overlapping_intersects[0];
            let overlap_ba = ba.overlapping_intersects[0];
            let default_overlap_ab = default_ab.overlapping_intersects[0];
            let default_overlap_ba = default_ba.overlapping_intersects[0];
            assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
            assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
            assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
            assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
            assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
            assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
            assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
            assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
            assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 3.0, 1.0);
            assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 2.0, 0.0);
            assert_point_close(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                default_overlap_ab.point1.x,
                default_overlap_ab.point1.y,
            );
            assert_point_close(
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                default_overlap_ab.point2.x,
                default_overlap_ab.point2.y,
            );
            assert_point_close(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                default_overlap_ba.point1.x,
                default_overlap_ba.point1.y,
            );
            assert_point_close(
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                default_overlap_ba.point2.x,
                default_overlap_ba.point2.y,
            );

            if *open_name == "open_reversed_nonzero" {
                assert_ne!(basic_ab.start_index1, 0);
                assert_ne!(basic_ba.start_index2, 0);
                assert_ne!(overlap_ab.start_index1, 0);
                assert_ne!(overlap_ba.start_index2, 0);
            }

            let lhs_after: Vec<_> = lhs.iter_vertexes().collect();
            let rhs_after: Vec<_> = rhs.iter_vertexes().collect();
            assert_eq!(
                lhs_after, lhs_before,
                "{open_name}/{closed_name}: open-side input mutated by find_intersects_opt"
            );
            assert_eq!(
                rhs_after, rhs_before,
                "{open_name}/{closed_name}: closed-side input mutated by find_intersects_opt"
            );
        }
    }
}

#[test]
fn cpp_skip_intr_at_end_options_matrix_parity() {
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_start_index1: usize,
        expected_start_index2: usize,
        expected_point: (f64, f64),
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut case1_lhs_open = Polyline::new();
    case1_lhs_open.add(0.0, 0.0, 0.0);
    case1_lhs_open.add(2.0, 0.0, 0.0);
    case1_lhs_open.add(2.0, 2.0, 0.0);

    let mut case1_lhs_closed = Polyline::new_closed();
    case1_lhs_closed.add(0.0, 0.0, 0.0);
    case1_lhs_closed.add(2.0, 0.0, 0.0);
    case1_lhs_closed.add(2.0, 2.0, 0.0);

    let mut case1_rhs = Polyline::new();
    case1_rhs.add(1.75, -0.25, 0.0);
    case1_rhs.add(2.25, 0.25, 0.0);

    let mut case2_lhs = Polyline::new();
    case2_lhs.add(-0.2, 0.0, 0.0);
    case2_lhs.add(0.2, 0.0, 0.0);

    let mut case2_rhs_open = Polyline::new();
    case2_rhs_open.add(0.0, -1.0, 0.0);
    case2_rhs_open.add(0.0, 0.0, 0.0);
    case2_rhs_open.add(0.4, 0.8, 0.0);

    let mut case2_rhs_closed = Polyline::new_closed();
    case2_rhs_closed.add(0.0, -1.0, 0.0);
    case2_rhs_closed.add(0.0, 0.0, 0.0);
    case2_rhs_closed.add(0.4, 0.8, 0.0);

    let cases = [
        Case {
            name: "skip_intr_at_end_open_pline1",
            lhs: case1_lhs_open,
            rhs: case1_rhs.clone(),
            expected_start_index1: 1,
            expected_start_index2: 0,
            expected_point: (2.0, 0.0),
        },
        Case {
            name: "skip_intr_at_end_closed_pline1",
            lhs: case1_lhs_closed,
            rhs: case1_rhs,
            expected_start_index1: 1,
            expected_start_index2: 0,
            expected_point: (2.0, 0.0),
        },
        Case {
            name: "skip_intr_at_end_open_pline2",
            lhs: case2_lhs.clone(),
            rhs: case2_rhs_open,
            expected_start_index1: 0,
            expected_start_index2: 1,
            expected_point: (0.0, 0.0),
        },
        Case {
            name: "skip_intr_at_end_closed_pline2",
            lhs: case2_lhs,
            rhs: case2_rhs_closed,
            expected_start_index1: 0,
            expected_start_index2: 1,
            expected_point: (0.0, 0.0),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert!(
            default_ab.overlapping_intersects.is_empty(),
            "{}: default AB expected no overlaps, got {:?}",
            case.name,
            default_ab.overlapping_intersects
        );
        assert!(
            default_ba.overlapping_intersects.is_empty(),
            "{}: default BA expected no overlaps, got {:?}",
            case.name,
            default_ba.overlapping_intersects
        );
        assert!(
            ab.overlapping_intersects.is_empty(),
            "{}: options AB expected no overlaps, got {:?}",
            case.name,
            ab.overlapping_intersects
        );
        assert!(
            ba.overlapping_intersects.is_empty(),
            "{}: options BA expected no overlaps, got {:?}",
            case.name,
            ba.overlapping_intersects
        );
        assert_eq!(
            default_ab.basic_intersects.len(),
            1,
            "{}: default AB expected one basic",
            case.name
        );
        assert_eq!(
            default_ba.basic_intersects.len(),
            1,
            "{}: default BA expected one basic",
            case.name
        );
        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: options AB expected one basic",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: options BA expected one basic",
            case.name
        );

        let default_intr_ab = default_ab.basic_intersects[0];
        let default_intr_ba = default_ba.basic_intersects[0];
        let intr_ab = ab.basic_intersects[0];
        let intr_ba = ba.basic_intersects[0];

        assert_eq!(intr_ab.start_index1, case.expected_start_index1);
        assert_eq!(intr_ab.start_index2, case.expected_start_index2);
        assert_point_close(
            intr_ab.point.x,
            intr_ab.point.y,
            case.expected_point.0,
            case.expected_point.1,
        );

        assert_eq!(intr_ab.start_index1, default_intr_ab.start_index1);
        assert_eq!(intr_ab.start_index2, default_intr_ab.start_index2);
        assert_point_close(
            intr_ab.point.x,
            intr_ab.point.y,
            default_intr_ab.point.x,
            default_intr_ab.point.y,
        );

        assert_eq!(intr_ba.start_index1, default_intr_ba.start_index1);
        assert_eq!(intr_ba.start_index2, default_intr_ba.start_index2);
        assert_point_close(
            intr_ba.point.x,
            intr_ba.point.y,
            default_intr_ba.point.x,
            default_intr_ba.point.y,
        );

        assert_eq!(intr_ab.start_index1, intr_ba.start_index2);
        assert_eq!(intr_ab.start_index2, intr_ba.start_index1);
        assert_point_close(
            intr_ab.point.x,
            intr_ab.point.y,
            intr_ba.point.x,
            intr_ba.point.y,
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_open_polyline_endpoint_touch_options_parity() {
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut end_touch_start_lhs = Polyline::new();
    end_touch_start_lhs.add(0.0, 0.0, 0.0);
    end_touch_start_lhs.add(1.0, 1.0, 0.0);

    let mut end_touch_start_rhs = Polyline::new();
    end_touch_start_rhs.add(-1.0, -1.0, 0.0);
    end_touch_start_rhs.add(0.0, 0.0, 0.0);

    let mut end_touch_start_flipped_lhs = Polyline::new();
    end_touch_start_flipped_lhs.add(-1.0, -1.0, 0.0);
    end_touch_start_flipped_lhs.add(0.0, 0.0, 0.0);

    let mut end_touch_start_flipped_rhs = Polyline::new();
    end_touch_start_flipped_rhs.add(0.0, 0.0, 0.0);
    end_touch_start_flipped_rhs.add(1.0, 1.0, 0.0);

    let mut start_touch_start_lhs = Polyline::new();
    start_touch_start_lhs.add(0.0, 0.0, 0.0);
    start_touch_start_lhs.add(1.0, 1.0, 0.0);

    let mut start_touch_start_rhs = Polyline::new();
    start_touch_start_rhs.add(0.0, 0.0, 0.0);
    start_touch_start_rhs.add(-1.0, -1.0, 0.0);

    let cases = [
        Case {
            name: "open_polylines_end_touch_start",
            lhs: end_touch_start_lhs,
            rhs: end_touch_start_rhs,
        },
        Case {
            name: "open_polylines_end_touch_start_flipped",
            lhs: end_touch_start_flipped_lhs,
            rhs: end_touch_start_flipped_rhs,
        },
        Case {
            name: "open_polylines_start_points_touch",
            lhs: start_touch_start_lhs,
            rhs: start_touch_start_rhs,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert!(
            default_ab.overlapping_intersects.is_empty(),
            "{}: default AB expected no overlaps, got {:?}",
            case.name,
            default_ab.overlapping_intersects
        );
        assert!(
            default_ba.overlapping_intersects.is_empty(),
            "{}: default BA expected no overlaps, got {:?}",
            case.name,
            default_ba.overlapping_intersects
        );
        assert!(
            ab.overlapping_intersects.is_empty(),
            "{}: options AB expected no overlaps, got {:?}",
            case.name,
            ab.overlapping_intersects
        );
        assert!(
            ba.overlapping_intersects.is_empty(),
            "{}: options BA expected no overlaps, got {:?}",
            case.name,
            ba.overlapping_intersects
        );
        assert_eq!(
            default_ab.basic_intersects.len(),
            1,
            "{}: default AB expected one basic",
            case.name
        );
        assert_eq!(
            default_ba.basic_intersects.len(),
            1,
            "{}: default BA expected one basic",
            case.name
        );
        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: options AB expected one basic",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: options BA expected one basic",
            case.name
        );

        let default_intr_ab = default_ab.basic_intersects[0];
        let default_intr_ba = default_ba.basic_intersects[0];
        let intr_ab = ab.basic_intersects[0];
        let intr_ba = ba.basic_intersects[0];

        assert_eq!(intr_ab.start_index1, 0);
        assert_eq!(intr_ab.start_index2, 0);
        assert_point_close(intr_ab.point.x, intr_ab.point.y, 0.0, 0.0);

        assert_eq!(intr_ab.start_index1, default_intr_ab.start_index1);
        assert_eq!(intr_ab.start_index2, default_intr_ab.start_index2);
        assert_eq!(intr_ba.start_index1, default_intr_ba.start_index1);
        assert_eq!(intr_ba.start_index2, default_intr_ba.start_index2);
        assert_point_close(
            intr_ab.point.x,
            intr_ab.point.y,
            default_intr_ab.point.x,
            default_intr_ab.point.y,
        );
        assert_point_close(
            intr_ba.point.x,
            intr_ba.point.y,
            default_intr_ba.point.x,
            default_intr_ba.point.y,
        );

        assert_eq!(intr_ab.start_index1, intr_ba.start_index2);
        assert_eq!(intr_ab.start_index2, intr_ba.start_index1);
        assert_point_close(
            intr_ab.point.x,
            intr_ab.point.y,
            intr_ba.point.x,
            intr_ba.point.y,
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_circle_touch_and_overlap_options_parity_matrix() {
    type Point = (f64, f64);
    type BasicExpected = (usize, usize, Point);
    type OverlapExpected = (usize, usize, Point, Point);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic: Vec<BasicExpected>,
        expected_overlap: Vec<OverlapExpected>,
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    let mut touch_lhs = Polyline::new_closed();
    touch_lhs.add(0.0, 0.0, 1.0);
    touch_lhs.add(1.0, 0.0, 1.0);

    let mut touch_rhs = Polyline::new_closed();
    touch_rhs.add(1.0, 0.0, 1.0);
    touch_rhs.add(2.0, 0.0, 1.0);

    let mut same_dir_lhs = Polyline::new_closed();
    same_dir_lhs.add(0.0, 0.0, 1.0);
    same_dir_lhs.add(1.0, 0.0, 1.0);
    let same_dir_rhs = same_dir_lhs.clone();

    let mut opposing_lhs = Polyline::new_closed();
    opposing_lhs.add(0.0, 0.0, 1.0);
    opposing_lhs.add(1.0, 0.0, 1.0);

    let mut opposing_rhs = Polyline::new_closed();
    opposing_rhs.add(0.0, 0.0, -1.0);
    opposing_rhs.add(1.0, 0.0, -1.0);

    let mut opposing_flipped_lhs = Polyline::new_closed();
    opposing_flipped_lhs.add(0.0, 0.0, -1.0);
    opposing_flipped_lhs.add(1.0, 0.0, -1.0);

    let mut opposing_flipped_rhs = Polyline::new_closed();
    opposing_flipped_rhs.add(0.0, 0.0, 1.0);
    opposing_flipped_rhs.add(1.0, 0.0, 1.0);

    let cases = [
        Case {
            name: "circles_touching",
            lhs: touch_lhs,
            rhs: touch_rhs,
            expected_basic: vec![(1, 0, (1.0, 0.0))],
            expected_overlap: vec![],
        },
        Case {
            name: "circles_overlapping_same_direction",
            lhs: same_dir_lhs,
            rhs: same_dir_rhs,
            expected_basic: vec![],
            expected_overlap: vec![
                (0, 0, (0.0, 0.0), (1.0, 0.0)),
                (1, 1, (1.0, 0.0), (0.0, 0.0)),
            ],
        },
        Case {
            name: "circles_overlapping_opposing_direction",
            lhs: opposing_lhs,
            rhs: opposing_rhs,
            expected_basic: vec![],
            expected_overlap: vec![
                (1, 0, (0.0, 0.0), (1.0, 0.0)),
                (0, 1, (1.0, 0.0), (0.0, 0.0)),
            ],
        },
        Case {
            name: "circles_overlapping_opposing_direction_flipped",
            lhs: opposing_flipped_lhs,
            rhs: opposing_flipped_rhs,
            expected_basic: vec![],
            expected_overlap: vec![
                (1, 0, (0.0, 0.0), (1.0, 0.0)),
                (0, 1, (1.0, 0.0), (0.0, 0.0)),
            ],
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            case.expected_basic.len(),
            "{}: basic count mismatch",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            case.expected_overlap.len(),
            "{}: overlap count mismatch",
            case.name
        );

        for &(idx1, idx2, (x, y)) in &case.expected_basic {
            let matched = ab.basic_intersects.iter().any(|intr| {
                intr.start_index1 == idx1
                    && intr.start_index2 == idx2
                    && (intr.point.x - x).abs() <= EPS
                    && (intr.point.y - y).abs() <= EPS
            });
            assert!(
                matched,
                "{}: missing expected basic (start_index1={}, start_index2={}, point=({}, {})), actual={:?}",
                case.name, idx1, idx2, x, y, ab.basic_intersects
            );
        }

        for &(idx1, idx2, (x1, y1), (x2, y2)) in &case.expected_overlap {
            let matched = ab.overlapping_intersects.iter().any(|intr| {
                intr.start_index1 == idx1
                    && intr.start_index2 == idx2
                    && (intr.point1.x - x1).abs() <= EPS
                    && (intr.point1.y - y1).abs() <= EPS
                    && (intr.point2.x - x2).abs() <= EPS
                    && (intr.point2.y - y2).abs() <= EPS
            });
            assert!(
                matched,
                "{}: missing expected overlap (start_index1={}, start_index2={}, point1=({}, {}), point2=({}, {})), actual={:?}",
                case.name, idx1, idx2, x1, y1, x2, y2, ab.overlapping_intersects
            );
        }

        for intr_ab in &ab.basic_intersects {
            let role_flip_match = ba.basic_intersects.iter().any(|intr_ba| {
                intr_ab.start_index1 == intr_ba.start_index2
                    && intr_ab.start_index2 == intr_ba.start_index1
                    && (intr_ab.point.x - intr_ba.point.x).abs() <= EPS
                    && (intr_ab.point.y - intr_ba.point.y).abs() <= EPS
            });
            assert!(
                role_flip_match,
                "{}: missing AB->BA basic role-flip counterpart for {:?}, BA={:?}",
                case.name, intr_ab, ba.basic_intersects
            );
        }

        for intr_ab in &ab.overlapping_intersects {
            let role_flip_match = ba.overlapping_intersects.iter().any(|intr_ba| {
                if intr_ab.start_index1 != intr_ba.start_index2
                    || intr_ab.start_index2 != intr_ba.start_index1
                {
                    return false;
                }
                let same_order = (intr_ab.point1.x - intr_ba.point1.x).abs() <= EPS
                    && (intr_ab.point1.y - intr_ba.point1.y).abs() <= EPS
                    && (intr_ab.point2.x - intr_ba.point2.x).abs() <= EPS
                    && (intr_ab.point2.y - intr_ba.point2.y).abs() <= EPS;
                let swapped_order = (intr_ab.point1.x - intr_ba.point2.x).abs() <= EPS
                    && (intr_ab.point1.y - intr_ba.point2.y).abs() <= EPS
                    && (intr_ab.point2.x - intr_ba.point1.x).abs() <= EPS
                    && (intr_ab.point2.y - intr_ba.point1.y).abs() <= EPS;
                same_order || swapped_order
            });
            assert!(
                role_flip_match,
                "{}: missing AB->BA overlap role-flip counterpart for {:?}, BA={:?}",
                case.name, intr_ab, ba.overlapping_intersects
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }

    // Sanity anchor on the touching-case expected point.
    let mut sanity_circle_a = Polyline::new_closed();
    sanity_circle_a.add(0.0, 0.0, 1.0);
    sanity_circle_a.add(1.0, 0.0, 1.0);
    let mut sanity_circle_b = Polyline::new_closed();
    sanity_circle_b.add(1.0, 0.0, 1.0);
    sanity_circle_b.add(2.0, 0.0, 1.0);
    let sanity = sanity_circle_a.find_intersects(&sanity_circle_b);
    assert_eq!(sanity.basic_intersects.len(), 1);
    assert_point_close(
        sanity.basic_intersects[0].point.x,
        sanity.basic_intersects[0].point.y,
        1.0,
        0.0,
    );
}

#[test]
fn cpp_non_circle_closed_overlap_adjacent_dedup_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_start_index1: usize,
        expected_start_index2: usize,
        expected_point_a: Point,
        expected_point_b: Point,
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 1.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 3.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 3.0, 0.0);
        pline.add(2.0, 0.0, 1.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_adjacent_dedup",
            lhs: closed_side_a(),
            rhs: closed_side_b(),
            expected_start_index1: 0,
            expected_start_index2: 0,
            expected_point_a: (2.0, 0.0),
            expected_point_b: (3.0, 1.0),
        },
        Case {
            name: "both_closed_adjacent_dedup_start_index_rotation_closed_pline2",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expected_start_index1: 0,
            expected_start_index2: 3,
            expected_point_a: (2.0, 0.0),
            expected_point_b: (3.0, 1.0),
        },
        Case {
            name: "both_closed_adjacent_dedup_start_index_rotation_closed_pline1",
            lhs: closed_side_a_rotated(),
            rhs: closed_side_b(),
            expected_start_index1: 3,
            expected_start_index2: 0,
            expected_point_a: (2.0, 0.0),
            expected_point_b: (3.0, 1.0),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basic intersects, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basic intersects, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: expected one AB overlap",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: expected one BA overlap",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];

        assert_eq!(
            overlap_ab.start_index1, case.expected_start_index1,
            "{}: unexpected AB overlap start_index1",
            case.name
        );
        assert_eq!(
            overlap_ab.start_index2, case.expected_start_index2,
            "{}: unexpected AB overlap start_index2",
            case.name
        );
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_point_a,
                case.expected_point_b,
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );

        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_point_a,
                case.expected_point_b,
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_opposing_direction_closed_overlap_adjacent_dedup_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_start_index1: usize,
        expected_start_index2: usize,
        expected_point_a: Point,
        expected_point_b: Point,
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(3.0, -2.0, 0.0);
        pline.add(1.0, -2.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(3.0, -2.0, 0.0);
        pline.add(1.0, -2.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, -1.0);
        pline.add(1.0, 1.0, 0.0);
        pline.add(1.0, 4.0, 0.0);
        pline.add(3.0, 4.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 0.0);
        pline.add(1.0, 4.0, 0.0);
        pline.add(3.0, 4.0, 0.0);
        pline.add(3.0, 1.0, -1.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_opposing_adjacent_dedup",
            lhs: closed_side_a(),
            rhs: closed_side_b(),
            expected_start_index1: 0,
            expected_start_index2: 0,
            expected_point_a: (3.0, 1.0),
            expected_point_b: (1.0, 1.0),
        },
        Case {
            name: "both_closed_opposing_adjacent_dedup_start_index_rotation_closed_pline2",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expected_start_index1: 0,
            expected_start_index2: 3,
            expected_point_a: (3.0, 1.0),
            expected_point_b: (1.0, 1.0),
        },
        Case {
            name: "both_closed_opposing_adjacent_dedup_start_index_rotation_closed_pline1",
            lhs: closed_side_a_rotated(),
            rhs: closed_side_b(),
            expected_start_index1: 3,
            expected_start_index2: 0,
            expected_point_a: (3.0, 1.0),
            expected_point_b: (1.0, 1.0),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basic intersects, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basic intersects, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: expected one AB overlap",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: expected one BA overlap",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];

        assert_eq!(
            overlap_ab.start_index1, case.expected_start_index1,
            "{}: unexpected AB overlap start_index1",
            case.name
        );
        assert_eq!(
            overlap_ab.start_index2, case.expected_start_index2,
            "{}: unexpected AB overlap start_index2",
            case.name
        );
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_point_a,
                case.expected_point_b,
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );

        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_point_a,
                case.expected_point_b,
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_overlap_endpoint_arc_adjacent_dedup_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_point_a: Point,
        expected_point_b: Point,
        expect_start_index1_nonzero: bool,
        expect_start_index2_nonzero: bool,
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(0.0, 0.0, 0.0);
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline.add(3.0, 1.0, 0.0);
        pline.add(0.0, 0.0, 0.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 0.0, 0.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(0.5, -2.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(0.5, -2.0, 0.0);
        pline.add(1.0, 0.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_arc_adjacent_dedup",
            lhs: closed_side_a(),
            rhs: closed_side_b(),
            expected_point_a: (1.0, 0.0),
            expected_point_b: (2.0, 0.0),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "both_closed_arc_adjacent_dedup_start_index_rotation_closed_pline2",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expected_point_a: (1.0, 0.0),
            expected_point_b: (2.0, 0.0),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "both_closed_arc_adjacent_dedup_start_index_rotation_closed_pline1",
            lhs: closed_side_a_rotated(),
            rhs: closed_side_b(),
            expected_point_a: (1.0, 0.0),
            expected_point_b: (2.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basic intersects, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basic intersects, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: expected one AB overlap",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: expected one BA overlap",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        let overlap_default_ab = default_ab.overlapping_intersects[0];
        let overlap_default_ba = default_ba.overlapping_intersects[0];

        assert_eq!(overlap_ab.start_index1, overlap_default_ab.start_index1);
        assert_eq!(overlap_ab.start_index2, overlap_default_ab.start_index2);
        assert_eq!(overlap_ba.start_index1, overlap_default_ba.start_index1);
        assert_eq!(overlap_ba.start_index2, overlap_default_ba.start_index2);

        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_point_a,
                case.expected_point_b,
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_point_a,
                case.expected_point_b,
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );

        if case.expect_start_index1_nonzero {
            assert_ne!(
                overlap_ab.start_index1, 0,
                "{}: expected non-zero AB overlap start_index1",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero AB overlap start_index1",
                case.name
            );
        }

        if case.expect_start_index2_nonzero {
            assert_ne!(
                overlap_ab.start_index2, 0,
                "{}: expected non-zero AB overlap start_index2",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero AB overlap start_index2",
                case.name
            );
        }

        // Role inversion keeps overlap endpoint ordering for this bounded branch.
        assert!(
            (overlap_ab.point1.x - overlap_ba.point1.x).abs() <= EPS
                && (overlap_ab.point1.y - overlap_ba.point1.y).abs() <= EPS
                && (overlap_ab.point2.x - overlap_ba.point2.x).abs() <= EPS
                && (overlap_ab.point2.y - overlap_ba.point2.y).abs() <= EPS,
            "{}: AB/BA endpoint ordering diverged: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn open_side_reversed() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic",
            lhs: closed_side_a(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline1_with_closure_basic_start_index_rotation",
            lhs: closed_side_a_rotated(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline2_with_closure_basic",
            lhs: open_side_normal(),
            rhs: closed_side_b(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline2_with_closure_basic_start_index_rotation",
            lhs: open_side_normal(),
            rhs: closed_side_b_rotated(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: expected one AB basic intersect",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: expected one AB overlap",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: expected one BA basic intersect",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: expected one BA overlap",
            case.name
        );

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        assert!(
            (basic_ab.point.x - case.expected_basic_point.0).abs() <= EPS
                && (basic_ab.point.y - case.expected_basic_point.1).abs() <= EPS,
            "{}: unexpected AB basic point: {:?}",
            case.name,
            basic_ab
        );
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert!(
            (basic_ab.point.x - basic_ba.point.x).abs() <= EPS
                && (basic_ab.point.y - basic_ba.point.y).abs() <= EPS,
            "{}: AB/BA basic point diverged: AB={:?}, BA={:?}",
            case.name,
            basic_ab,
            basic_ba
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b,
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b,
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_role_flip_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn open_side_reversed() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic_role_flip",
            lhs: closed_side_a(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline1_with_closure_basic_start_index_rotation_role_flip",
            lhs: closed_side_a_rotated(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline2_with_closure_basic_role_flip",
            lhs: open_side_normal(),
            rhs: closed_side_b(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline2_with_closure_basic_start_index_rotation_role_flip",
            lhs: open_side_normal(),
            rhs: closed_side_b_rotated(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: expected one AB basic intersect",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: expected one AB overlap",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: expected one BA basic intersect",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: expected one BA overlap",
            case.name
        );

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        assert!(
            (basic_ab.point.x - case.expected_basic_point.0).abs() <= EPS
                && (basic_ab.point.y - case.expected_basic_point.1).abs() <= EPS,
            "{}: unexpected AB basic point: {:?}",
            case.name,
            basic_ab
        );
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert!(
            (basic_ab.point.x - basic_ba.point.x).abs() <= EPS
                && (basic_ab.point.y - basic_ba.point.y).abs() <= EPS,
            "{}: AB/BA basic point diverged: AB={:?}, BA={:?}",
            case.name,
            basic_ab,
            basic_ba
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b,
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b,
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );
        // For reversed-endpoint-order role-flip cases, role inversion swaps overlap ordering.
        assert!(
            (overlap_ab.point1.x - overlap_ba.point2.x).abs() <= EPS
                && (overlap_ab.point1.y - overlap_ba.point2.y).abs() <= EPS
                && (overlap_ab.point2.x - overlap_ba.point1.x).abs() <= EPS
                && (overlap_ab.point2.y - overlap_ba.point1.y).abs() <= EPS,
            "{}: expected swapped overlap ordering under role inversion: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_nonzero_open_index_options_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
        open_side_on_lhs: bool,
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn open_side_reversed_nonzero() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal_nonzero() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic_nonzero_open_index",
            lhs: closed_side_a(),
            rhs: open_side_reversed_nonzero(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            open_side_on_lhs: false,
        },
        Case {
            name: "closed_pline2_with_closure_basic_nonzero_open_index",
            lhs: open_side_normal_nonzero(),
            rhs: closed_side_b(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            open_side_on_lhs: true,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert!(
            (basic_ab.point.x - case.expected_basic_point.0).abs() <= EPS
                && (basic_ab.point.y - case.expected_basic_point.1).abs() <= EPS,
            "{}: unexpected AB basic point: {:?}",
            case.name,
            basic_ab
        );
        assert!(
            (basic_ab.point.x - basic_ba.point.x).abs() <= EPS
                && (basic_ab.point.y - basic_ba.point.y).abs() <= EPS,
            "{}: AB/BA basic point diverged: AB={:?}, BA={:?}",
            case.name,
            basic_ab,
            basic_ba
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b,
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b,
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );

        if case.open_side_on_lhs {
            assert!(
                basic_ab.start_index1 > 0 && overlap_ab.start_index1 > 0,
                "{}: expected non-zero AB open-side index attribution",
                case.name
            );
            assert!(
                basic_ba.start_index2 > 0 && overlap_ba.start_index2 > 0,
                "{}: expected non-zero BA open-side index attribution",
                case.name
            );
        } else {
            assert!(
                basic_ab.start_index2 > 0 && overlap_ab.start_index2 > 0,
                "{}: expected non-zero AB open-side index attribution",
                case.name
            );
            assert!(
                basic_ba.start_index1 > 0 && overlap_ba.start_index1 > 0,
                "{}: expected non-zero BA open-side index attribution",
                case.name
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_role_flip_nonzero_open_index_options_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
        open_side_on_lhs: bool,
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn open_side_reversed_nonzero() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal_nonzero() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic_role_flip_nonzero_open_index",
            lhs: closed_side_a(),
            rhs: open_side_reversed_nonzero(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            open_side_on_lhs: false,
        },
        Case {
            name: "closed_pline2_with_closure_basic_role_flip_nonzero_open_index",
            lhs: open_side_normal_nonzero(),
            rhs: closed_side_b(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            open_side_on_lhs: true,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);
        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );
        assert_eq!(default_ab.basic_intersects.len(), 1);
        assert_eq!(default_ab.overlapping_intersects.len(), 1);
        assert_eq!(default_ba.basic_intersects.len(), 1);
        assert_eq!(default_ba.overlapping_intersects.len(), 1);

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        let default_basic_ab = default_ab.basic_intersects[0];
        let default_basic_ba = default_ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
        assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
        assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
        assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            case.expected_basic_point.0,
            case.expected_basic_point.1,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            basic_ba.point.x,
            basic_ba.point.y,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            default_basic_ab.point.x,
            default_basic_ab.point.y,
        );
        assert_point_close(
            basic_ba.point.x,
            basic_ba.point.y,
            default_basic_ba.point.x,
            default_basic_ba.point.y,
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        let default_overlap_ab = default_ab.overlapping_intersects[0];
        let default_overlap_ba = default_ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
        assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
        assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
        assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ab.point1.x,
                default_overlap_ab.point1.y,
                default_overlap_ab.point2.x,
                default_overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default AB overlap endpoints: {:?}",
            case.name,
            default_overlap_ab
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ba.point1.x,
                default_overlap_ba.point1.y,
                default_overlap_ba.point2.x,
                default_overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default BA overlap endpoints: {:?}",
            case.name,
            default_overlap_ba
        );
        // For reversed-endpoint-order role-flip cases, role inversion swaps overlap ordering.
        assert_point_close(
            overlap_ab.point1.x,
            overlap_ab.point1.y,
            overlap_ba.point2.x,
            overlap_ba.point2.y,
        );
        assert_point_close(
            overlap_ab.point2.x,
            overlap_ab.point2.y,
            overlap_ba.point1.x,
            overlap_ba.point1.y,
        );

        if case.open_side_on_lhs {
            assert!(
                basic_ab.start_index1 > 0 && overlap_ab.start_index1 > 0,
                "{}: expected non-zero AB open-side index attribution",
                case.name
            );
            assert!(
                basic_ba.start_index2 > 0 && overlap_ba.start_index2 > 0,
                "{}: expected non-zero BA open-side index attribution",
                case.name
            );
        } else {
            assert!(
                basic_ab.start_index2 > 0 && overlap_ab.start_index2 > 0,
                "{}: expected non-zero AB open-side index attribution",
                case.name
            );
            assert!(
                basic_ba.start_index1 > 0 && overlap_ba.start_index1 > 0,
                "{}: expected non-zero BA open-side index attribution",
                case.name
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_intersect_nonzero_open_index_options_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
        open_side_on_lhs: bool,
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn open_side_reversed_nonzero() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal_nonzero() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic_intersect_nonzero_open_index",
            lhs: closed_side_a(),
            rhs: open_side_reversed_nonzero(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            open_side_on_lhs: false,
        },
        Case {
            name: "closed_pline2_with_closure_basic_intersect_nonzero_open_index",
            lhs: open_side_normal_nonzero(),
            rhs: closed_side_b(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            open_side_on_lhs: true,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);
        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );
        assert_eq!(default_ab.basic_intersects.len(), 1);
        assert_eq!(default_ab.overlapping_intersects.len(), 1);
        assert_eq!(default_ba.basic_intersects.len(), 1);
        assert_eq!(default_ba.overlapping_intersects.len(), 1);

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        let default_basic_ab = default_ab.basic_intersects[0];
        let default_basic_ba = default_ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
        assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
        assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
        assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            case.expected_basic_point.0,
            case.expected_basic_point.1,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            basic_ba.point.x,
            basic_ba.point.y,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            default_basic_ab.point.x,
            default_basic_ab.point.y,
        );
        assert_point_close(
            basic_ba.point.x,
            basic_ba.point.y,
            default_basic_ba.point.x,
            default_basic_ba.point.y,
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        let default_overlap_ab = default_ab.overlapping_intersects[0];
        let default_overlap_ba = default_ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
        assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
        assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
        assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ab.point1.x,
                default_overlap_ab.point1.y,
                default_overlap_ab.point2.x,
                default_overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default AB overlap endpoints: {:?}",
            case.name,
            default_overlap_ab
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ba.point1.x,
                default_overlap_ba.point1.y,
                default_overlap_ba.point2.x,
                default_overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default BA overlap endpoints: {:?}",
            case.name,
            default_overlap_ba
        );

        if case.open_side_on_lhs {
            assert!(
                basic_ab.start_index1 > 0 && overlap_ab.start_index1 > 0,
                "{}: expected non-zero AB open-side index attribution",
                case.name
            );
            assert!(
                basic_ba.start_index2 > 0 && overlap_ba.start_index2 > 0,
                "{}: expected non-zero BA open-side index attribution",
                case.name
            );
        } else {
            assert!(
                basic_ab.start_index2 > 0 && overlap_ab.start_index2 > 0,
                "{}: expected non-zero AB open-side index attribution",
                case.name
            );
            assert!(
                basic_ba.start_index1 > 0 && overlap_ba.start_index1 > 0,
                "{}: expected non-zero BA open-side index attribution",
                case.name
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_intersect_role_flip_nonzero_open_index_options_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
        open_side_on_lhs: bool,
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn open_side_reversed_nonzero() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal_nonzero() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic_intersect_role_flip_nonzero_open_index",
            lhs: closed_side_a(),
            rhs: open_side_reversed_nonzero(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            open_side_on_lhs: false,
        },
        Case {
            name: "closed_pline2_with_closure_basic_intersect_role_flip_nonzero_open_index",
            lhs: open_side_normal_nonzero(),
            rhs: closed_side_b(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            open_side_on_lhs: true,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);
        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );
        assert_eq!(default_ab.basic_intersects.len(), 1);
        assert_eq!(default_ab.overlapping_intersects.len(), 1);
        assert_eq!(default_ba.basic_intersects.len(), 1);
        assert_eq!(default_ba.overlapping_intersects.len(), 1);

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        let default_basic_ab = default_ab.basic_intersects[0];
        let default_basic_ba = default_ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
        assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
        assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
        assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            case.expected_basic_point.0,
            case.expected_basic_point.1,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            basic_ba.point.x,
            basic_ba.point.y,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            default_basic_ab.point.x,
            default_basic_ab.point.y,
        );
        assert_point_close(
            basic_ba.point.x,
            basic_ba.point.y,
            default_basic_ba.point.x,
            default_basic_ba.point.y,
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        let default_overlap_ab = default_ab.overlapping_intersects[0];
        let default_overlap_ba = default_ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
        assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
        assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
        assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ab.point1.x,
                default_overlap_ab.point1.y,
                default_overlap_ab.point2.x,
                default_overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default AB overlap endpoints: {:?}",
            case.name,
            default_overlap_ab
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ba.point1.x,
                default_overlap_ba.point1.y,
                default_overlap_ba.point2.x,
                default_overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default BA overlap endpoints: {:?}",
            case.name,
            default_overlap_ba
        );
        // For reversed-endpoint-order role-flip cases, role inversion swaps overlap ordering.
        assert_point_close(
            overlap_ab.point1.x,
            overlap_ab.point1.y,
            overlap_ba.point2.x,
            overlap_ba.point2.y,
        );
        assert_point_close(
            overlap_ab.point2.x,
            overlap_ab.point2.y,
            overlap_ba.point1.x,
            overlap_ba.point1.y,
        );

        if case.open_side_on_lhs {
            assert!(
                basic_ab.start_index1 > 0 && overlap_ab.start_index1 > 0,
                "{}: expected non-zero AB open-side index attribution",
                case.name
            );
            assert!(
                basic_ba.start_index2 > 0 && overlap_ba.start_index2 > 0,
                "{}: expected non-zero BA open-side index attribution",
                case.name
            );
        } else {
            assert!(
                basic_ab.start_index2 > 0 && overlap_ab.start_index2 > 0,
                "{}: expected non-zero AB open-side index attribution",
                case.name
            );
            assert!(
                basic_ba.start_index1 > 0 && overlap_ba.start_index1 > 0,
                "{}: expected non-zero BA open-side index attribution",
                case.name
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_intersect_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn open_side_reversed() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic_intersect",
            lhs: closed_side_a(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline1_with_closure_basic_intersect_start_index_rotation",
            lhs: closed_side_a_rotated(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline2_with_closure_basic_intersect",
            lhs: open_side_normal(),
            rhs: closed_side_b(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline2_with_closure_basic_intersect_start_index_rotation",
            lhs: open_side_normal(),
            rhs: closed_side_b_rotated(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);
        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );
        assert_eq!(default_ab.basic_intersects.len(), 1);
        assert_eq!(default_ab.overlapping_intersects.len(), 1);
        assert_eq!(default_ba.basic_intersects.len(), 1);
        assert_eq!(default_ba.overlapping_intersects.len(), 1);

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        let default_basic_ab = default_ab.basic_intersects[0];
        let default_basic_ba = default_ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
        assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
        assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
        assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            case.expected_basic_point.0,
            case.expected_basic_point.1,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            basic_ba.point.x,
            basic_ba.point.y,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            default_basic_ab.point.x,
            default_basic_ab.point.y,
        );
        assert_point_close(
            basic_ba.point.x,
            basic_ba.point.y,
            default_basic_ba.point.x,
            default_basic_ba.point.y,
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        let default_overlap_ab = default_ab.overlapping_intersects[0];
        let default_overlap_ba = default_ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
        assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
        assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
        assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ab.point1.x,
                default_overlap_ab.point1.y,
                default_overlap_ab.point2.x,
                default_overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default AB overlap endpoints: {:?}",
            case.name,
            default_overlap_ab
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ba.point1.x,
                default_overlap_ba.point1.y,
                default_overlap_ba.point2.x,
                default_overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default BA overlap endpoints: {:?}",
            case.name,
            default_overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_intersect_role_flip_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn open_side_reversed() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic_intersect_role_flip",
            lhs: closed_side_a(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline1_with_closure_basic_intersect_start_index_rotation_role_flip",
            lhs: closed_side_a_rotated(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline2_with_closure_basic_intersect_role_flip",
            lhs: open_side_normal(),
            rhs: closed_side_b(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
        Case {
            name: "closed_pline2_with_closure_basic_intersect_start_index_rotation_role_flip",
            lhs: open_side_normal(),
            rhs: closed_side_b_rotated(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);
        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );
        assert_eq!(default_ab.basic_intersects.len(), 1);
        assert_eq!(default_ab.overlapping_intersects.len(), 1);
        assert_eq!(default_ba.basic_intersects.len(), 1);
        assert_eq!(default_ba.overlapping_intersects.len(), 1);

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        let default_basic_ab = default_ab.basic_intersects[0];
        let default_basic_ba = default_ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
        assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
        assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
        assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            case.expected_basic_point.0,
            case.expected_basic_point.1,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            basic_ba.point.x,
            basic_ba.point.y,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            default_basic_ab.point.x,
            default_basic_ab.point.y,
        );
        assert_point_close(
            basic_ba.point.x,
            basic_ba.point.y,
            default_basic_ba.point.x,
            default_basic_ba.point.y,
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        let default_overlap_ab = default_ab.overlapping_intersects[0];
        let default_overlap_ba = default_ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
        assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
        assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
        assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ab.point1.x,
                default_overlap_ab.point1.y,
                default_overlap_ab.point2.x,
                default_overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default AB overlap endpoints: {:?}",
            case.name,
            default_overlap_ab
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ba.point1.x,
                default_overlap_ba.point1.y,
                default_overlap_ba.point2.x,
                default_overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default BA overlap endpoints: {:?}",
            case.name,
            default_overlap_ba
        );
        // For reversed-endpoint-order role-flip cases, role inversion swaps overlap ordering.
        assert_point_close(
            overlap_ab.point1.x,
            overlap_ab.point1.y,
            overlap_ba.point2.x,
            overlap_ba.point2.y,
        );
        assert_point_close(
            overlap_ab.point2.x,
            overlap_ab.point2.y,
            overlap_ba.point1.x,
            overlap_ba.point1.y,
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_start_index_rotation_zero_length_lead_options_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
        closed_side_on_lhs: bool,
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn closed_side_pline1_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn open_side_reversed() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_pline2_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic_start_index_rotation_zero_length_lead",
            lhs: closed_side_pline1_rotated_zero_lead(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            closed_side_on_lhs: true,
        },
        Case {
            name: "closed_pline2_with_closure_basic_start_index_rotation_zero_length_lead",
            lhs: open_side_normal(),
            rhs: closed_side_pline2_rotated_zero_lead(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            closed_side_on_lhs: false,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);
        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );
        assert_eq!(default_ab.basic_intersects.len(), 1);
        assert_eq!(default_ab.overlapping_intersects.len(), 1);
        assert_eq!(default_ba.basic_intersects.len(), 1);
        assert_eq!(default_ba.overlapping_intersects.len(), 1);

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        let default_basic_ab = default_ab.basic_intersects[0];
        let default_basic_ba = default_ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
        assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
        assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
        assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            case.expected_basic_point.0,
            case.expected_basic_point.1,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            basic_ba.point.x,
            basic_ba.point.y,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            default_basic_ab.point.x,
            default_basic_ab.point.y,
        );
        assert_point_close(
            basic_ba.point.x,
            basic_ba.point.y,
            default_basic_ba.point.x,
            default_basic_ba.point.y,
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        let default_overlap_ab = default_ab.overlapping_intersects[0];
        let default_overlap_ba = default_ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
        assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
        assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
        assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ab.point1.x,
                default_overlap_ab.point1.y,
                default_overlap_ab.point2.x,
                default_overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default AB overlap endpoints: {:?}",
            case.name,
            default_overlap_ab
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ba.point1.x,
                default_overlap_ba.point1.y,
                default_overlap_ba.point2.x,
                default_overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default BA overlap endpoints: {:?}",
            case.name,
            default_overlap_ba
        );

        // Reversed-endpoint-order branch keeps point set but swaps AB/BA endpoint ordering.
        assert_point_close(
            overlap_ab.point1.x,
            overlap_ab.point1.y,
            overlap_ba.point2.x,
            overlap_ba.point2.y,
        );
        assert_point_close(
            overlap_ab.point2.x,
            overlap_ab.point2.y,
            overlap_ba.point1.x,
            overlap_ba.point1.y,
        );

        if case.closed_side_on_lhs {
            assert!(
                basic_ab.start_index1 > 0 && overlap_ab.start_index1 > 0,
                "{}: expected non-zero AB closed-side index attribution",
                case.name
            );
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero AB open-side overlap index",
                case.name
            );
        } else {
            assert!(
                basic_ab.start_index2 > 0 && overlap_ab.start_index2 > 0,
                "{}: expected non-zero AB closed-side index attribution",
                case.name
            );
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero AB open-side overlap index",
                case.name
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
        closed_side_on_lhs: bool,
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn closed_side_pline1_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn open_side_reversed() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_pline2_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic_start_index_rotation_zero_length_lead_role_flip",
            lhs: closed_side_pline1_rotated_zero_lead(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            closed_side_on_lhs: true,
        },
        Case {
            name: "closed_pline2_with_closure_basic_start_index_rotation_zero_length_lead_role_flip",
            lhs: open_side_normal(),
            rhs: closed_side_pline2_rotated_zero_lead(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            closed_side_on_lhs: false,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);
        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );
        assert_eq!(default_ab.basic_intersects.len(), 1);
        assert_eq!(default_ab.overlapping_intersects.len(), 1);
        assert_eq!(default_ba.basic_intersects.len(), 1);
        assert_eq!(default_ba.overlapping_intersects.len(), 1);

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        let default_basic_ab = default_ab.basic_intersects[0];
        let default_basic_ba = default_ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
        assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
        assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
        assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            case.expected_basic_point.0,
            case.expected_basic_point.1,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            basic_ba.point.x,
            basic_ba.point.y,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            default_basic_ab.point.x,
            default_basic_ab.point.y,
        );
        assert_point_close(
            basic_ba.point.x,
            basic_ba.point.y,
            default_basic_ba.point.x,
            default_basic_ba.point.y,
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        let default_overlap_ab = default_ab.overlapping_intersects[0];
        let default_overlap_ba = default_ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
        assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
        assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
        assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ab.point1.x,
                default_overlap_ab.point1.y,
                default_overlap_ab.point2.x,
                default_overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default AB overlap endpoints: {:?}",
            case.name,
            default_overlap_ab
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ba.point1.x,
                default_overlap_ba.point1.y,
                default_overlap_ba.point2.x,
                default_overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default BA overlap endpoints: {:?}",
            case.name,
            default_overlap_ba
        );

        // For reversed-endpoint-order role-flip cases, role inversion swaps overlap ordering.
        assert_point_close(
            overlap_ab.point1.x,
            overlap_ab.point1.y,
            overlap_ba.point2.x,
            overlap_ba.point2.y,
        );
        assert_point_close(
            overlap_ab.point2.x,
            overlap_ab.point2.y,
            overlap_ba.point1.x,
            overlap_ba.point1.y,
        );

        if case.closed_side_on_lhs {
            assert!(
                basic_ab.start_index1 > 0 && overlap_ab.start_index1 > 0,
                "{}: expected non-zero AB closed-side index attribution",
                case.name
            );
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero AB open-side overlap index",
                case.name
            );
        } else {
            assert!(
                basic_ab.start_index2 > 0 && overlap_ab.start_index2 > 0,
                "{}: expected non-zero AB closed-side index attribution",
                case.name
            );
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero AB open-side overlap index",
                case.name
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
        closed_side_on_lhs: bool,
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn closed_side_pline1_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn open_side_reversed() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_pline2_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic_intersect_start_index_rotation_zero_length_lead",
            lhs: closed_side_pline1_rotated_zero_lead(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            closed_side_on_lhs: true,
        },
        Case {
            name: "closed_pline2_with_closure_basic_intersect_start_index_rotation_zero_length_lead",
            lhs: open_side_normal(),
            rhs: closed_side_pline2_rotated_zero_lead(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            closed_side_on_lhs: false,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);
        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );
        assert_eq!(default_ab.basic_intersects.len(), 1);
        assert_eq!(default_ab.overlapping_intersects.len(), 1);
        assert_eq!(default_ba.basic_intersects.len(), 1);
        assert_eq!(default_ba.overlapping_intersects.len(), 1);

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        let default_basic_ab = default_ab.basic_intersects[0];
        let default_basic_ba = default_ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
        assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
        assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
        assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            case.expected_basic_point.0,
            case.expected_basic_point.1,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            basic_ba.point.x,
            basic_ba.point.y,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            default_basic_ab.point.x,
            default_basic_ab.point.y,
        );
        assert_point_close(
            basic_ba.point.x,
            basic_ba.point.y,
            default_basic_ba.point.x,
            default_basic_ba.point.y,
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        let default_overlap_ab = default_ab.overlapping_intersects[0];
        let default_overlap_ba = default_ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
        assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
        assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
        assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ab.point1.x,
                default_overlap_ab.point1.y,
                default_overlap_ab.point2.x,
                default_overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default AB overlap endpoints: {:?}",
            case.name,
            default_overlap_ab
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ba.point1.x,
                default_overlap_ba.point1.y,
                default_overlap_ba.point2.x,
                default_overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default BA overlap endpoints: {:?}",
            case.name,
            default_overlap_ba
        );

        // Reversed-endpoint-order branch keeps point set but swaps AB/BA endpoint ordering.
        assert_point_close(
            overlap_ab.point1.x,
            overlap_ab.point1.y,
            overlap_ba.point2.x,
            overlap_ba.point2.y,
        );
        assert_point_close(
            overlap_ab.point2.x,
            overlap_ab.point2.y,
            overlap_ba.point1.x,
            overlap_ba.point1.y,
        );

        if case.closed_side_on_lhs {
            assert!(
                basic_ab.start_index1 > 0 && overlap_ab.start_index1 > 0,
                "{}: expected non-zero AB closed-side index attribution",
                case.name
            );
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero AB open-side overlap index",
                case.name
            );
        } else {
            assert!(
                basic_ab.start_index2 > 0 && overlap_ab.start_index2 > 0,
                "{}: expected non-zero AB closed-side index attribution",
                case.name
            );
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero AB open-side overlap index",
                case.name
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_basic_point: Point,
        expected_overlap_point_a: Point,
        expected_overlap_point_b: Point,
        closed_side_on_lhs: bool,
    }

    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    fn endpoint_set_matches(
        ax: f64,
        ay: f64,
        bx: f64,
        by: f64,
        expected_a: Point,
        expected_b: Point,
    ) -> bool {
        let same_order = (ax - expected_a.0).abs() <= EPS
            && (ay - expected_a.1).abs() <= EPS
            && (bx - expected_b.0).abs() <= EPS
            && (by - expected_b.1).abs() <= EPS;
        let swapped_order = (ax - expected_b.0).abs() <= EPS
            && (ay - expected_b.1).abs() <= EPS
            && (bx - expected_a.0).abs() <= EPS
            && (by - expected_a.1).abs() <= EPS;
        same_order || swapped_order
    }

    fn closed_side_pline1_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn open_side_reversed() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_normal() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_pline2_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1_with_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip",
            lhs: closed_side_pline1_rotated_zero_lead(),
            rhs: open_side_reversed(),
            expected_basic_point: (2.0, -1.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            closed_side_on_lhs: true,
        },
        Case {
            name: "closed_pline2_with_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip",
            lhs: open_side_normal(),
            rhs: closed_side_pline2_rotated_zero_lead(),
            expected_basic_point: (2.0, 0.0),
            expected_overlap_point_a: (3.0, 1.0),
            expected_overlap_point_b: (2.0, 0.0),
            closed_side_on_lhs: false,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);
        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );
        assert_eq!(default_ab.basic_intersects.len(), 1);
        assert_eq!(default_ab.overlapping_intersects.len(), 1);
        assert_eq!(default_ba.basic_intersects.len(), 1);
        assert_eq!(default_ba.overlapping_intersects.len(), 1);

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        let default_basic_ab = default_ab.basic_intersects[0];
        let default_basic_ba = default_ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
        assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
        assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
        assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            case.expected_basic_point.0,
            case.expected_basic_point.1,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            basic_ba.point.x,
            basic_ba.point.y,
        );
        assert_point_close(
            basic_ab.point.x,
            basic_ab.point.y,
            default_basic_ab.point.x,
            default_basic_ab.point.y,
        );
        assert_point_close(
            basic_ba.point.x,
            basic_ba.point.y,
            default_basic_ba.point.x,
            default_basic_ba.point.y,
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        let default_overlap_ab = default_ab.overlapping_intersects[0];
        let default_overlap_ba = default_ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
        assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
        assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
        assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
        assert!(
            endpoint_set_matches(
                overlap_ab.point1.x,
                overlap_ab.point1.y,
                overlap_ab.point2.x,
                overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            endpoint_set_matches(
                overlap_ba.point1.x,
                overlap_ba.point1.y,
                overlap_ba.point2.x,
                overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected BA overlap endpoints: {:?}",
            case.name,
            overlap_ba
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ab.point1.x,
                default_overlap_ab.point1.y,
                default_overlap_ab.point2.x,
                default_overlap_ab.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default AB overlap endpoints: {:?}",
            case.name,
            default_overlap_ab
        );
        assert!(
            endpoint_set_matches(
                default_overlap_ba.point1.x,
                default_overlap_ba.point1.y,
                default_overlap_ba.point2.x,
                default_overlap_ba.point2.y,
                case.expected_overlap_point_a,
                case.expected_overlap_point_b
            ),
            "{}: unexpected default BA overlap endpoints: {:?}",
            case.name,
            default_overlap_ba
        );

        // For reversed-endpoint-order role-flip cases, role inversion swaps overlap ordering.
        assert_point_close(
            overlap_ab.point1.x,
            overlap_ab.point1.y,
            overlap_ba.point2.x,
            overlap_ba.point2.y,
        );
        assert_point_close(
            overlap_ab.point2.x,
            overlap_ab.point2.y,
            overlap_ba.point1.x,
            overlap_ba.point1.y,
        );

        if case.closed_side_on_lhs {
            assert!(
                basic_ab.start_index1 > 0 && overlap_ab.start_index1 > 0,
                "{}: expected non-zero AB closed-side index attribution",
                case.name
            );
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero AB open-side overlap index",
                case.name
            );
        } else {
            assert!(
                basic_ab.start_index2 > 0 && overlap_ab.start_index2 > 0,
                "{}: expected non-zero AB closed-side index attribution",
                case.name
            );
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero AB open-side overlap index",
                case.name
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_adjacent_line_flip_both_closed_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_start_index1_nonzero: bool,
        expect_start_index2_nonzero: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    fn has_point(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    let cases = [
        Case {
            name: "both_closed",
            lhs: closed_side_a(),
            rhs: closed_side_b(),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1",
            lhs: closed_side_a_rotated(),
            rhs: closed_side_b(),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            3,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            3,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        // Key dedup invariant for this branch: no basic at the shared overlap endpoint (3, 1).
        assert!(
            !ab.basic_intersects
                .iter()
                .any(|b| has_point((b.point.x, b.point.y), (3.0, 1.0))),
            "{}: unexpected AB basic at (3,1): {:?}",
            case.name,
            ab.basic_intersects
        );
        assert!(
            !ba.basic_intersects
                .iter()
                .any(|b| has_point((b.point.x, b.point.y), (3.0, 1.0))),
            "{}: unexpected BA basic at (3,1): {:?}",
            case.name,
            ba.basic_intersects
        );

        // Role-flip symmetry for basics.
        for basic_ab in &ab.basic_intersects {
            let has_match = ba.basic_intersects.iter().any(|basic_ba| {
                basic_ab.start_index1 == basic_ba.start_index2
                    && basic_ab.start_index2 == basic_ba.start_index1
                    && (basic_ab.point.x - basic_ba.point.x).abs() <= EPS
                    && (basic_ab.point.y - basic_ba.point.y).abs() <= EPS
            });
            assert!(
                has_match,
                "{}: missing AB->BA basic role-flip match for {:?}",
                case.name, basic_ab
            );
        }

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if case.expect_start_index1_nonzero {
            assert_ne!(
                overlap_ab.start_index1, 0,
                "{}: expected non-zero overlap start_index1",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero overlap start_index1",
                case.name
            );
        }
        if case.expect_start_index2_nonzero {
            assert_ne!(
                overlap_ab.start_index2, 0,
                "{}: expected non-zero overlap start_index2",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero overlap start_index2",
                case.name
            );
        }
        assert!(
            has_point((overlap_ab.point1.x, overlap_ab.point1.y), (3.0, 1.0))
                && has_point((overlap_ab.point2.x, overlap_ab.point2.y), (2.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // In this branch role inversion swaps overlap endpoint ordering.
        assert!(
            (overlap_ab.point1.x - overlap_ba.point2.x).abs() <= EPS
                && (overlap_ab.point1.y - overlap_ba.point2.y).abs() <= EPS
                && (overlap_ab.point2.x - overlap_ba.point1.x).abs() <= EPS
                && (overlap_ab.point2.y - overlap_ba.point1.y).abs() <= EPS,
            "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_adjacent_line_flip_both_closed_start_index_rotation_role_flip_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_start_index1_nonzero: bool,
        expect_start_index2_nonzero: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    fn has_point(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    let cases = [
        Case {
            name: "both_closed_start_index_rotation_role_flip",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_role_flip",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_role_flip",
            lhs: closed_side_a_rotated(),
            rhs: closed_side_b(),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            3,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            3,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        assert!(
            !ab.basic_intersects
                .iter()
                .any(|b| has_point((b.point.x, b.point.y), (3.0, 1.0))),
            "{}: unexpected AB basic at (3,1): {:?}",
            case.name,
            ab.basic_intersects
        );
        assert!(
            !ba.basic_intersects
                .iter()
                .any(|b| has_point((b.point.x, b.point.y), (3.0, 1.0))),
            "{}: unexpected BA basic at (3,1): {:?}",
            case.name,
            ba.basic_intersects
        );

        for basic_ab in &ab.basic_intersects {
            let has_match = ba.basic_intersects.iter().any(|basic_ba| {
                basic_ab.start_index1 == basic_ba.start_index2
                    && basic_ab.start_index2 == basic_ba.start_index1
                    && (basic_ab.point.x - basic_ba.point.x).abs() <= EPS
                    && (basic_ab.point.y - basic_ba.point.y).abs() <= EPS
            });
            assert!(
                has_match,
                "{}: missing AB->BA basic role-flip match for {:?}",
                case.name, basic_ab
            );
        }

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if case.expect_start_index1_nonzero {
            assert_ne!(
                overlap_ab.start_index1, 0,
                "{}: expected non-zero overlap start_index1",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero overlap start_index1",
                case.name
            );
        }
        if case.expect_start_index2_nonzero {
            assert_ne!(
                overlap_ab.start_index2, 0,
                "{}: expected non-zero overlap start_index2",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero overlap start_index2",
                case.name
            );
        }
        assert!(
            has_point((overlap_ab.point1.x, overlap_ab.point1.y), (3.0, 1.0))
                && has_point((overlap_ab.point2.x, overlap_ab.point2.y), (2.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // In this branch role inversion swaps overlap endpoint ordering.
        assert!(
            (overlap_ab.point1.x - overlap_ba.point2.x).abs() <= EPS
                && (overlap_ab.point1.y - overlap_ba.point2.y).abs() <= EPS
                && (overlap_ab.point2.x - overlap_ba.point1.x).abs() <= EPS
                && (overlap_ab.point2.y - overlap_ba.point1.y).abs() <= EPS,
            "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_reversed_endpoint_adjacent_line_flip_both_closed_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_start_index1_nonzero: bool,
        expect_start_index2_nonzero: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn closed_side_a_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn closed_side_b_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline
    }

    fn closed_side_b_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    fn closed_side_a_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(0.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -3.0, 0.0);
        pline
    }

    fn has_point(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    let cases = [
        Case {
            name: "both_closed_zero_length_lead_role_flip",
            lhs: closed_side_a_zero_lead(),
            rhs: closed_side_b_zero_lead(),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated_zero_lead(),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip",
            lhs: closed_side_a_rotated_zero_lead(),
            rhs: closed_side_b(),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            3,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            3,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        assert!(
            !ab.basic_intersects
                .iter()
                .any(|b| has_point((b.point.x, b.point.y), (3.0, 1.0))),
            "{}: unexpected AB basic at (3,1): {:?}",
            case.name,
            ab.basic_intersects
        );
        assert!(
            !ba.basic_intersects
                .iter()
                .any(|b| has_point((b.point.x, b.point.y), (3.0, 1.0))),
            "{}: unexpected BA basic at (3,1): {:?}",
            case.name,
            ba.basic_intersects
        );

        for basic_ab in &ab.basic_intersects {
            let has_match = ba.basic_intersects.iter().any(|basic_ba| {
                basic_ab.start_index1 == basic_ba.start_index2
                    && basic_ab.start_index2 == basic_ba.start_index1
                    && (basic_ab.point.x - basic_ba.point.x).abs() <= EPS
                    && (basic_ab.point.y - basic_ba.point.y).abs() <= EPS
            });
            assert!(
                has_match,
                "{}: missing AB->BA basic role-flip match for {:?}",
                case.name, basic_ab
            );
        }

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if case.expect_start_index1_nonzero {
            assert_ne!(
                overlap_ab.start_index1, 0,
                "{}: expected non-zero overlap start_index1",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero overlap start_index1",
                case.name
            );
        }
        if case.expect_start_index2_nonzero {
            assert_ne!(
                overlap_ab.start_index2, 0,
                "{}: expected non-zero overlap start_index2",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero overlap start_index2",
                case.name
            );
        }
        assert!(
            has_point((overlap_ab.point1.x, overlap_ab.point1.y), (3.0, 1.0))
                && has_point((overlap_ab.point2.x, overlap_ab.point2.y), (2.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            (overlap_ab.point1.x - overlap_ba.point2.x).abs() <= EPS
                && (overlap_ab.point1.y - overlap_ba.point2.y).abs() <= EPS
                && (overlap_ab.point2.x - overlap_ba.point1.x).abs() <= EPS
                && (overlap_ab.point2.y - overlap_ba.point1.y).abs() <= EPS,
            "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_arc1_reverse_dir_both_closed_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_start_index1_nonzero: bool,
        expect_start_index2_nonzero: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, -1.0);
        pline.add(1.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline.add(3.0, 1.0, -1.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 1.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(2.0, 0.0, 1.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed",
            lhs: closed_side_a(),
            rhs: closed_side_b(),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1",
            lhs: closed_side_a_rotated(),
            rhs: closed_side_b(),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        assert!(point_eq((basic_ab.point.x, basic_ab.point.y), (3.0, 1.0)));
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert!(
            point_eq(
                (basic_ab.point.x, basic_ab.point.y),
                (basic_ba.point.x, basic_ba.point.y)
            ),
            "{}: AB/BA basic point diverged: AB={:?}, BA={:?}",
            case.name,
            basic_ab,
            basic_ba
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if case.expect_start_index1_nonzero {
            assert_ne!(
                overlap_ab.start_index1, 0,
                "{}: expected non-zero overlap start_index1",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero overlap start_index1",
                case.name
            );
        }
        if case.expect_start_index2_nonzero {
            assert_ne!(
                overlap_ab.start_index2, 0,
                "{}: expected non-zero overlap start_index2",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero overlap start_index2",
                case.name
            );
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (2.0, 0.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (3.0, 1.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // For this branch role inversion swaps overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ),
            "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_arc1_reverse_dir_both_closed_start_index_rotation_zero_length_lead_role_flip_options_parity()
{
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_basic_start_index1_nonzero: Option<bool>,
        expect_basic_start_index2_nonzero: Option<bool>,
        expect_overlap_start_index1_nonzero: Option<bool>,
        expect_overlap_start_index2_nonzero: Option<bool>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(3.0, 1.0, -1.0);
        pline.add(1.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline
    }

    fn closed_side_b_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, 0.0, 1.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn closed_side_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 0.0);
        pline.add(1.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline.add(3.0, 1.0, -1.0);
        pline
    }

    fn other_closed() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 1.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn other_closed_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(2.0, 0.0, 1.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_zero_length_lead_role_flip",
            lhs: closed_side_a_zero_lead(),
            rhs: closed_side_b_zero_lead(),
            expect_basic_start_index1_nonzero: Some(true),
            expect_basic_start_index2_nonzero: Some(true),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip",
            lhs: closed_side_rotated_zero_lead(),
            rhs: other_closed(),
            expect_basic_start_index1_nonzero: Some(true),
            expect_basic_start_index2_nonzero: None,
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: None,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip",
            lhs: closed_side_a_zero_lead(),
            rhs: other_closed_rotated_zero_lead(),
            expect_basic_start_index1_nonzero: Some(true),
            expect_basic_start_index2_nonzero: Some(true),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        assert!(point_eq((basic_ab.point.x, basic_ab.point.y), (3.0, 1.0)));
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert!(
            point_eq(
                (basic_ab.point.x, basic_ab.point.y),
                (basic_ba.point.x, basic_ba.point.y)
            ),
            "{}: AB/BA basic point diverged: AB={:?}, BA={:?}",
            case.name,
            basic_ab,
            basic_ba
        );

        if let Some(expect_nonzero) = case.expect_basic_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    basic_ab.start_index1, 0,
                    "{}: expected non-zero basic start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    basic_ab.start_index1, 0,
                    "{}: expected zero basic start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_basic_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    basic_ab.start_index2, 0,
                    "{}: expected non-zero basic start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    basic_ab.start_index2, 0,
                    "{}: expected zero basic start_index2",
                    case.name
                );
            }
        }

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if let Some(expect_nonzero) = case.expect_overlap_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index1, 0,
                    "{}: expected non-zero overlap start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index1, 0,
                    "{}: expected zero overlap start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_overlap_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index2, 0,
                    "{}: expected non-zero overlap start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index2, 0,
                    "{}: expected zero overlap start_index2",
                    case.name
                );
            }
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (2.0, 0.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (3.0, 1.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // For this branch role inversion swaps overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ),
            "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_both_reverse_dir_both_closed_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_start_index1_nonzero: bool,
        expect_start_index2_nonzero: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, -1.0);
        pline.add(1.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline.add(3.0, 1.0, -1.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed",
            lhs: closed_side_a(),
            rhs: closed_side_b(),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1",
            lhs: closed_side_a_rotated(),
            rhs: closed_side_b(),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        assert!(point_eq((basic_ab.point.x, basic_ab.point.y), (3.0, 1.0)));
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert!(
            point_eq(
                (basic_ab.point.x, basic_ab.point.y),
                (basic_ba.point.x, basic_ba.point.y)
            ),
            "{}: AB/BA basic point diverged: AB={:?}, BA={:?}",
            case.name,
            basic_ab,
            basic_ba
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if case.expect_start_index1_nonzero {
            assert_ne!(
                overlap_ab.start_index1, 0,
                "{}: expected non-zero overlap start_index1",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero overlap start_index1",
                case.name
            );
        }
        if case.expect_start_index2_nonzero {
            assert_ne!(
                overlap_ab.start_index2, 0,
                "{}: expected non-zero overlap start_index2",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero overlap start_index2",
                case.name
            );
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (3.0, 1.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (2.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // For this branch role inversion keeps overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ),
            "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_both_reverse_dir_both_closed_start_index_rotation_zero_length_lead_role_flip_options_parity()
{
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_basic_start_index1_nonzero: Option<bool>,
        expect_basic_start_index2_nonzero: Option<bool>,
        expect_overlap_start_index1_nonzero: Option<bool>,
        expect_overlap_start_index2_nonzero: Option<bool>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(3.0, 1.0, -1.0);
        pline.add(1.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline
    }

    fn closed_side_b_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn closed_side_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 0.0);
        pline.add(1.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline.add(3.0, 1.0, -1.0);
        pline
    }

    fn other_closed() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn closed_side_a_for_rotated_pline2_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, -3.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline.add(3.0, 1.0, -1.0);
        pline.add(1.0, 1.0, 0.0);
        pline
    }

    fn other_closed_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_zero_length_lead_role_flip",
            lhs: closed_side_a_zero_lead(),
            rhs: closed_side_b_zero_lead(),
            expect_basic_start_index1_nonzero: Some(true),
            expect_basic_start_index2_nonzero: Some(true),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip",
            lhs: closed_side_rotated_zero_lead(),
            rhs: other_closed(),
            expect_basic_start_index1_nonzero: Some(true),
            expect_basic_start_index2_nonzero: None,
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: None,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip",
            lhs: closed_side_a_for_rotated_pline2_zero_lead(),
            rhs: other_closed_rotated_zero_lead(),
            expect_basic_start_index1_nonzero: Some(true),
            expect_basic_start_index2_nonzero: Some(true),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        assert!(point_eq((basic_ab.point.x, basic_ab.point.y), (3.0, 1.0)));
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert!(
            point_eq(
                (basic_ab.point.x, basic_ab.point.y),
                (basic_ba.point.x, basic_ba.point.y)
            ),
            "{}: AB/BA basic point diverged: AB={:?}, BA={:?}",
            case.name,
            basic_ab,
            basic_ba
        );

        if let Some(expect_nonzero) = case.expect_basic_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    basic_ab.start_index1, 0,
                    "{}: expected non-zero basic start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    basic_ab.start_index1, 0,
                    "{}: expected zero basic start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_basic_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    basic_ab.start_index2, 0,
                    "{}: expected non-zero basic start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    basic_ab.start_index2, 0,
                    "{}: expected zero basic start_index2",
                    case.name
                );
            }
        }

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if let Some(expect_nonzero) = case.expect_overlap_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index1, 0,
                    "{}: expected non-zero overlap start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index1, 0,
                    "{}: expected zero overlap start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_overlap_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index2, 0,
                    "{}: expected non-zero overlap start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index2, 0,
                    "{}: expected zero overlap start_index2",
                    case.name
                );
            }
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (3.0, 1.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (2.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // For this branch role inversion keeps overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ),
            "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_arc2_reverse_dir_both_closed_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_start_index1_nonzero: bool,
        expect_start_index2_nonzero: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed",
            lhs: closed_side_a(),
            rhs: closed_side_b(),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1",
            lhs: closed_side_a_rotated(),
            rhs: closed_side_b(),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basic intersects, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basic intersects, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if case.expect_start_index1_nonzero {
            assert_ne!(
                overlap_ab.start_index1, 0,
                "{}: expected non-zero overlap start_index1",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero overlap start_index1",
                case.name
            );
        }
        if case.expect_start_index2_nonzero {
            assert_ne!(
                overlap_ab.start_index2, 0,
                "{}: expected non-zero overlap start_index2",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero overlap start_index2",
                case.name
            );
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (3.0, 1.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (2.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // For this branch role inversion swaps overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ),
            "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_arc2_reverse_dir_both_closed_start_index_rotation_zero_length_lead_role_flip_options_parity()
{
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_overlap_start_index1_nonzero: Option<bool>,
        expect_overlap_start_index2_nonzero: Option<bool>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline
    }

    fn closed_side_b_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn closed_side_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline.add(3.0, -3.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn other_closed() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn other_closed_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, 0.0);
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_zero_length_lead_role_flip",
            lhs: closed_side_a_zero_lead(),
            rhs: closed_side_b_zero_lead(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip",
            lhs: closed_side_rotated_zero_lead(),
            rhs: other_closed(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: None,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip",
            lhs: closed_side_a_zero_lead(),
            rhs: other_closed_rotated_zero_lead(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basics, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basics, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if let Some(expect_nonzero) = case.expect_overlap_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index1, 0,
                    "{}: expected non-zero overlap start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index1, 0,
                    "{}: expected zero overlap start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_overlap_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index2, 0,
                    "{}: expected non-zero overlap start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index2, 0,
                    "{}: expected zero overlap start_index2",
                    case.name
                );
            }
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (3.0, 1.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (2.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // For this branch role inversion swaps overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ),
            "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_closed_pline1_dedup_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_overlap_point1: Point,
        expected_overlap_point2: Point,
        expect_start_index1_nonzero: bool,
        expect_role_flip_swaps_endpoint_order: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 5.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn closed_side_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 5.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn open_same_order() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 0.0, 1.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn open_reversed_order() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "same_order_closed_pline1",
            lhs: closed_side(),
            rhs: open_same_order(),
            expected_overlap_point1: (2.0, 0.0),
            expected_overlap_point2: (3.0, 1.0),
            expect_start_index1_nonzero: true,
            expect_role_flip_swaps_endpoint_order: false,
        },
        Case {
            name: "same_order_closed_pline1_start_index_rotation",
            lhs: closed_side_rotated(),
            rhs: open_same_order(),
            expected_overlap_point1: (2.0, 0.0),
            expected_overlap_point2: (3.0, 1.0),
            expect_start_index1_nonzero: true,
            expect_role_flip_swaps_endpoint_order: false,
        },
        Case {
            name: "reversed_order_closed_pline1",
            lhs: closed_side(),
            rhs: open_reversed_order(),
            expected_overlap_point1: (3.0, 1.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_role_flip_swaps_endpoint_order: true,
        },
        Case {
            name: "reversed_order_closed_pline1_start_index_rotation",
            lhs: closed_side_rotated(),
            rhs: open_reversed_order(),
            expected_overlap_point1: (3.0, 1.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_role_flip_swaps_endpoint_order: true,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basic intersects, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basic intersects, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if case.expect_start_index1_nonzero {
            assert_ne!(
                overlap_ab.start_index1, 0,
                "{}: expected non-zero overlap start_index1",
                case.name
            );
        }
        assert_eq!(
            overlap_ab.start_index2, 0,
            "{}: expected zero overlap start_index2",
            case.name
        );

        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                case.expected_overlap_point1
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                case.expected_overlap_point2
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );

        if case.expect_role_flip_swaps_endpoint_order {
            assert!(
                point_eq(
                    (overlap_ab.point1.x, overlap_ab.point1.y),
                    (overlap_ba.point2.x, overlap_ba.point2.y)
                ) && point_eq(
                    (overlap_ab.point2.x, overlap_ab.point2.y),
                    (overlap_ba.point1.x, overlap_ba.point1.y)
                ),
                "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
                case.name,
                overlap_ab,
                overlap_ba
            );
        } else {
            assert!(
                point_eq(
                    (overlap_ab.point1.x, overlap_ab.point1.y),
                    (overlap_ba.point1.x, overlap_ba.point1.y)
                ) && point_eq(
                    (overlap_ab.point2.x, overlap_ab.point2.y),
                    (overlap_ba.point2.x, overlap_ba.point2.y)
                ),
                "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
                case.name,
                overlap_ab,
                overlap_ba
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_closed_pline2_dedup_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_overlap_point1: Point,
        expected_overlap_point2: Point,
        expect_start_index2_nonzero: bool,
        expect_role_flip_swaps_endpoint_order: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn open_side() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_same_order() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 3.0, 0.0);
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline
    }

    fn closed_same_order_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 3.0, 0.0);
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn closed_reversed_order() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(6.0, -3.0, 0.0);
        pline.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
        pline
    }

    fn closed_reversed_order_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(6.0, -3.0, 0.0);
        pline.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
        pline.add(2.0, 0.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "same_order_closed_pline2",
            lhs: open_side(),
            rhs: closed_same_order(),
            expected_overlap_point1: (2.0, 0.0),
            expected_overlap_point2: (3.0, 1.0),
            expect_start_index2_nonzero: true,
            expect_role_flip_swaps_endpoint_order: false,
        },
        Case {
            name: "same_order_closed_pline2_start_index_rotation",
            lhs: open_side(),
            rhs: closed_same_order_rotated(),
            expected_overlap_point1: (2.0, 0.0),
            expected_overlap_point2: (3.0, 1.0),
            expect_start_index2_nonzero: true,
            expect_role_flip_swaps_endpoint_order: false,
        },
        Case {
            name: "reversed_order_closed_pline2",
            lhs: open_side(),
            rhs: closed_reversed_order(),
            expected_overlap_point1: (3.0, 1.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index2_nonzero: true,
            expect_role_flip_swaps_endpoint_order: true,
        },
        Case {
            name: "reversed_order_closed_pline2_start_index_rotation",
            lhs: open_side(),
            rhs: closed_reversed_order_rotated(),
            expected_overlap_point1: (3.0, 1.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index2_nonzero: true,
            expect_role_flip_swaps_endpoint_order: true,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basic intersects, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basic intersects, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert_eq!(
            overlap_ab.start_index1, 0,
            "{}: expected zero overlap start_index1",
            case.name
        );
        if case.expect_start_index2_nonzero {
            assert_ne!(
                overlap_ab.start_index2, 0,
                "{}: expected non-zero overlap start_index2",
                case.name
            );
        }

        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                case.expected_overlap_point1
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                case.expected_overlap_point2
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );

        if case.expect_role_flip_swaps_endpoint_order {
            assert!(
                point_eq(
                    (overlap_ab.point1.x, overlap_ab.point1.y),
                    (overlap_ba.point2.x, overlap_ba.point2.y)
                ) && point_eq(
                    (overlap_ab.point2.x, overlap_ab.point2.y),
                    (overlap_ba.point1.x, overlap_ba.point1.y)
                ),
                "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
                case.name,
                overlap_ab,
                overlap_ba
            );
        } else {
            assert!(
                point_eq(
                    (overlap_ab.point1.x, overlap_ab.point1.y),
                    (overlap_ba.point1.x, overlap_ba.point1.y)
                ) && point_eq(
                    (overlap_ab.point2.x, overlap_ab.point2.y),
                    (overlap_ba.point2.x, overlap_ba.point2.y)
                ),
                "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
                case.name,
                overlap_ab,
                overlap_ba
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_same_order_closed_pline2_closure_basic_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn open_side() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 0.0, 1.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn closed_side() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 4.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn closed_side_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 4.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "same_order_closed_pline2_with_closure_basic",
            lhs: open_side(),
            rhs: closed_side(),
        },
        Case {
            name: "same_order_closed_pline2_with_closure_basic_start_index_rotation",
            lhs: open_side(),
            rhs: closed_side_rotated(),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert!(
            point_eq((basic_ab.point.x, basic_ab.point.y), (2.0, 2.0)),
            "{}: unexpected AB basic point: {:?}",
            case.name,
            basic_ab
        );
        assert!(
            point_eq(
                (basic_ab.point.x, basic_ab.point.y),
                (basic_ba.point.x, basic_ba.point.y)
            ),
            "{}: AB/BA basic point diverged: AB={:?}, BA={:?}",
            case.name,
            basic_ab,
            basic_ba
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (2.0, 0.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (3.0, 1.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // For this same-order closure-edge branch, role inversion keeps endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ),
            "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_same_order_closed_pline2_nonzero_open_index_options_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_nonzero = Polyline::new();
    open_side_nonzero.add(2.0, 0.0, 0.0);
    open_side_nonzero.add(2.0, 0.0, 1.0);
    open_side_nonzero.add(2.0, 2.0, 0.0);
    open_side_nonzero.add(3.0, 1.0, 0.0);

    let mut closed_side = Polyline::new_closed();
    closed_side.add(3.0, 1.0, 0.0);
    closed_side.add(4.0, 4.0, 0.0);
    closed_side.add(1.0, 1.0, 1.0);

    let open_before: Vec<_> = open_side_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side.iter_vertexes().collect();

    let open_aabb = open_side_nonzero.create_approx_aabb_index();
    let closed_aabb = closed_side.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_nonzero.find_intersects_opt(&closed_side, &options_ab);
    let ba = closed_side.find_intersects_opt(&open_side_nonzero, &options_ba);
    let default_ab = open_side_nonzero.find_intersects(&closed_side);
    let default_ba = closed_side.find_intersects(&open_side_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert!(basic_ab.start_index1 > 0);
    assert!(basic_ba.start_index2 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert!(overlap_ab.start_index1 > 0);
    assert!(overlap_ba.start_index2 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    // For this same-order branch, role inversion keeps overlap endpoint ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_same_order_closed_pline2_closure_basic_nonzero_open_index_options_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_nonzero = Polyline::new();
    open_side_nonzero.add(2.0, 0.0, 0.0);
    open_side_nonzero.add(2.0, 0.0, 1.0);
    open_side_nonzero.add(2.0, 2.0, 0.0);
    open_side_nonzero.add(3.0, 1.0, 0.0);

    let mut closed_side = Polyline::new_closed();
    closed_side.add(3.0, 1.0, 0.0);
    closed_side.add(4.0, 4.0, 0.0);
    closed_side.add(1.0, 1.0, 1.0);

    let open_before: Vec<_> = open_side_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side.iter_vertexes().collect();

    let open_aabb = open_side_nonzero.create_approx_aabb_index();
    let closed_aabb = closed_side.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_nonzero.find_intersects_opt(&closed_side, &options_ab);
    let ba = closed_side.find_intersects_opt(&open_side_nonzero, &options_ba);
    let default_ab = open_side_nonzero.find_intersects(&closed_side);
    let default_ba = closed_side.find_intersects(&open_side_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert!(basic_ab.start_index1 > 0);
    assert!(basic_ba.start_index2 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert!(overlap_ab.start_index1 > 0);
    assert!(overlap_ba.start_index2 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    // For this same-order branch, role inversion keeps overlap endpoint ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_same_order_closed_pline2_closure_basic_intersect_nonzero_open_index_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_nonzero = Polyline::new();
    open_side_nonzero.add(2.0, 0.0, 0.0);
    open_side_nonzero.add(2.0, 0.0, 1.0);
    open_side_nonzero.add(2.0, 2.0, 0.0);
    open_side_nonzero.add(3.0, 1.0, 0.0);

    let mut closed_side = Polyline::new_closed();
    closed_side.add(3.0, 1.0, 0.0);
    closed_side.add(4.0, 4.0, 0.0);
    closed_side.add(1.0, 1.0, 1.0);

    let open_before: Vec<_> = open_side_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side.iter_vertexes().collect();

    let open_aabb = open_side_nonzero.create_approx_aabb_index();
    let closed_aabb = closed_side.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_nonzero.find_intersects_opt(&closed_side, &options_ab);
    let ba = closed_side.find_intersects_opt(&open_side_nonzero, &options_ba);
    let default_ab = open_side_nonzero.find_intersects(&closed_side);
    let default_ba = closed_side.find_intersects(&open_side_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert!(basic_ab.start_index1 > 0);
    assert!(basic_ba.start_index2 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert!(overlap_ab.start_index1 > 0);
    assert!(overlap_ba.start_index2 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    // For this same-order branch, role inversion keeps overlap endpoint ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_same_order_closed_pline2_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side = Polyline::new();
    open_side.add(2.0, 0.0, 1.0);
    open_side.add(2.0, 2.0, 0.0);
    open_side.add(3.0, 1.0, 0.0);

    let mut closed_side_rotated_zero_lead = Polyline::new_closed();
    closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    closed_side_rotated_zero_lead.add(1.0, 1.0, 1.0);
    closed_side_rotated_zero_lead.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_rotated_zero_lead.iter_vertexes().collect();

    let open_aabb = open_side.create_approx_aabb_index();
    let closed_aabb = closed_side_rotated_zero_lead.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side.find_intersects_opt(&closed_side_rotated_zero_lead, &options_ab);
    let ba = closed_side_rotated_zero_lead.find_intersects_opt(&open_side, &options_ba);
    let default_ab = open_side.find_intersects(&closed_side_rotated_zero_lead);
    let default_ba = closed_side_rotated_zero_lead.find_intersects(&open_side);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index1, 1);
    assert!(basic_ab.start_index2 > 0);
    assert_eq!(basic_ba.start_index2, 1);
    assert!(basic_ba.start_index1 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index1, 0);
    assert!(overlap_ab.start_index2 > 0);
    assert_eq!(overlap_ba.start_index2, 0);
    assert!(overlap_ba.start_index1 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    // For this same-order branch, role inversion keeps overlap endpoint ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_rotated_zero_lead.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_same_order_closed_pline2_closure_basic_start_index_rotation_zero_length_lead_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side = Polyline::new();
    open_side.add(2.0, 0.0, 1.0);
    open_side.add(2.0, 2.0, 0.0);
    open_side.add(3.0, 1.0, 0.0);

    let mut closed_side_rotated_zero_lead = Polyline::new_closed();
    closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    closed_side_rotated_zero_lead.add(1.0, 1.0, 1.0);
    closed_side_rotated_zero_lead.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_rotated_zero_lead.iter_vertexes().collect();

    let open_aabb = open_side.create_approx_aabb_index();
    let closed_aabb = closed_side_rotated_zero_lead.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side.find_intersects_opt(&closed_side_rotated_zero_lead, &options_ab);
    let ba = closed_side_rotated_zero_lead.find_intersects_opt(&open_side, &options_ba);
    let default_ab = open_side.find_intersects(&closed_side_rotated_zero_lead);
    let default_ba = closed_side_rotated_zero_lead.find_intersects(&open_side);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index1, 1);
    assert!(basic_ab.start_index2 > 0);
    assert_eq!(basic_ba.start_index2, 1);
    assert!(basic_ba.start_index1 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index1, 0);
    assert!(overlap_ab.start_index2 > 0);
    assert_eq!(overlap_ba.start_index2, 0);
    assert!(overlap_ba.start_index1 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    // For this same-order branch, role inversion keeps overlap endpoint ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_rotated_zero_lead.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_same_order_closed_pline2_closure_basic_flipped_roles_options_parity() {
    let mut closed_side = Polyline::new_closed();
    closed_side.add(3.0, 1.0, 0.0);
    closed_side.add(4.0, 4.0, 0.0);
    closed_side.add(1.0, 1.0, 1.0);

    let mut open_side = Polyline::new();
    open_side.add(2.0, 0.0, 1.0);
    open_side.add(2.0, 2.0, 0.0);
    open_side.add(3.0, 1.0, 0.0);

    let closed_before: Vec<_> = closed_side.iter_vertexes().collect();
    let open_before: Vec<_> = open_side.iter_vertexes().collect();

    let closed_aabb = closed_side.create_approx_aabb_index();
    let open_aabb = open_side.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let default_ab = closed_side.find_intersects(&open_side);
    let default_ba = open_side.find_intersects(&closed_side);
    let ab = closed_side.find_intersects_opt(&open_side, &options_ab);
    let ba = open_side.find_intersects_opt(&closed_side, &options_ba);

    assert_eq!(
        ab.basic_intersects.len(),
        1,
        "expected one AB basic intersect"
    );
    assert_eq!(
        ab.overlapping_intersects.len(),
        1,
        "expected one AB overlap"
    );
    assert_eq!(
        ba.basic_intersects.len(),
        1,
        "expected one BA basic intersect"
    );
    assert_eq!(
        ba.overlapping_intersects.len(),
        1,
        "expected one BA overlap"
    );

    assert_eq!(ab.basic_intersects.len(), default_ab.basic_intersects.len());
    assert_eq!(
        ab.overlapping_intersects.len(),
        default_ab.overlapping_intersects.len()
    );
    assert_eq!(ba.basic_intersects.len(), default_ba.basic_intersects.len());
    assert_eq!(
        ba.overlapping_intersects.len(),
        default_ba.overlapping_intersects.len()
    );

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 1);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert!((basic_ab.point.x - 2.0).abs() <= EPS && (basic_ab.point.y - 2.0).abs() <= EPS);
    assert!(
        (basic_ab.point.x - basic_ba.point.x).abs() <= EPS
            && (basic_ab.point.y - basic_ba.point.y).abs() <= EPS
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 2);
    assert_eq!(overlap_ab.start_index2, 0);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert!(
        (overlap_ab.point1.x - 2.0).abs() <= EPS
            && (overlap_ab.point1.y - 0.0).abs() <= EPS
            && (overlap_ab.point2.x - 3.0).abs() <= EPS
            && (overlap_ab.point2.y - 1.0).abs() <= EPS
    );
    assert!(
        (overlap_ab.point1.x - overlap_ba.point1.x).abs() <= EPS
            && (overlap_ab.point1.y - overlap_ba.point1.y).abs() <= EPS
            && (overlap_ab.point2.x - overlap_ba.point2.x).abs() <= EPS
            && (overlap_ab.point2.y - overlap_ba.point2.y).abs() <= EPS
    );

    let closed_after: Vec<_> = closed_side.iter_vertexes().collect();
    let open_after: Vec<_> = open_side.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_closed_pline1_closure_basic_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_overlap_point1: Point,
        expected_overlap_point2: Point,
        expect_role_flip_swaps_endpoint_order: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 4.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn closed_side_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 4.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn open_same_order() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 0.0, 1.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn open_reversed_order() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "same_order_closed_pline1_with_closure_basic",
            lhs: closed_side(),
            rhs: open_same_order(),
            expected_overlap_point1: (2.0, 0.0),
            expected_overlap_point2: (3.0, 1.0),
            expect_role_flip_swaps_endpoint_order: false,
        },
        Case {
            name: "same_order_closed_pline1_with_closure_basic_start_index_rotation",
            lhs: closed_side_rotated(),
            rhs: open_same_order(),
            expected_overlap_point1: (2.0, 0.0),
            expected_overlap_point2: (3.0, 1.0),
            expect_role_flip_swaps_endpoint_order: false,
        },
        Case {
            name: "reversed_order_closed_pline1_with_closure_basic",
            lhs: closed_side(),
            rhs: open_reversed_order(),
            expected_overlap_point1: (3.0, 1.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_role_flip_swaps_endpoint_order: true,
        },
        Case {
            name: "reversed_order_closed_pline1_with_closure_basic_start_index_rotation",
            lhs: closed_side_rotated(),
            rhs: open_reversed_order(),
            expected_overlap_point1: (3.0, 1.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_role_flip_swaps_endpoint_order: true,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert_eq!(
            ab.basic_intersects.len(),
            1,
            "{}: AB basic count",
            case.name
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert_eq!(
            ba.basic_intersects.len(),
            1,
            "{}: BA basic count",
            case.name
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let basic_ab = ab.basic_intersects[0];
        let basic_ba = ba.basic_intersects[0];
        assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
        assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
        assert!(
            point_eq((basic_ab.point.x, basic_ab.point.y), (2.0, 2.0)),
            "{}: unexpected AB basic point: {:?}",
            case.name,
            basic_ab
        );
        assert!(
            point_eq(
                (basic_ab.point.x, basic_ab.point.y),
                (basic_ba.point.x, basic_ba.point.y)
            ),
            "{}: AB/BA basic point diverged: AB={:?}, BA={:?}",
            case.name,
            basic_ab,
            basic_ba
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        assert_ne!(
            overlap_ab.start_index1, 0,
            "{}: expected non-zero overlap start_index1",
            case.name
        );
        assert_eq!(
            overlap_ab.start_index2, 0,
            "{}: expected zero overlap start_index2",
            case.name
        );
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                case.expected_overlap_point1
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                case.expected_overlap_point2
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );

        if case.expect_role_flip_swaps_endpoint_order {
            assert!(
                point_eq(
                    (overlap_ab.point1.x, overlap_ab.point1.y),
                    (overlap_ba.point2.x, overlap_ba.point2.y)
                ) && point_eq(
                    (overlap_ab.point2.x, overlap_ab.point2.y),
                    (overlap_ba.point1.x, overlap_ba.point1.y)
                ),
                "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
                case.name,
                overlap_ab,
                overlap_ba
            );
        } else {
            assert!(
                point_eq(
                    (overlap_ab.point1.x, overlap_ab.point1.y),
                    (overlap_ba.point1.x, overlap_ba.point1.y)
                ) && point_eq(
                    (overlap_ab.point2.x, overlap_ab.point2.y),
                    (overlap_ba.point2.x, overlap_ba.point2.y)
                ),
                "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
                case.name,
                overlap_ab,
                overlap_ba
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_both_closed_dedup_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_overlap_point1: Point,
        expected_overlap_point2: Point,
        expect_start_index1_nonzero: bool,
        expect_start_index2_nonzero: bool,
        expect_role_flip_swaps_endpoint_order: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn same_order_closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 5.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn same_order_closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 5.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn same_order_closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 1.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, -1.0, 0.0);
        pline
    }

    fn same_order_closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, -1.0, 0.0);
        pline.add(2.0, 0.0, 1.0);
        pline
    }

    fn reversed_order_closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(6.0, 4.0, 0.0);
        pline
    }

    fn reversed_order_closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(6.0, 4.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn reversed_order_closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(6.0, -3.0, 0.0);
        pline.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
        pline
    }

    fn reversed_order_closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(6.0, -3.0, 0.0);
        pline.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
        pline.add(2.0, 0.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "same_order_both_closed",
            lhs: same_order_closed_side_a(),
            rhs: same_order_closed_side_b(),
            expected_overlap_point1: (2.0, 0.0),
            expected_overlap_point2: (3.0, 1.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
            expect_role_flip_swaps_endpoint_order: false,
        },
        Case {
            name: "same_order_both_closed_start_index_rotation_closed_pline2",
            lhs: same_order_closed_side_a(),
            rhs: same_order_closed_side_b_rotated(),
            expected_overlap_point1: (2.0, 0.0),
            expected_overlap_point2: (3.0, 1.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: true,
            expect_role_flip_swaps_endpoint_order: false,
        },
        Case {
            name: "same_order_both_closed_start_index_rotation_closed_pline1",
            lhs: same_order_closed_side_a_rotated(),
            rhs: same_order_closed_side_b(),
            expected_overlap_point1: (2.0, 0.0),
            expected_overlap_point2: (3.0, 1.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
            expect_role_flip_swaps_endpoint_order: false,
        },
        Case {
            name: "reversed_order_both_closed",
            lhs: reversed_order_closed_side_a(),
            rhs: reversed_order_closed_side_b(),
            expected_overlap_point1: (3.0, 1.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
            expect_role_flip_swaps_endpoint_order: true,
        },
        Case {
            name: "reversed_order_both_closed_start_index_rotation_closed_pline2",
            lhs: reversed_order_closed_side_a(),
            rhs: reversed_order_closed_side_b_rotated(),
            expected_overlap_point1: (3.0, 1.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
            expect_role_flip_swaps_endpoint_order: true,
        },
        Case {
            name: "reversed_order_both_closed_start_index_rotation_closed_pline1",
            lhs: reversed_order_closed_side_a_rotated(),
            rhs: reversed_order_closed_side_b(),
            expected_overlap_point1: (3.0, 1.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: true,
            expect_role_flip_swaps_endpoint_order: true,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basic intersects, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basic intersects, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if case.expect_start_index1_nonzero {
            assert_ne!(
                overlap_ab.start_index1, 0,
                "{}: expected non-zero overlap start_index1",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero overlap start_index1",
                case.name
            );
        }
        if case.expect_start_index2_nonzero {
            assert_ne!(
                overlap_ab.start_index2, 0,
                "{}: expected non-zero overlap start_index2",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero overlap start_index2",
                case.name
            );
        }
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                case.expected_overlap_point1
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                case.expected_overlap_point2
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );

        if case.expect_role_flip_swaps_endpoint_order {
            assert!(
                point_eq(
                    (overlap_ab.point1.x, overlap_ab.point1.y),
                    (overlap_ba.point2.x, overlap_ba.point2.y)
                ) && point_eq(
                    (overlap_ab.point2.x, overlap_ab.point2.y),
                    (overlap_ba.point1.x, overlap_ba.point1.y)
                ),
                "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
                case.name,
                overlap_ab,
                overlap_ba
            );
        } else {
            assert!(
                point_eq(
                    (overlap_ab.point1.x, overlap_ab.point1.y),
                    (overlap_ba.point1.x, overlap_ba.point1.y)
                ) && point_eq(
                    (overlap_ab.point2.x, overlap_ab.point2.y),
                    (overlap_ba.point2.x, overlap_ba.point2.y)
                ),
                "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
                case.name,
                overlap_ab,
                overlap_ba
            );
        }

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_overlap_endpoint_dedup_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_overlap_point1: Point,
        expected_overlap_point2: Point,
        expect_start_index1_nonzero: bool,
        expect_start_index2_nonzero: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(0.0, 0.0, 0.0);
        pline.add(2.0, 1.0, 0.0);
        pline.add(4.0, 0.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 1.0, 0.0);
        pline.add(4.0, 0.0, 0.0);
        pline.add(0.0, 0.0, 0.0);
        pline
    }

    fn open_side() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(3.0, 0.0, 0.0);
        pline.add(0.0, 0.0, 0.0);
        pline.add(0.0, -1.0, 0.0);
        pline
    }

    fn both_closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 0.0, 0.0);
        pline.add(0.0, 0.0, 0.0);
        pline.add(0.0, -1.0, 0.0);
        pline.add(-1.0, -2.0, 0.0);
        pline
    }

    fn both_closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(0.0, 0.0, 0.0);
        pline.add(0.0, -1.0, 0.0);
        pline.add(-1.0, -2.0, 0.0);
        pline.add(3.0, 0.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1",
            lhs: closed_side_a(),
            rhs: open_side(),
            expected_overlap_point1: (3.0, 0.0),
            expected_overlap_point2: (0.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "closed_pline2",
            lhs: open_side(),
            rhs: closed_side_a(),
            expected_overlap_point1: (3.0, 0.0),
            expected_overlap_point2: (0.0, 0.0),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "both_closed",
            lhs: closed_side_a(),
            rhs: both_closed_side_b(),
            expected_overlap_point1: (3.0, 0.0),
            expected_overlap_point2: (0.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2",
            lhs: closed_side_a(),
            rhs: both_closed_side_b_rotated(),
            expected_overlap_point1: (3.0, 0.0),
            expected_overlap_point2: (0.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "closed_pline2_start_index_rotation",
            lhs: open_side(),
            rhs: closed_side_a_rotated(),
            expected_overlap_point1: (3.0, 0.0),
            expected_overlap_point2: (0.0, 0.0),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "closed_pline1_start_index_rotation",
            lhs: closed_side_a_rotated(),
            rhs: open_side(),
            expected_overlap_point1: (3.0, 0.0),
            expected_overlap_point2: (0.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basic intersects, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basic intersects, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);

        if case.expect_start_index1_nonzero {
            assert_ne!(
                overlap_ab.start_index1, 0,
                "{}: expected non-zero overlap start_index1",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero overlap start_index1",
                case.name
            );
        }
        if case.expect_start_index2_nonzero {
            assert_ne!(
                overlap_ab.start_index2, 0,
                "{}: expected non-zero overlap start_index2",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero overlap start_index2",
                case.name
            );
        }

        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                case.expected_overlap_point1
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                case.expected_overlap_point2
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ),
            "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_overlap_endpoint_dedup_start_index_rotation_role_flip_options_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_overlap_start_index1_nonzero: Option<bool>,
        expect_overlap_start_index2_nonzero: Option<bool>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(0.0, 0.0, 0.0);
        pline.add(2.0, 1.0, 0.0);
        pline.add(4.0, 0.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(0.0, 0.0, 0.0);
        pline.add(0.0, -1.0, 0.0);
        pline.add(-1.0, -2.0, 0.0);
        pline.add(3.0, 0.0, 0.0);
        pline
    }

    fn closed_side_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 1.0, 0.0);
        pline.add(4.0, 0.0, 0.0);
        pline.add(0.0, 0.0, 0.0);
        pline
    }

    fn open_side() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(3.0, 0.0, 0.0);
        pline.add(0.0, 0.0, 0.0);
        pline.add(0.0, -1.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_start_index_rotation_role_flip",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_role_flip",
            lhs: closed_side_rotated(),
            rhs: open_side(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(false),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_role_flip",
            lhs: open_side(),
            rhs: closed_side_rotated(),
            expect_overlap_start_index1_nonzero: Some(false),
            expect_overlap_start_index2_nonzero: Some(true),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basics, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basics, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if let Some(expect_nonzero) = case.expect_overlap_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index1, 0,
                    "{}: expected non-zero overlap start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index1, 0,
                    "{}: expected zero overlap start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_overlap_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index2, 0,
                    "{}: expected non-zero overlap start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index2, 0,
                    "{}: expected zero overlap start_index2",
                    case.name
                );
            }
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (3.0, 0.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (0.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // In this bounded wrap-around overlap-endpoint dedup branch, role inversion keeps
        // overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ),
            "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_overlap_endpoint_dedup_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_overlap_start_index1_nonzero: Option<bool>,
        expect_overlap_start_index2_nonzero: Option<bool>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(0.0, 0.0, 0.0);
        pline.add(2.0, 1.0, 0.0);
        pline.add(4.0, 0.0, 0.0);
        pline
    }

    fn closed_side_b_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(0.0, 0.0, 0.0);
        pline.add(0.0, -1.0, 0.0);
        pline.add(0.0, -1.0, 0.0);
        pline.add(-1.0, -2.0, 0.0);
        pline.add(3.0, 0.0, 0.0);
        pline
    }

    fn closed_side_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 1.0, 0.0);
        pline.add(2.0, 1.0, 0.0);
        pline.add(4.0, 0.0, 0.0);
        pline.add(0.0, 0.0, 0.0);
        pline
    }

    fn open_side() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(3.0, 0.0, 0.0);
        pline.add(0.0, 0.0, 0.0);
        pline.add(0.0, -1.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_start_index_rotation_zero_length_lead_role_flip",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated_zero_lead(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip",
            lhs: closed_side_rotated_zero_lead(),
            rhs: open_side(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(false),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip",
            lhs: open_side(),
            rhs: closed_side_rotated_zero_lead(),
            expect_overlap_start_index1_nonzero: Some(false),
            expect_overlap_start_index2_nonzero: Some(true),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basics, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basics, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if let Some(expect_nonzero) = case.expect_overlap_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index1, 0,
                    "{}: expected non-zero overlap start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index1, 0,
                    "{}: expected zero overlap start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_overlap_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index2, 0,
                    "{}: expected non-zero overlap start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index2, 0,
                    "{}: expected zero overlap start_index2",
                    case.name
                );
            }
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (3.0, 0.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (0.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // In this bounded wrap-around overlap-endpoint dedup branch, role inversion keeps
        // overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ),
            "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_overlap_endpoint_arc_adjacent_dedup_options_matrix_parity() {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expected_overlap_point1: Point,
        expected_overlap_point2: Point,
        expect_start_index1_nonzero: bool,
        expect_start_index2_nonzero: bool,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline.add(3.0, 1.0, 0.0);
        pline.add(1.0, 0.0, 0.0);
        pline
    }

    fn closed_side_a_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(1.0, 0.0, 0.0);
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline
    }

    fn open_side() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.5, 0.0, 0.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn both_closed_side_b() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.5, 0.0, 0.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(0.0, -2.0, 0.0);
        pline
    }

    fn both_closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(0.0, -2.0, 0.0);
        pline.add(1.5, 0.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "closed_pline1",
            lhs: closed_side_a(),
            rhs: open_side(),
            expected_overlap_point1: (1.5, 0.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "closed_pline2",
            lhs: open_side(),
            rhs: closed_side_a(),
            expected_overlap_point1: (1.5, 0.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "both_closed",
            lhs: closed_side_a(),
            rhs: both_closed_side_b(),
            expected_overlap_point1: (1.5, 0.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2",
            lhs: closed_side_a(),
            rhs: both_closed_side_b_rotated(),
            expected_overlap_point1: (1.5, 0.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "closed_pline2_start_index_rotation",
            lhs: open_side(),
            rhs: closed_side_a_rotated(),
            expected_overlap_point1: (1.5, 0.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index1_nonzero: false,
            expect_start_index2_nonzero: true,
        },
        Case {
            name: "closed_pline1_start_index_rotation",
            lhs: closed_side_a_rotated(),
            rhs: open_side(),
            expected_overlap_point1: (1.5, 0.0),
            expected_overlap_point2: (2.0, 0.0),
            expect_start_index1_nonzero: true,
            expect_start_index2_nonzero: false,
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basic intersects, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basic intersects, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);

        if case.expect_start_index1_nonzero {
            assert_ne!(
                overlap_ab.start_index1, 0,
                "{}: expected non-zero overlap start_index1",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index1, 0,
                "{}: expected zero overlap start_index1",
                case.name
            );
        }
        if case.expect_start_index2_nonzero {
            assert_ne!(
                overlap_ab.start_index2, 0,
                "{}: expected non-zero overlap start_index2",
                case.name
            );
        } else {
            assert_eq!(
                overlap_ab.start_index2, 0,
                "{}: expected zero overlap start_index2",
                case.name
            );
        }

        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                case.expected_overlap_point1
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                case.expected_overlap_point2
            ),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ),
            "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_overlap_endpoint_arc_adjacent_dedup_start_index_rotation_role_flip_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_overlap_start_index1_nonzero: Option<bool>,
        expect_overlap_start_index2_nonzero: Option<bool>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline.add(3.0, 1.0, 0.0);
        pline.add(1.0, 0.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(0.0, -2.0, 0.0);
        pline.add(1.5, 0.0, 0.0);
        pline
    }

    fn closed_side_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(1.0, 0.0, 0.0);
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline
    }

    fn open_side() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.5, 0.0, 0.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_start_index_rotation_role_flip",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_role_flip",
            lhs: closed_side_rotated(),
            rhs: open_side(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(false),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_role_flip",
            lhs: open_side(),
            rhs: closed_side_rotated(),
            expect_overlap_start_index1_nonzero: Some(false),
            expect_overlap_start_index2_nonzero: Some(true),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basics, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basics, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if let Some(expect_nonzero) = case.expect_overlap_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index1, 0,
                    "{}: expected non-zero overlap start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index1, 0,
                    "{}: expected zero overlap start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_overlap_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index2, 0,
                    "{}: expected non-zero overlap start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index2, 0,
                    "{}: expected zero overlap start_index2",
                    case.name
                );
            }
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (1.5, 0.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (2.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // In this bounded wrap-around arc-adjacent dedup branch, role inversion keeps
        // overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ),
            "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_overlap_endpoint_arc_adjacent_dedup_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_overlap_start_index1_nonzero: Option<bool>,
        expect_overlap_start_index2_nonzero: Option<bool>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline.add(3.0, 1.0, 0.0);
        pline.add(1.0, 0.0, 0.0);
        pline
    }

    fn closed_side_b_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline.add(0.0, -2.0, 0.0);
        pline.add(1.5, 0.0, 0.0);
        pline
    }

    fn closed_side_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(1.0, 0.0, 0.0);
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline
    }

    fn open_side() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.5, 0.0, 0.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_start_index_rotation_zero_length_lead_role_flip",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated_zero_lead(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip",
            lhs: closed_side_rotated_zero_lead(),
            rhs: open_side(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(false),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip",
            lhs: open_side(),
            rhs: closed_side_rotated_zero_lead(),
            expect_overlap_start_index1_nonzero: Some(false),
            expect_overlap_start_index2_nonzero: Some(true),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basics, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basics, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if let Some(expect_nonzero) = case.expect_overlap_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index1, 0,
                    "{}: expected non-zero overlap start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index1, 0,
                    "{}: expected zero overlap start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_overlap_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index2, 0,
                    "{}: expected non-zero overlap start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index2, 0,
                    "{}: expected zero overlap start_index2",
                    case.name
                );
            }
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (1.5, 0.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (2.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // In this bounded wrap-around arc-adjacent dedup branch, role inversion keeps
        // overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ),
            "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_non_circle_arc_overlap_deduplication_same_order_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_overlap_start_index1_nonzero: Option<bool>,
        expect_overlap_start_index2_nonzero: Option<bool>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 5.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn closed_side_b_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, -1.0, 0.0);
        pline.add(4.0, -1.0, 0.0);
        pline.add(2.0, 0.0, 1.0);
        pline
    }

    fn closed_side_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 5.0, 0.0);
        pline.add(4.0, 5.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn open_side_closed_pline1() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 0.0, 1.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn open_side_closed_pline2() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_rotated_pline2_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 3.0, 0.0);
        pline.add(4.0, 3.0, 0.0);
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_start_index_rotation_zero_length_lead_role_flip",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated_zero_lead(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip",
            lhs: closed_side_rotated_zero_lead(),
            rhs: open_side_closed_pline1(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(false),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip",
            lhs: open_side_closed_pline2(),
            rhs: closed_side_rotated_pline2_zero_lead(),
            expect_overlap_start_index1_nonzero: Some(false),
            expect_overlap_start_index2_nonzero: Some(true),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basics, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basics, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if let Some(expect_nonzero) = case.expect_overlap_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index1, 0,
                    "{}: expected non-zero overlap start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index1, 0,
                    "{}: expected zero overlap start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_overlap_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index2, 0,
                    "{}: expected non-zero overlap start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index2, 0,
                    "{}: expected zero overlap start_index2",
                    case.name
                );
            }
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (2.0, 0.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (3.0, 1.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // In this bounded same-order wrap-around non-circle arc-overlap dedup branch,
        // role inversion keeps overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ),
            "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_non_circle_arc_overlap_deduplication_reversed_order_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_overlap_start_index1_nonzero: Option<bool>,
        expect_overlap_start_index2_nonzero: Option<bool>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(6.0, 4.0, 0.0);
        pline
    }

    fn closed_side_b_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(6.0, -3.0, 0.0);
        pline.add(6.0, -3.0, 0.0);
        pline.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
        pline.add(2.0, 0.0, 0.0);
        pline
    }

    fn closed_side_rotated_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 5.0, 0.0);
        pline.add(4.0, 5.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn open_side_closed_pline1() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_closed_pline2() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_rotated_pline2_zero_lead() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(6.0, -3.0, 0.0);
        pline.add(6.0, -3.0, 0.0);
        pline.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
        pline.add(2.0, 0.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_start_index_rotation_zero_length_lead_role_flip",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated_zero_lead(),
            expect_overlap_start_index1_nonzero: Some(false),
            expect_overlap_start_index2_nonzero: Some(true),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip",
            lhs: closed_side_rotated_zero_lead(),
            rhs: open_side_closed_pline1(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(false),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip",
            lhs: open_side_closed_pline2(),
            rhs: closed_side_rotated_pline2_zero_lead(),
            expect_overlap_start_index1_nonzero: Some(false),
            expect_overlap_start_index2_nonzero: Some(true),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basics, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basics, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if let Some(expect_nonzero) = case.expect_overlap_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index1, 0,
                    "{}: expected non-zero overlap start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index1, 0,
                    "{}: expected zero overlap start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_overlap_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index2, 0,
                    "{}: expected non-zero overlap start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index2, 0,
                    "{}: expected zero overlap start_index2",
                    case.name
                );
            }
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (3.0, 1.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (2.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // In this bounded reversed-order wrap-around non-circle arc-overlap dedup branch,
        // role inversion swaps overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ),
            "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_non_circle_arc_overlap_deduplication_same_order_start_index_rotation_role_flip_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_overlap_start_index1_nonzero: Option<bool>,
        expect_overlap_start_index2_nonzero: Option<bool>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 5.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, -1.0, 0.0);
        pline.add(2.0, 0.0, 1.0);
        pline
    }

    fn closed_side_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 5.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn open_side_closed_pline1() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 0.0, 1.0);
        pline.add(2.0, 2.0, 0.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn open_side_closed_pline2() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_rotated_pline2() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 3.0, 0.0);
        pline.add(2.0, 0.0, bulge_from_angle(FRAC_PI_2));
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_start_index_rotation_role_flip",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(true),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_role_flip",
            lhs: closed_side_rotated(),
            rhs: open_side_closed_pline1(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(false),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_role_flip",
            lhs: open_side_closed_pline2(),
            rhs: closed_side_rotated_pline2(),
            expect_overlap_start_index1_nonzero: Some(false),
            expect_overlap_start_index2_nonzero: Some(true),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basics, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basics, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if let Some(expect_nonzero) = case.expect_overlap_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index1, 0,
                    "{}: expected non-zero overlap start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index1, 0,
                    "{}: expected zero overlap start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_overlap_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index2, 0,
                    "{}: expected non-zero overlap start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index2, 0,
                    "{}: expected zero overlap start_index2",
                    case.name
                );
            }
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (2.0, 0.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (3.0, 1.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // In this bounded same-order wrap-around non-circle arc-overlap dedup branch,
        // role inversion keeps overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ),
            "{}: AB/BA overlap order invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_non_circle_arc_overlap_deduplication_reversed_order_start_index_rotation_role_flip_options_parity()
 {
    type Point = (f64, f64);

    #[derive(Clone)]
    struct Case {
        name: &'static str,
        lhs: Polyline<f64>,
        rhs: Polyline<f64>,
        expect_overlap_start_index1_nonzero: Option<bool>,
        expect_overlap_start_index2_nonzero: Option<bool>,
    }

    fn normalize_basics(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64)> {
        let mut v = intersects
            .basic_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point.x * 1.0e12).round() as i64,
                    (intr.point.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn normalize_overlaps(
        intersects: &cavalier_contours::polyline::PlineIntersectsCollection<f64>,
    ) -> Vec<(usize, usize, i64, i64, i64, i64)> {
        let mut v = intersects
            .overlapping_intersects
            .iter()
            .map(|intr| {
                (
                    intr.start_index1,
                    intr.start_index2,
                    (intr.point1.x * 1.0e12).round() as i64,
                    (intr.point1.y * 1.0e12).round() as i64,
                    (intr.point2.x * 1.0e12).round() as i64,
                    (intr.point2.y * 1.0e12).round() as i64,
                )
            })
            .collect::<Vec<_>>();
        v.sort_unstable();
        v
    }

    fn point_eq(p: Point, expected: Point) -> bool {
        (p.0 - expected.0).abs() <= EPS && (p.1 - expected.1).abs() <= EPS
    }

    fn closed_side_a() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline.add(6.0, 4.0, 0.0);
        pline
    }

    fn closed_side_b_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(6.0, -3.0, 0.0);
        pline.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
        pline.add(2.0, 0.0, 0.0);
        pline
    }

    fn closed_side_rotated() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(4.0, 5.0, 0.0);
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline
    }

    fn open_side_closed_pline1() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(2.0, 2.0, -1.0);
        pline.add(2.0, 0.0, 0.0);
        pline.add(2.0, -1.0, 0.0);
        pline
    }

    fn open_side_closed_pline2() -> Polyline<f64> {
        let mut pline = Polyline::new();
        pline.add(1.0, 1.0, 1.0);
        pline.add(3.0, 1.0, 0.0);
        pline.add(4.0, 1.0, 0.0);
        pline
    }

    fn closed_side_rotated_pline2() -> Polyline<f64> {
        let mut pline = Polyline::new_closed();
        pline.add(6.0, -3.0, 0.0);
        pline.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
        pline.add(2.0, 0.0, 0.0);
        pline
    }

    let cases = [
        Case {
            name: "both_closed_start_index_rotation_role_flip",
            lhs: closed_side_a(),
            rhs: closed_side_b_rotated(),
            expect_overlap_start_index1_nonzero: Some(false),
            expect_overlap_start_index2_nonzero: Some(true),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline1_role_flip",
            lhs: closed_side_rotated(),
            rhs: open_side_closed_pline1(),
            expect_overlap_start_index1_nonzero: Some(true),
            expect_overlap_start_index2_nonzero: Some(false),
        },
        Case {
            name: "both_closed_start_index_rotation_closed_pline2_role_flip",
            lhs: open_side_closed_pline2(),
            rhs: closed_side_rotated_pline2(),
            expect_overlap_start_index1_nonzero: Some(false),
            expect_overlap_start_index2_nonzero: Some(true),
        },
    ];

    for case in &cases {
        let lhs_before: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_before: Vec<_> = case.rhs.iter_vertexes().collect();

        let lhs_aabb = case.lhs.create_approx_aabb_index();
        let rhs_aabb = case.rhs.create_approx_aabb_index();
        let options_ab = FindIntersectsOptions {
            pline1_aabb_index: Some(&lhs_aabb),
            pos_equal_eps: EPS,
        };
        let options_ba = FindIntersectsOptions {
            pline1_aabb_index: Some(&rhs_aabb),
            pos_equal_eps: EPS,
        };

        let default_ab = case.lhs.find_intersects(&case.rhs);
        let default_ba = case.rhs.find_intersects(&case.lhs);
        let ab = case.lhs.find_intersects_opt(&case.rhs, &options_ab);
        let ba = case.rhs.find_intersects_opt(&case.lhs, &options_ba);

        assert_eq!(
            normalize_basics(&ab),
            normalize_basics(&default_ab),
            "{}: options/default AB basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ab),
            normalize_overlaps(&default_ab),
            "{}: options/default AB overlap mismatch",
            case.name
        );
        assert_eq!(
            normalize_basics(&ba),
            normalize_basics(&default_ba),
            "{}: options/default BA basic mismatch",
            case.name
        );
        assert_eq!(
            normalize_overlaps(&ba),
            normalize_overlaps(&default_ba),
            "{}: options/default BA overlap mismatch",
            case.name
        );

        assert!(
            ab.basic_intersects.is_empty(),
            "{}: expected no AB basics, got {:?}",
            case.name,
            ab.basic_intersects
        );
        assert_eq!(
            ab.overlapping_intersects.len(),
            1,
            "{}: AB overlap count",
            case.name
        );
        assert!(
            ba.basic_intersects.is_empty(),
            "{}: expected no BA basics, got {:?}",
            case.name,
            ba.basic_intersects
        );
        assert_eq!(
            ba.overlapping_intersects.len(),
            1,
            "{}: BA overlap count",
            case.name
        );

        let overlap_ab = ab.overlapping_intersects[0];
        let overlap_ba = ba.overlapping_intersects[0];
        assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
        assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
        if let Some(expect_nonzero) = case.expect_overlap_start_index1_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index1, 0,
                    "{}: expected non-zero overlap start_index1",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index1, 0,
                    "{}: expected zero overlap start_index1",
                    case.name
                );
            }
        }
        if let Some(expect_nonzero) = case.expect_overlap_start_index2_nonzero {
            if expect_nonzero {
                assert_ne!(
                    overlap_ab.start_index2, 0,
                    "{}: expected non-zero overlap start_index2",
                    case.name
                );
            } else {
                assert_eq!(
                    overlap_ab.start_index2, 0,
                    "{}: expected zero overlap start_index2",
                    case.name
                );
            }
        }
        assert!(
            point_eq((overlap_ab.point1.x, overlap_ab.point1.y), (3.0, 1.0))
                && point_eq((overlap_ab.point2.x, overlap_ab.point2.y), (2.0, 0.0)),
            "{}: unexpected AB overlap endpoints: {:?}",
            case.name,
            overlap_ab
        );
        // In this bounded reversed-order wrap-around non-circle arc-overlap dedup branch,
        // role inversion swaps overlap endpoint ordering.
        assert!(
            point_eq(
                (overlap_ab.point1.x, overlap_ab.point1.y),
                (overlap_ba.point2.x, overlap_ba.point2.y)
            ) && point_eq(
                (overlap_ab.point2.x, overlap_ab.point2.y),
                (overlap_ba.point1.x, overlap_ba.point1.y)
            ),
            "{}: AB/BA overlap swap invariant failed: AB={:?}, BA={:?}",
            case.name,
            overlap_ab,
            overlap_ba
        );

        let lhs_after: Vec<_> = case.lhs.iter_vertexes().collect();
        let rhs_after: Vec<_> = case.rhs.iter_vertexes().collect();
        assert_eq!(
            lhs_after, lhs_before,
            "{}: lhs mutated by find_intersects_opt",
            case.name
        );
        assert_eq!(
            rhs_after, rhs_before,
            "{}: rhs mutated by find_intersects_opt",
            case.name
        );
    }
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_options_matrix_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed = Polyline::new_closed();
    closed_side_reversed.add(2.0, 0.0, 0.0);
    closed_side_reversed.add(1.0, 3.0, 0.0);
    closed_side_reversed.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed.iter_vertexes().collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed.find_intersects_opt(&closed_side_reversed, &options_ab);
    let ba = closed_side_reversed.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&closed_side_reversed);
    let default_ba = closed_side_reversed.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 1);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 0);
    assert_eq!(overlap_ab.start_index2, 2);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_closed_side_reversed_closure_basic_options_matrix_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_side_reversed = Polyline::new_closed();
    closed_side_reversed.add(2.0, 0.0, 0.0);
    closed_side_reversed.add(1.0, 3.0, 0.0);
    closed_side_reversed.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_side_reversed.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_side_reversed.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_side_reversed.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&closed_side_reversed, &options_ba);
    let default_ab = closed_side_reversed.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 2);
    assert_eq!(overlap_ab.start_index2, 0);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_side_reversed.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_closed_side_reversed_closure_basic_role_flip_options_matrix_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_side_reversed = Polyline::new_closed();
    closed_side_reversed.add(2.0, 0.0, 0.0);
    closed_side_reversed.add(1.0, 3.0, 0.0);
    closed_side_reversed.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_side_reversed.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_side_reversed.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_side_reversed.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&closed_side_reversed, &options_ba);
    let default_ab = closed_side_reversed.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this closed-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_side_reversed.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_side_reversed_role_flip_options_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_side_reversed = Polyline::new_closed();
    closed_side_reversed.add(2.0, 0.0, 0.0);
    closed_side_reversed.add(1.0, 3.0, 0.0);
    closed_side_reversed.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_side_reversed.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_side_reversed.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_side_reversed.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&closed_side_reversed, &options_ba);
    let default_ab = closed_side_reversed.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 2);
    assert_eq!(overlap_ab.start_index2, 0);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_side_reversed.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_options_matrix_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut normal_closed_side = Polyline::new_closed();
    normal_closed_side.add(3.0, 1.0, 0.0);
    normal_closed_side.add(4.0, 4.0, 0.0);
    normal_closed_side.add(1.0, 1.0, 1.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side.iter_vertexes().collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = normal_closed_side.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed.find_intersects_opt(&normal_closed_side, &options_ab);
    let ba = normal_closed_side.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&normal_closed_side);
    let default_ba = normal_closed_side.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 1);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 0);
    assert_eq!(overlap_ab.start_index2, 2);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut normal_closed_side = Polyline::new_closed();
    normal_closed_side.add(3.0, 1.0, 0.0);
    normal_closed_side.add(4.0, 4.0, 0.0);
    normal_closed_side.add(1.0, 1.0, 1.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side.iter_vertexes().collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = normal_closed_side.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed.find_intersects_opt(&normal_closed_side, &options_ab);
    let ba = normal_closed_side.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&normal_closed_side);
    let default_ba = normal_closed_side.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 1);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 0);
    assert_eq!(overlap_ab.start_index2, 2);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut normal_closed_side = Polyline::new_closed();
    normal_closed_side.add(3.0, 1.0, 0.0);
    normal_closed_side.add(4.0, 4.0, 0.0);
    normal_closed_side.add(1.0, 1.0, 1.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side.iter_vertexes().collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = normal_closed_side.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed.find_intersects_opt(&normal_closed_side, &options_ab);
    let ba = normal_closed_side.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&normal_closed_side);
    let default_ba = normal_closed_side.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 1);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 0);
    assert_eq!(overlap_ab.start_index2, 2);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_role_flip_options_matrix_parity()
{
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_side_reversed = Polyline::new_closed();
    closed_side_reversed.add(2.0, 0.0, 0.0);
    closed_side_reversed.add(1.0, 3.0, 0.0);
    closed_side_reversed.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_side_reversed.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_side_reversed.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_side_reversed.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&closed_side_reversed, &options_ba);
    let default_ab = closed_side_reversed.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 2);
    assert_eq!(overlap_ab.start_index2, 0);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_side_reversed.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_role_flip_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut normal_closed_side = Polyline::new_closed();
    normal_closed_side.add(3.0, 1.0, 0.0);
    normal_closed_side.add(4.0, 4.0, 0.0);
    normal_closed_side.add(1.0, 1.0, 1.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = normal_closed_side.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = normal_closed_side.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = normal_closed_side.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&normal_closed_side, &options_ba);
    let default_ab = normal_closed_side.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&normal_closed_side);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 2);
    assert_eq!(overlap_ab.start_index2, 0);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 3.0, 1.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = normal_closed_side.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_role_flip_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut normal_closed_side = Polyline::new_closed();
    normal_closed_side.add(3.0, 1.0, 0.0);
    normal_closed_side.add(4.0, 4.0, 0.0);
    normal_closed_side.add(1.0, 1.0, 1.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = normal_closed_side.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = normal_closed_side.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = normal_closed_side.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&normal_closed_side, &options_ba);
    let default_ab = normal_closed_side.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&normal_closed_side);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 2);
    assert_eq!(overlap_ab.start_index2, 0);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 3.0, 1.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = normal_closed_side.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_role_flip_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut normal_closed_side = Polyline::new_closed();
    normal_closed_side.add(3.0, 1.0, 0.0);
    normal_closed_side.add(4.0, 4.0, 0.0);
    normal_closed_side.add(1.0, 1.0, 1.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = normal_closed_side.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = normal_closed_side.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = normal_closed_side.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&normal_closed_side, &options_ba);
    let default_ab = normal_closed_side.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&normal_closed_side);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 2);
    assert_eq!(overlap_ab.start_index2, 0);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 3.0, 1.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = normal_closed_side.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_role_flip_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut normal_closed_side_rotated = Polyline::new_closed();
    normal_closed_side_rotated.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated.add(3.0, 1.0, 0.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = normal_closed_side_rotated.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = normal_closed_side_rotated.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&normal_closed_side_rotated, &options_ba);
    let default_ab = normal_closed_side_rotated.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&normal_closed_side_rotated);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 3.0, 1.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_role_flip_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut normal_closed_side_rotated = Polyline::new_closed();
    normal_closed_side_rotated.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated.add(3.0, 1.0, 0.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = normal_closed_side_rotated.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = normal_closed_side_rotated.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&normal_closed_side_rotated, &options_ba);
    let default_ab = normal_closed_side_rotated.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&normal_closed_side_rotated);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 3.0, 1.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut normal_closed_side_rotated_zero_lead = Polyline::new_closed();
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated_zero_lead.add(3.0, 1.0, 0.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = normal_closed_side_rotated_zero_lead.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        normal_closed_side_rotated_zero_lead.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba =
        open_side_reversed.find_intersects_opt(&normal_closed_side_rotated_zero_lead, &options_ba);
    let default_ab = normal_closed_side_rotated_zero_lead.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&normal_closed_side_rotated_zero_lead);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ba.start_index1, 0);
    assert!(basic_ab.start_index1 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ba.start_index1, 0);
    assert!(overlap_ab.start_index1 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 3.0, 1.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut closed_pline2_rotated = Polyline::new_closed();
    closed_pline2_rotated.add(1.0, 3.0, 0.0);
    closed_pline2_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_pline2_rotated.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_pline2_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = closed_pline2_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed.find_intersects_opt(&closed_pline2_rotated, &options_ab);
    let ba = closed_pline2_rotated.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&closed_pline2_rotated);
    let default_ba = closed_pline2_rotated.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 0);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_pline2_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_zero_length_lead_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed_rotated_zero_lead = Polyline::new_closed();
    closed_side_reversed_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated_zero_lead.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated_zero_lead.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed_rotated_zero_lead
        .iter_vertexes()
        .collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed_rotated_zero_lead.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed
        .find_intersects_opt(&closed_side_reversed_rotated_zero_lead, &options_ab);
    let ba = closed_side_reversed_rotated_zero_lead
        .find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&closed_side_reversed_rotated_zero_lead);
    let default_ba = closed_side_reversed_rotated_zero_lead.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index1, 0);
    assert!(basic_ab.start_index2 > 0);
    assert_eq!(basic_ba.start_index2, 0);
    assert!(basic_ba.start_index1 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index1, 0);
    assert!(overlap_ab.start_index2 > 0);
    assert_eq!(overlap_ba.start_index2, 0);
    assert!(overlap_ba.start_index1 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this open-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed_rotated_zero_lead
        .iter_vertexes()
        .collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_role_flip_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_pline2_rotated = Polyline::new_closed();
    closed_pline2_rotated.add(1.0, 3.0, 0.0);
    closed_pline2_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_pline2_rotated.add(2.0, 0.0, 0.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_pline2_rotated.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_pline2_rotated.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_pline2_rotated.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&closed_pline2_rotated, &options_ba);
    let default_ab = closed_pline2_rotated.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_pline2_rotated);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 0);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this open-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_pline2_rotated.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_intersect_start_index_rotation_role_flip_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_pline2_rotated = Polyline::new_closed();
    closed_pline2_rotated.add(1.0, 3.0, 0.0);
    closed_pline2_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_pline2_rotated.add(2.0, 0.0, 0.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_pline2_rotated.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_pline2_rotated.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_pline2_rotated.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&closed_pline2_rotated, &options_ba);
    let default_ab = closed_pline2_rotated.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_pline2_rotated);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 0);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this open-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_pline2_rotated.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_pline2_rotated_zero_lead = Polyline::new_closed();
    closed_pline2_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_pline2_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_pline2_rotated_zero_lead.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_pline2_rotated_zero_lead.add(2.0, 0.0, 0.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_pline2_rotated_zero_lead.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_pline2_rotated_zero_lead.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_pline2_rotated_zero_lead.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&closed_pline2_rotated_zero_lead, &options_ba);
    let default_ab = closed_pline2_rotated_zero_lead.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_pline2_rotated_zero_lead);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, 0);
    assert!(basic_ab.start_index1 > 0);
    assert_eq!(basic_ba.start_index1, 0);
    assert!(basic_ba.start_index2 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, 0);
    assert!(overlap_ab.start_index1 > 0);
    assert_eq!(overlap_ba.start_index1, 0);
    assert!(overlap_ba.start_index2 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this open-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_pline2_rotated_zero_lead.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_pline2_rotated_zero_lead = Polyline::new_closed();
    closed_pline2_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_pline2_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_pline2_rotated_zero_lead.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_pline2_rotated_zero_lead.add(2.0, 0.0, 0.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_pline2_rotated_zero_lead.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_pline2_rotated_zero_lead.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_pline2_rotated_zero_lead.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&closed_pline2_rotated_zero_lead, &options_ba);
    let default_ab = closed_pline2_rotated_zero_lead.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_pline2_rotated_zero_lead);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, 0);
    assert!(basic_ab.start_index1 > 0);
    assert_eq!(basic_ba.start_index1, 0);
    assert!(basic_ba.start_index2 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, 0);
    assert!(overlap_ab.start_index1 > 0);
    assert_eq!(overlap_ba.start_index1, 0);
    assert!(overlap_ba.start_index2 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this open-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_pline2_rotated_zero_lead.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed_rotated_zero_lead = Polyline::new_closed();
    closed_side_reversed_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated_zero_lead.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated_zero_lead.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed_rotated_zero_lead
        .iter_vertexes()
        .collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed_rotated_zero_lead.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed
        .find_intersects_opt(&closed_side_reversed_rotated_zero_lead, &options_ab);
    let ba = closed_side_reversed_rotated_zero_lead
        .find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&closed_side_reversed_rotated_zero_lead);
    let default_ba = closed_side_reversed_rotated_zero_lead.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index1, 0);
    assert!(basic_ab.start_index2 > 0);
    assert_eq!(basic_ba.start_index2, 0);
    assert!(basic_ba.start_index1 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index1, 0);
    assert!(overlap_ab.start_index2 > 0);
    assert_eq!(overlap_ba.start_index2, 0);
    assert!(overlap_ba.start_index1 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this open-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed_rotated_zero_lead
        .iter_vertexes()
        .collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_start_index_rotation_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated = Polyline::new_closed();
    normal_closed_side_rotated.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed.find_intersects_opt(&normal_closed_side_rotated, &options_ab);
    let ba = normal_closed_side_rotated.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&normal_closed_side_rotated);
    let default_ba = normal_closed_side_rotated.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 0);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated = Polyline::new_closed();
    normal_closed_side_rotated.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed.find_intersects_opt(&normal_closed_side_rotated, &options_ab);
    let ba = normal_closed_side_rotated.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&normal_closed_side_rotated);
    let default_ba = normal_closed_side_rotated.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 0);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_start_index_rotation_zero_length_lead_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated_zero_lead = Polyline::new_closed();
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated_zero_lead.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated_zero_lead.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed.find_intersects_opt(&normal_closed_side_rotated_zero_lead, &options_ab);
    let ba =
        normal_closed_side_rotated_zero_lead.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&normal_closed_side_rotated_zero_lead);
    let default_ba = normal_closed_side_rotated_zero_lead.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index1, 0);
    assert!(basic_ab.start_index2 > 0);
    assert_eq!(basic_ba.start_index2, 0);
    assert!(basic_ba.start_index1 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index1, 0);
    assert!(overlap_ab.start_index2 > 0);
    assert_eq!(overlap_ba.start_index2, 0);
    assert!(overlap_ba.start_index1 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    // For normal-closed-side branch, role inversion swaps overlap endpoint ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_zero_length_lead_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated_zero_lead = Polyline::new_closed();
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated_zero_lead.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated_zero_lead.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed.find_intersects_opt(&normal_closed_side_rotated_zero_lead, &options_ab);
    let ba =
        normal_closed_side_rotated_zero_lead.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&normal_closed_side_rotated_zero_lead);
    let default_ba = normal_closed_side_rotated_zero_lead.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index1, 0);
    assert!(basic_ab.start_index2 > 0);
    assert_eq!(basic_ba.start_index2, 0);
    assert!(basic_ba.start_index1 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index1, 0);
    assert!(overlap_ab.start_index2 > 0);
    assert_eq!(overlap_ba.start_index2, 0);
    assert!(overlap_ba.start_index1 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    // For normal-closed-side branch, role inversion swaps overlap endpoint ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_options_matrix_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated = Polyline::new_closed();
    normal_closed_side_rotated.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed.find_intersects_opt(&normal_closed_side_rotated, &options_ab);
    let ba = normal_closed_side_rotated.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&normal_closed_side_rotated);
    let default_ba = normal_closed_side_rotated.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated_zero_lead = Polyline::new_closed();
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated_zero_lead.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated_zero_lead.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed.find_intersects_opt(&normal_closed_side_rotated_zero_lead, &options_ab);
    let ba =
        normal_closed_side_rotated_zero_lead.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&normal_closed_side_rotated_zero_lead);
    let default_ba = normal_closed_side_rotated_zero_lead.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index1, 0);
    assert!(basic_ab.start_index2 > 0);
    assert_eq!(basic_ba.start_index2, 0);
    assert!(basic_ba.start_index1 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index1, 0);
    assert!(overlap_ab.start_index2 > 0);
    assert_eq!(overlap_ba.start_index2, 0);
    assert!(overlap_ba.start_index1 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    // For normal-closed-side branch, role inversion swaps overlap endpoint ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_zero_length_lead_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated_zero_lead = Polyline::new_closed();
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated_zero_lead.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated_zero_lead.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed.find_intersects_opt(&normal_closed_side_rotated_zero_lead, &options_ab);
    let ba =
        normal_closed_side_rotated_zero_lead.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&normal_closed_side_rotated_zero_lead);
    let default_ba = normal_closed_side_rotated_zero_lead.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index1, 0);
    assert!(basic_ab.start_index2 > 0);
    assert_eq!(basic_ba.start_index2, 0);
    assert!(basic_ba.start_index1 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index1, 0);
    assert!(overlap_ab.start_index2 > 0);
    assert_eq!(overlap_ba.start_index2, 0);
    assert!(overlap_ba.start_index1 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    // For normal-closed-side branch, role inversion swaps overlap endpoint ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated_zero_lead = Polyline::new_closed();
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated_zero_lead.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated_zero_lead.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated_zero_lead.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed.find_intersects_opt(&normal_closed_side_rotated_zero_lead, &options_ab);
    let ba =
        normal_closed_side_rotated_zero_lead.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&normal_closed_side_rotated_zero_lead);
    let default_ba = normal_closed_side_rotated_zero_lead.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index1, 0);
    assert!(basic_ab.start_index2 > 0);
    assert_eq!(basic_ba.start_index2, 0);
    assert!(basic_ba.start_index1 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index1, 0);
    assert!(overlap_ab.start_index2 > 0);
    assert_eq!(overlap_ba.start_index2, 0);
    assert!(overlap_ba.start_index1 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    // For normal-closed-side branch, role inversion swaps overlap endpoint ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated_zero_lead
        .iter_vertexes()
        .collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_nonzero_open_index_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed_rotated = Polyline::new_closed();
    closed_side_reversed_rotated.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&closed_side_reversed_rotated, &options_ab);
    let ba =
        closed_side_reversed_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&closed_side_reversed_rotated);
    let default_ba = closed_side_reversed_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_intersect_nonzero_open_index_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed_rotated = Polyline::new_closed();
    closed_side_reversed_rotated.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&closed_side_reversed_rotated, &options_ab);
    let ba =
        closed_side_reversed_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&closed_side_reversed_rotated);
    let default_ba = closed_side_reversed_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_side_reversed_nonzero_open_index_options_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed_rotated = Polyline::new_closed();
    closed_side_reversed_rotated.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&closed_side_reversed_rotated, &options_ab);
    let ba =
        closed_side_reversed_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&closed_side_reversed_rotated);
    let default_ba = closed_side_reversed_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_side_reversed_start_index_rotation_options_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed_rotated = Polyline::new_closed();
    closed_side_reversed_rotated.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed.find_intersects_opt(&closed_side_reversed_rotated, &options_ab);
    let ba = closed_side_reversed_rotated.find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&closed_side_reversed_rotated);
    let default_ba = closed_side_reversed_rotated.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 0);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_side_reversed_start_index_rotation_zero_length_lead_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed_rotated_zero_lead = Polyline::new_closed();
    closed_side_reversed_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated_zero_lead.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated_zero_lead.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed_rotated_zero_lead
        .iter_vertexes()
        .collect();

    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed_rotated_zero_lead.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab = open_side_reversed
        .find_intersects_opt(&closed_side_reversed_rotated_zero_lead, &options_ab);
    let ba = closed_side_reversed_rotated_zero_lead
        .find_intersects_opt(&open_side_reversed, &options_ba);
    let default_ab = open_side_reversed.find_intersects(&closed_side_reversed_rotated_zero_lead);
    let default_ba = closed_side_reversed_rotated_zero_lead.find_intersects(&open_side_reversed);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index1, 0);
    assert!(basic_ab.start_index2 > 0);
    assert_eq!(basic_ba.start_index2, 0);
    assert!(basic_ba.start_index1 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index1, 0);
    assert!(overlap_ab.start_index2 > 0);
    assert_eq!(overlap_ba.start_index2, 0);
    assert!(overlap_ba.start_index1 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this closed-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed_rotated_zero_lead
        .iter_vertexes()
        .collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_role_flip_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_side_reversed_rotated = Polyline::new_closed();
    closed_side_reversed_rotated.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated.add(2.0, 0.0, 0.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_side_reversed_rotated.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_side_reversed_rotated.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&closed_side_reversed_rotated, &options_ba);
    let default_ab = closed_side_reversed_rotated.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_side_reversed_rotated);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 0);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this closed-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_side_reversed_start_index_rotation_role_flip_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_side_reversed_rotated = Polyline::new_closed();
    closed_side_reversed_rotated.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated.add(2.0, 0.0, 0.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_side_reversed_rotated.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_side_reversed_rotated.find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed.find_intersects_opt(&closed_side_reversed_rotated, &options_ba);
    let default_ab = closed_side_reversed_rotated.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_side_reversed_rotated);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 0);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this closed-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_side_reversed_rotated_zero_lead = Polyline::new_closed();
    closed_side_reversed_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated_zero_lead.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated_zero_lead.add(2.0, 0.0, 0.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_side_reversed_rotated_zero_lead
        .iter_vertexes()
        .collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_side_reversed_rotated_zero_lead.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_side_reversed_rotated_zero_lead
        .find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed
        .find_intersects_opt(&closed_side_reversed_rotated_zero_lead, &options_ba);
    let default_ab = closed_side_reversed_rotated_zero_lead.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_side_reversed_rotated_zero_lead);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, 0);
    assert!(basic_ab.start_index1 > 0);
    assert_eq!(basic_ba.start_index1, 0);
    assert!(basic_ba.start_index2 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, 0);
    assert!(overlap_ab.start_index1 > 0);
    assert_eq!(overlap_ba.start_index1, 0);
    assert!(overlap_ba.start_index2 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this closed-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_side_reversed_rotated_zero_lead
        .iter_vertexes()
        .collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_side_reversed_start_index_rotation_zero_length_lead_role_flip_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_side_reversed_rotated_zero_lead = Polyline::new_closed();
    closed_side_reversed_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated_zero_lead.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated_zero_lead.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated_zero_lead.add(2.0, 0.0, 0.0);

    let mut open_side_reversed = Polyline::new();
    open_side_reversed.add(2.0, 2.0, -1.0);
    open_side_reversed.add(2.0, 0.0, 0.0);
    open_side_reversed.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_side_reversed_rotated_zero_lead
        .iter_vertexes()
        .collect();
    let open_before: Vec<_> = open_side_reversed.iter_vertexes().collect();

    let closed_aabb = closed_side_reversed_rotated_zero_lead.create_approx_aabb_index();
    let open_aabb = open_side_reversed.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab = closed_side_reversed_rotated_zero_lead
        .find_intersects_opt(&open_side_reversed, &options_ab);
    let ba = open_side_reversed
        .find_intersects_opt(&closed_side_reversed_rotated_zero_lead, &options_ba);
    let default_ab = closed_side_reversed_rotated_zero_lead.find_intersects(&open_side_reversed);
    let default_ba = open_side_reversed.find_intersects(&closed_side_reversed_rotated_zero_lead);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, 0);
    assert!(basic_ab.start_index1 > 0);
    assert_eq!(basic_ba.start_index1, 0);
    assert!(basic_ba.start_index2 > 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, 0);
    assert!(overlap_ab.start_index1 > 0);
    assert_eq!(overlap_ba.start_index1, 0);
    assert!(overlap_ba.start_index2 > 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this closed-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_side_reversed_rotated_zero_lead
        .iter_vertexes()
        .collect();
    let open_after: Vec<_> = open_side_reversed.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_closed_side_reversed_closure_basic_role_flip_nonzero_open_index_options_parity()
{
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed_rotated = Polyline::new_closed();
    closed_side_reversed_rotated.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&closed_side_reversed_rotated, &options_ab);
    let ba =
        closed_side_reversed_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&closed_side_reversed_rotated);
    let default_ba = closed_side_reversed_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_ne!(basic_ab.start_index1, 0);
    assert_ne!(basic_ba.start_index2, 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_ne!(overlap_ab.start_index1, 0);
    assert_ne!(overlap_ba.start_index2, 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this closed-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_side_reversed_role_flip_nonzero_open_index_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut closed_side_reversed_rotated = Polyline::new_closed();
    closed_side_reversed_rotated.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated.add(2.0, 0.0, 0.0);

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let closed_before: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();

    let closed_aabb = closed_side_reversed_rotated.create_approx_aabb_index();
    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        closed_side_reversed_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ab);
    let ba =
        open_side_reversed_nonzero.find_intersects_opt(&closed_side_reversed_rotated, &options_ba);
    let default_ab = closed_side_reversed_rotated.find_intersects(&open_side_reversed_nonzero);
    let default_ba = open_side_reversed_nonzero.find_intersects(&closed_side_reversed_rotated);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 0);
    assert_eq!(basic_ab.start_index2, 1);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this closed-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let closed_after: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_closure_basic_role_flip_nonzero_open_index_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed_rotated = Polyline::new_closed();
    closed_side_reversed_rotated.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&closed_side_reversed_rotated, &options_ab);
    let ba =
        closed_side_reversed_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&closed_side_reversed_rotated);
    let default_ba = closed_side_reversed_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_ne!(basic_ab.start_index1, 0);
    assert_ne!(basic_ba.start_index2, 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_ne!(overlap_ab.start_index1, 0);
    assert_ne!(overlap_ba.start_index2, 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this closed-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_closure_basic_intersect_role_flip_nonzero_open_index_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut closed_side_reversed_rotated = Polyline::new_closed();
    closed_side_reversed_rotated.add(1.0, 3.0, 0.0);
    closed_side_reversed_rotated.add(3.0, 1.0, bulge_from_angle(-FRAC_PI_2));
    closed_side_reversed_rotated.add(2.0, 0.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = closed_side_reversed_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&closed_side_reversed_rotated, &options_ab);
    let ba =
        closed_side_reversed_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&closed_side_reversed_rotated);
    let default_ba = closed_side_reversed_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_ne!(basic_ab.start_index1, 0);
    assert_ne!(basic_ba.start_index2, 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_ne!(overlap_ab.start_index1, 0);
    assert_ne!(overlap_ba.start_index2, 0);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 2.0, 0.0);
    // For this closed-side-reversed branch, role inversion keeps overlap ordering.
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        overlap_ba.point1.x,
        overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        overlap_ba.point2.x,
        overlap_ba.point2.y,
    );
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = closed_side_reversed_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_nonzero_open_index_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated = Polyline::new_closed();
    normal_closed_side_rotated.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&normal_closed_side_rotated, &options_ab);
    let ba =
        normal_closed_side_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&normal_closed_side_rotated);
    let default_ba = normal_closed_side_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_role_flip_nonzero_open_index_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated = Polyline::new_closed();
    normal_closed_side_rotated.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&normal_closed_side_rotated, &options_ab);
    let ba =
        normal_closed_side_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&normal_closed_side_rotated);
    let default_ba = normal_closed_side_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_nonzero_open_index_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated = Polyline::new_closed();
    normal_closed_side_rotated.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&normal_closed_side_rotated, &options_ab);
    let ba =
        normal_closed_side_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&normal_closed_side_rotated);
    let default_ba = normal_closed_side_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_role_flip_nonzero_open_index_options_parity()
 {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated = Polyline::new_closed();
    normal_closed_side_rotated.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&normal_closed_side_rotated, &options_ab);
    let ba =
        normal_closed_side_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&normal_closed_side_rotated);
    let default_ba = normal_closed_side_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}

#[test]
fn cpp_wrap_around_open_side_reversed_normal_closed_side_nonzero_open_index_options_parity() {
    fn assert_point_close(actual_x: f64, actual_y: f64, expected_x: f64, expected_y: f64) {
        assert!(
            (actual_x - expected_x).abs() <= EPS && (actual_y - expected_y).abs() <= EPS,
            "point mismatch: actual=({actual_x}, {actual_y}), expected=({expected_x}, {expected_y})"
        );
    }

    let mut open_side_reversed_nonzero = Polyline::new();
    open_side_reversed_nonzero.add(2.0, 2.0, 0.0);
    open_side_reversed_nonzero.add(2.0, 2.0, -1.0);
    open_side_reversed_nonzero.add(2.0, 0.0, 0.0);
    open_side_reversed_nonzero.add(2.0, -1.0, 0.0);

    let mut normal_closed_side_rotated = Polyline::new_closed();
    normal_closed_side_rotated.add(4.0, 4.0, 0.0);
    normal_closed_side_rotated.add(1.0, 1.0, 1.0);
    normal_closed_side_rotated.add(3.0, 1.0, 0.0);

    let open_before: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_before: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();

    let open_aabb = open_side_reversed_nonzero.create_approx_aabb_index();
    let closed_aabb = normal_closed_side_rotated.create_approx_aabb_index();
    let options_ab = FindIntersectsOptions {
        pline1_aabb_index: Some(&open_aabb),
        pos_equal_eps: EPS,
    };
    let options_ba = FindIntersectsOptions {
        pline1_aabb_index: Some(&closed_aabb),
        pos_equal_eps: EPS,
    };

    let ab =
        open_side_reversed_nonzero.find_intersects_opt(&normal_closed_side_rotated, &options_ab);
    let ba =
        normal_closed_side_rotated.find_intersects_opt(&open_side_reversed_nonzero, &options_ba);
    let default_ab = open_side_reversed_nonzero.find_intersects(&normal_closed_side_rotated);
    let default_ba = normal_closed_side_rotated.find_intersects(&open_side_reversed_nonzero);

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);
    assert_eq!(default_ab.basic_intersects.len(), 1);
    assert_eq!(default_ab.overlapping_intersects.len(), 1);
    assert_eq!(default_ba.basic_intersects.len(), 1);
    assert_eq!(default_ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    let default_basic_ab = default_ab.basic_intersects[0];
    let default_basic_ba = default_ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, 1);
    assert_eq!(basic_ab.start_index2, 0);
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_eq!(basic_ab.start_index1, default_basic_ab.start_index1);
    assert_eq!(basic_ab.start_index2, default_basic_ab.start_index2);
    assert_eq!(basic_ba.start_index1, default_basic_ba.start_index1);
    assert_eq!(basic_ba.start_index2, default_basic_ba.start_index2);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        default_basic_ab.point.x,
        default_basic_ab.point.y,
    );
    assert_point_close(
        basic_ba.point.x,
        basic_ba.point.y,
        default_basic_ba.point.x,
        default_basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    let default_overlap_ab = default_ab.overlapping_intersects[0];
    let default_overlap_ba = default_ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, 1);
    assert_eq!(overlap_ab.start_index2, 1);
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_eq!(overlap_ab.start_index1, default_overlap_ab.start_index1);
    assert_eq!(overlap_ab.start_index2, default_overlap_ab.start_index2);
    assert_eq!(overlap_ba.start_index1, default_overlap_ba.start_index1);
    assert_eq!(overlap_ba.start_index2, default_overlap_ba.start_index2);
    assert_point_close(overlap_ab.point1.x, overlap_ab.point1.y, 2.0, 0.0);
    assert_point_close(overlap_ab.point2.x, overlap_ab.point2.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point1.x, overlap_ba.point1.y, 3.0, 1.0);
    assert_point_close(overlap_ba.point2.x, overlap_ba.point2.y, 2.0, 0.0);
    assert_point_close(
        overlap_ab.point1.x,
        overlap_ab.point1.y,
        default_overlap_ab.point1.x,
        default_overlap_ab.point1.y,
    );
    assert_point_close(
        overlap_ab.point2.x,
        overlap_ab.point2.y,
        default_overlap_ab.point2.x,
        default_overlap_ab.point2.y,
    );
    assert_point_close(
        overlap_ba.point1.x,
        overlap_ba.point1.y,
        default_overlap_ba.point1.x,
        default_overlap_ba.point1.y,
    );
    assert_point_close(
        overlap_ba.point2.x,
        overlap_ba.point2.y,
        default_overlap_ba.point2.x,
        default_overlap_ba.point2.y,
    );

    let open_after: Vec<_> = open_side_reversed_nonzero.iter_vertexes().collect();
    let closed_after: Vec<_> = normal_closed_side_rotated.iter_vertexes().collect();
    assert_eq!(
        open_after, open_before,
        "open-side input mutated by find_intersects_opt"
    );
    assert_eq!(
        closed_after, closed_before,
        "normal-closed-side input mutated by find_intersects_opt"
    );
}
