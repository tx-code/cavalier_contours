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

    assert_eq!(ab.basic_intersects.len(), 1);
    assert_eq!(ab.overlapping_intersects.len(), 1);
    assert_eq!(ba.basic_intersects.len(), 1);
    assert_eq!(ba.overlapping_intersects.len(), 1);

    let basic_ab = ab.basic_intersects[0];
    let basic_ba = ba.basic_intersects[0];
    assert_eq!(basic_ab.start_index1, basic_ba.start_index2);
    assert_eq!(basic_ab.start_index2, basic_ba.start_index1);
    assert_ne!(basic_ab.start_index1, 0);
    assert_ne!(basic_ba.start_index2, 0);
    assert_point_close(basic_ab.point.x, basic_ab.point.y, 2.0, 2.0);
    assert_point_close(
        basic_ab.point.x,
        basic_ab.point.y,
        basic_ba.point.x,
        basic_ba.point.y,
    );

    let overlap_ab = ab.overlapping_intersects[0];
    let overlap_ba = ba.overlapping_intersects[0];
    assert_eq!(overlap_ab.start_index1, overlap_ba.start_index2);
    assert_eq!(overlap_ab.start_index2, overlap_ba.start_index1);
    assert_ne!(overlap_ab.start_index1, 0);
    assert_ne!(overlap_ba.start_index2, 0);
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
