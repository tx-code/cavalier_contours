mod test_utils;

use cavalier_contours::pline_closed;
use cavalier_contours::polyline::{
    BooleanOp, PlineBooleanOptions, PlineSource, PlineSourceMut, Polyline,
};
use test_utils::{PlineProperties, aabb_fuzzy_eq_eps, create_property_set};

const EPS: f64 = 1e-4;

fn circle_rectangle_inputs() -> (Polyline<f64>, Polyline<f64>) {
    let subject = pline_closed![(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)];
    let clip = pline_closed![
        (3.0, -10.0, 0.0),
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0)
    ];

    (subject, clip)
}

fn coincident_case1_inputs() -> (Polyline<f64>, Polyline<f64>) {
    let pline_a = pline_closed![
        (-0.105, 0.235, 0.0),
        (-0.095, 0.235, 0.0),
        (-0.095, 0.0, -1.0),
        (-0.105, 0.0, 0.0)
    ];
    let pline_b = pline_closed![
        (-0.25, 0.235, -0.414214),
        (-0.255, 0.24, 0.0),
        (-0.255, 0.29, -0.414214),
        (-0.25, 0.295, 0.0),
        (0.25, 0.295, -0.414214),
        (0.255, 0.29, 0.0),
        (0.255, 0.24, -0.414214),
        (0.25, 0.235, 0.0)
    ];
    (pline_a, pline_b)
}

fn coincident_case2_inputs() -> (Polyline<f64>, Polyline<f64>) {
    let pline_a = pline_closed![
        (0.0, 0.0, 0.0),
        (0.0, 20.0, 0.0),
        (20.0, 20.0, 0.0),
        (20.0, 0.0, 0.0)
    ];
    let pline_b = pline_closed![
        (-2.0, 10.0, 0.0),
        (-2.0, 20.0, 0.0),
        (2.0, 20.0, 0.0),
        (2.0, 25.0, 0.0),
        (4.0, 25.0, 0.0),
        (4.0, 20.0, 0.0),
        (6.0, 20.0, 0.0),
        (6.0, 15.0, 0.0),
        (8.0, 15.0, 0.0),
        (8.0, 20.0, 0.0),
        (10.0, 40.0, 0.0),
        (30.0, 40.0, 0.0),
        (30.0, 20.0, 0.0)
    ];
    (pline_a, pline_b)
}

fn cpp_expected(op: BooleanOp) -> Vec<PlineProperties> {
    match op {
        BooleanOp::Or => vec![PlineProperties::new(
            10,
            109.15381629282,
            52.324068506275,
            0.0,
            -10.0,
            10.0,
            10.0,
            vec![],
        )],
        BooleanOp::Not => vec![
            PlineProperties::new(
                3,
                29.336980664548,
                23.492343031178,
                6.0,
                -3.8989794855664,
                10.0,
                5.898979485566356,
                vec![],
            ),
            PlineProperties::new(
                3,
                19.816835628274,
                20.757946197186,
                0.0,
                -3.582575694955841,
                3.0,
                5.5825756949558,
                vec![],
            ),
        ],
        BooleanOp::And => vec![PlineProperties::new(
            4,
            29.386000046924,
            25.091858029623,
            3.0,
            -4.0,
            6.0,
            6.0,
            vec![],
        )],
        BooleanOp::Xor => vec![
            PlineProperties::new(
                3,
                19.816835628274,
                20.757946197186,
                0.0,
                -3.582575694955841,
                3.0,
                5.5825756949558,
                vec![],
            ),
            PlineProperties::new(
                4,
                -18.306999976538,
                18.582818653767,
                3.0,
                -10.0,
                6.0,
                -3.5825756949558,
                vec![],
            ),
            PlineProperties::new(
                3,
                29.336980664548,
                23.492343031178,
                6.0,
                -3.8989794855664,
                10.0,
                5.898979485566356,
                vec![],
            ),
            PlineProperties::new(
                4,
                -12.306999976538,
                14.582818653767,
                3.0,
                5.5825756949558,
                6.0,
                10.0,
                vec![],
            ),
        ],
    }
}

fn geometry_equal_ignore_vertex_count(a: &PlineProperties, b: &PlineProperties) -> bool {
    (a.area.abs() - b.area.abs()).abs() <= EPS
        && (a.path_length - b.path_length).abs() <= EPS
        && aabb_fuzzy_eq_eps(&a.extents, &b.extents, EPS)
}

fn geometry_sets_match_ignore_vertex_count(
    actual: &[PlineProperties],
    expected: &[PlineProperties],
) -> bool {
    if actual.len() != expected.len() {
        return false;
    }

    expected.iter().all(|exp| {
        actual
            .iter()
            .filter(|act| geometry_equal_ignore_vertex_count(act, exp))
            .count()
            == 1
    })
}

fn sorted_vertex_counts(properties: &[PlineProperties]) -> Vec<usize> {
    let mut counts = properties
        .iter()
        .map(|p| p.vertex_count)
        .collect::<Vec<_>>();
    counts.sort_unstable();
    counts
}

struct CppCombineCase {
    name: &'static str,
    subject: Polyline<f64>,
    clip: Polyline<f64>,
    op: BooleanOp,
    expected: Vec<PlineProperties>,
}

fn cpp_coincident_cases() -> Vec<CppCombineCase> {
    let (c1_a, c1_b) = coincident_case1_inputs();
    let (c2_a, c2_b) = coincident_case2_inputs();

    vec![
        CppCombineCase {
            name: "coincident_case1_union",
            subject: c1_a.clone(),
            clip: c1_b.clone(),
            op: BooleanOp::Or,
            expected: vec![PlineProperties::new(
                12,
                -0.032967809756574,
                1.6071238962168,
                -0.255,
                -0.005,
                0.255,
                0.295,
                vec![],
            )],
        },
        CppCombineCase {
            name: "coincident_case1_excludeAFromB",
            subject: c1_a.clone(),
            clip: c1_b.clone(),
            op: BooleanOp::Not,
            expected: vec![PlineProperties::new(
                4,
                -0.0023892699081699,
                0.49570796326795,
                -0.105,
                -0.005,
                -0.095,
                0.235,
                vec![],
            )],
        },
        CppCombineCase {
            name: "coincident_case1_excludeBFromA",
            subject: c1_b.clone(),
            clip: c1_a.clone(),
            op: BooleanOp::Not,
            expected: vec![PlineProperties::new(
                10,
                -0.030578539848405,
                1.1314159329489,
                -0.255,
                0.235,
                0.255,
                0.295,
                vec![],
            )],
        },
        CppCombineCase {
            name: "coincident_case1_intersect",
            subject: c1_a.clone(),
            clip: c1_b.clone(),
            op: BooleanOp::And,
            expected: vec![],
        },
        CppCombineCase {
            name: "coincident_case1_xor",
            subject: c1_a.clone(),
            clip: c1_b.clone(),
            op: BooleanOp::Xor,
            expected: vec![
                PlineProperties::new(
                    4,
                    -0.0023892699081699,
                    0.49570796326795,
                    -0.105,
                    -0.005,
                    -0.095,
                    0.235,
                    vec![],
                ),
                PlineProperties::new(
                    10,
                    0.030578539848405,
                    1.1314159329489,
                    -0.255,
                    0.235,
                    0.255,
                    0.295,
                    vec![],
                ),
            ],
        },
        CppCombineCase {
            name: "coincident_case2_union",
            subject: c2_a.clone(),
            clip: c2_b.clone(),
            op: BooleanOp::Or,
            expected: vec![PlineProperties::new(
                16,
                -865.0,
                150.17204220292,
                -2.0,
                0.0,
                30.0,
                40.0,
                vec![],
            )],
        },
        CppCombineCase {
            name: "coincident_case2_excludeAFromB",
            subject: c2_a.clone(),
            clip: c2_b.clone(),
            op: BooleanOp::Not,
            expected: vec![
                PlineProperties::new(4, -275.0, 68.4538182678, 0.0, 0.0, 20.0, 16.875, vec![]),
                PlineProperties::new(4, -10.0, 14.0, 6.0, 15.0, 8.0, 20.0, vec![]),
            ],
        },
        CppCombineCase {
            name: "coincident_case2_excludeBFromA",
            subject: c2_b.clone(),
            clip: c2_a.clone(),
            op: BooleanOp::Not,
            expected: vec![
                PlineProperties::new(4, -19.375, 23.47038182678, -2.0, 10.0, 0.0, 20.0, vec![]),
                PlineProperties::new(
                    6,
                    -435.625,
                    85.701660376142,
                    8.0,
                    16.875,
                    30.0,
                    40.0,
                    vec![],
                ),
                PlineProperties::new(4, -10.0, 14.0, 2.0, 20.0, 4.0, 25.0, vec![]),
            ],
        },
        CppCombineCase {
            name: "coincident_case2_intersect",
            subject: c2_a.clone(),
            clip: c2_b.clone(),
            op: BooleanOp::And,
            expected: vec![PlineProperties::new(
                10,
                -115.0,
                63.4538182678,
                0.0,
                10.625,
                20.0,
                20.0,
                vec![],
            )],
        },
        CppCombineCase {
            name: "coincident_case2_xor",
            subject: c2_a,
            clip: c2_b,
            op: BooleanOp::Xor,
            expected: vec![
                PlineProperties::new(4, -19.375, 23.47038182678, -2.0, 10.0, 0.0, 20.0, vec![]),
                PlineProperties::new(
                    6,
                    -435.625,
                    85.701660376142,
                    8.0,
                    16.875,
                    30.0,
                    40.0,
                    vec![],
                ),
                PlineProperties::new(4, -10.0, 14.0, 2.0, 20.0, 4.0, 25.0, vec![]),
                PlineProperties::new(4, 275.0, 68.4538182678, 0.0, 0.0, 20.0, 16.875, vec![]),
                PlineProperties::new(4, 10.0, 14.0, 6.0, 15.0, 8.0, 20.0, vec![]),
            ],
        },
    ]
}

#[test]
fn cpp_circle_rectangle_geometry_parity_holds() {
    let (subject, clip) = circle_rectangle_inputs();

    for op in [
        BooleanOp::Or,
        BooleanOp::Not,
        BooleanOp::And,
        BooleanOp::Xor,
    ] {
        let result = subject.boolean(&clip, op);
        let actual = create_property_set(result.pos_plines.iter().map(|r| &r.pline), false);
        let expected = cpp_expected(op);

        assert!(
            geometry_sets_match_ignore_vertex_count(&actual, &expected),
            "geometry parity mismatch for op={op:?}\nactual={actual:?}\nexpected={expected:?}"
        );
    }
}

#[test]
fn cpp_circle_rectangle_topology_delta_snapshot() {
    let (subject, clip) = circle_rectangle_inputs();

    let union_actual = create_property_set(
        subject
            .boolean(&clip, BooleanOp::Or)
            .pos_plines
            .iter()
            .map(|r| &r.pline),
        false,
    );
    let exclude_actual = create_property_set(
        subject
            .boolean(&clip, BooleanOp::Not)
            .pos_plines
            .iter()
            .map(|r| &r.pline),
        false,
    );
    let intersect_actual = create_property_set(
        subject
            .boolean(&clip, BooleanOp::And)
            .pos_plines
            .iter()
            .map(|r| &r.pline),
        false,
    );
    let xor_actual = create_property_set(
        subject
            .boolean(&clip, BooleanOp::Xor)
            .pos_plines
            .iter()
            .map(|r| &r.pline),
        false,
    );

    let union_expected = cpp_expected(BooleanOp::Or);
    let exclude_expected = cpp_expected(BooleanOp::Not);
    let intersect_expected = cpp_expected(BooleanOp::And);
    let xor_expected = cpp_expected(BooleanOp::Xor);

    assert_eq!(sorted_vertex_counts(&union_actual), vec![8]);
    assert_eq!(sorted_vertex_counts(&exclude_actual), vec![2, 2]);
    assert_eq!(sorted_vertex_counts(&intersect_actual), vec![4]);
    assert_eq!(sorted_vertex_counts(&xor_actual), vec![2, 2, 4, 4]);

    assert_eq!(sorted_vertex_counts(&union_expected), vec![10]);
    assert_eq!(sorted_vertex_counts(&exclude_expected), vec![3, 3]);
    assert_eq!(sorted_vertex_counts(&intersect_expected), vec![4]);
    assert_eq!(sorted_vertex_counts(&xor_expected), vec![3, 3, 4, 4]);
}

#[test]
fn cpp_circle_rectangle_combine_does_not_modify_input() {
    let (subject, clip) = circle_rectangle_inputs();

    for op in [
        BooleanOp::Or,
        BooleanOp::Not,
        BooleanOp::And,
        BooleanOp::Xor,
    ] {
        let subject_before: Vec<_> = subject.iter_vertexes().collect();
        let clip_before: Vec<_> = clip.iter_vertexes().collect();

        let _ = subject.boolean(&clip, op);

        let subject_after: Vec<_> = subject.iter_vertexes().collect();
        let clip_after: Vec<_> = clip.iter_vertexes().collect();

        assert_eq!(
            subject_after, subject_before,
            "subject modified for op={op:?}"
        );
        assert_eq!(clip_after, clip_before, "clip modified for op={op:?}");
    }
}

#[test]
fn cpp_coincident_combine_does_not_modify_input() {
    for case in cpp_coincident_cases() {
        let subject_before: Vec<_> = case.subject.iter_vertexes().collect();
        let clip_before: Vec<_> = case.clip.iter_vertexes().collect();

        let _ = case.subject.boolean(&case.clip, case.op);

        let subject_after: Vec<_> = case.subject.iter_vertexes().collect();
        let clip_after: Vec<_> = case.clip.iter_vertexes().collect();

        assert_eq!(
            subject_after, subject_before,
            "subject modified for case={} op={:?}",
            case.name, case.op
        );
        assert_eq!(
            clip_after, clip_before,
            "clip modified for case={} op={:?}",
            case.name, case.op
        );
    }
}

#[test]
fn cpp_circle_rectangle_commutative_role_flip_matrix_parity() {
    fn reversed(mut pline: Polyline<f64>) -> Polyline<f64> {
        pline.invert_direction_mut();
        pline
    }

    let (subject, clip) = circle_rectangle_inputs();
    let subject_reversed = reversed(subject.clone());
    let clip_reversed = reversed(clip.clone());

    let orientation_pairs = [
        (&subject, &clip),
        (&subject, &clip_reversed),
        (&subject_reversed, &clip),
        (&subject_reversed, &clip_reversed),
    ];

    for op in [BooleanOp::Or, BooleanOp::And, BooleanOp::Xor] {
        let expected = cpp_expected(op);

        for (a, b) in orientation_pairs {
            let ab =
                create_property_set(a.boolean(b, op).pos_plines.iter().map(|r| &r.pline), false);
            let ba =
                create_property_set(b.boolean(a, op).pos_plines.iter().map(|r| &r.pline), false);

            assert!(
                geometry_sets_match_ignore_vertex_count(&ab, &expected),
                "AB mismatch for op={op:?}, ab={ab:?}, expected={expected:?}"
            );
            assert!(
                geometry_sets_match_ignore_vertex_count(&ba, &expected),
                "BA mismatch for op={op:?}, ba={ba:?}, expected={expected:?}"
            );
            assert!(
                geometry_sets_match_ignore_vertex_count(&ab, &ba),
                "AB/BA role-flip mismatch for op={op:?}, ab={ab:?}, ba={ba:?}"
            );
        }
    }
}

#[test]
fn cpp_circle_rectangle_commutative_start_index_rotation_matrix_parity() {
    fn reversed(mut pline: Polyline<f64>) -> Polyline<f64> {
        pline.invert_direction_mut();
        pline
    }

    let (subject, clip) = circle_rectangle_inputs();
    let subject_rotated = pline_closed![(10.0, 1.0, 1.0), (0.0, 1.0, 1.0)];
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

    for op in [BooleanOp::Or, BooleanOp::And, BooleanOp::Xor] {
        let expected = cpp_expected(op);

        for a in &subject_variants {
            for b in &clip_variants {
                let ab = create_property_set(
                    a.boolean(b, op).pos_plines.iter().map(|r| &r.pline),
                    false,
                );
                let ba = create_property_set(
                    b.boolean(a, op).pos_plines.iter().map(|r| &r.pline),
                    false,
                );

                assert!(
                    geometry_sets_match_ignore_vertex_count(&ab, &expected),
                    "AB mismatch for op={op:?}, ab={ab:?}, expected={expected:?}"
                );
                assert!(
                    geometry_sets_match_ignore_vertex_count(&ba, &expected),
                    "BA mismatch for op={op:?}, ba={ba:?}, expected={expected:?}"
                );
                assert!(
                    geometry_sets_match_ignore_vertex_count(&ab, &ba),
                    "AB/BA mismatch for op={op:?}, ab={ab:?}, ba={ba:?}"
                );
            }
        }
    }
}

#[test]
fn cpp_combine_expected_subtracted_empty_parity() {
    // Source-aligned with old C++ `combine_plines_test` expectations in
    // TEST_cavc_combine_plines.cpp where simple/circle-rectangle and coincident
    // fixtures define empty expectedSubtracted lists.
    let (subject, clip) = circle_rectangle_inputs();
    for op in [
        BooleanOp::Or,
        BooleanOp::Not,
        BooleanOp::And,
        BooleanOp::Xor,
    ] {
        let result = subject.boolean(&clip, op);
        assert!(
            result.neg_plines.is_empty(),
            "expected empty neg_plines for circle/rectangle op={op:?}, got {:?}",
            result.neg_plines
        );
    }

    for case in cpp_coincident_cases() {
        let result = case.subject.boolean(&case.clip, case.op);
        assert!(
            result.neg_plines.is_empty(),
            "expected empty neg_plines for case={} op={:?}, got {:?}",
            case.name,
            case.op,
            result.neg_plines
        );
    }
}

#[test]
fn cpp_coincident_matrix_geometry_parity_holds() {
    for case in cpp_coincident_cases() {
        let actual = create_property_set(
            case.subject
                .boolean(&case.clip, case.op)
                .pos_plines
                .iter()
                .map(|r| &r.pline),
            false,
        );
        assert!(
            geometry_sets_match_ignore_vertex_count(&actual, &case.expected),
            "coincident geometry parity mismatch for case={} op={:?}\nactual={actual:?}\nexpected={:?}",
            case.name,
            case.op,
            case.expected
        );
    }
}

#[test]
fn cpp_coincident_commutative_role_flip_matrix_parity() {
    fn reversed(mut pline: Polyline<f64>) -> Polyline<f64> {
        pline.invert_direction_mut();
        pline
    }

    for (case_prefix, inputs) in [
        (
            "coincident_case1_",
            coincident_case1_inputs as fn() -> (Polyline<f64>, Polyline<f64>),
        ),
        (
            "coincident_case2_",
            coincident_case2_inputs as fn() -> (Polyline<f64>, Polyline<f64>),
        ),
    ] {
        let case_expectations: Vec<_> = cpp_coincident_cases()
            .into_iter()
            .filter(|c| c.name.starts_with(case_prefix))
            .collect();

        let (subject, clip) = inputs();
        let subject_reversed = reversed(subject.clone());
        let clip_reversed = reversed(clip.clone());

        let orientation_pairs = [
            (&subject, &clip),
            (&subject, &clip_reversed),
            (&subject_reversed, &clip),
            (&subject_reversed, &clip_reversed),
        ];

        for op in [BooleanOp::Or, BooleanOp::And, BooleanOp::Xor] {
            let expected = &case_expectations
                .iter()
                .find(|c| c.op == op)
                .unwrap_or_else(|| {
                    panic!("missing expected coincident case for {case_prefix}{op:?}")
                })
                .expected;

            for (a, b) in orientation_pairs {
                let ab = create_property_set(
                    a.boolean(b, op).pos_plines.iter().map(|r| &r.pline),
                    false,
                );
                let ba = create_property_set(
                    b.boolean(a, op).pos_plines.iter().map(|r| &r.pline),
                    false,
                );

                assert!(
                    geometry_sets_match_ignore_vertex_count(&ab, expected),
                    "AB mismatch for case_prefix={case_prefix}, op={op:?}, ab={ab:?}, expected={expected:?}"
                );
                assert!(
                    geometry_sets_match_ignore_vertex_count(&ba, expected),
                    "BA mismatch for case_prefix={case_prefix}, op={op:?}, ba={ba:?}, expected={expected:?}"
                );
                assert!(
                    geometry_sets_match_ignore_vertex_count(&ab, &ba),
                    "AB/BA role-flip mismatch for case_prefix={case_prefix}, op={op:?}, ab={ab:?}, ba={ba:?}"
                );
            }
        }
    }
}

#[test]
fn cpp_coincident_commutative_start_index_rotation_matrix_parity() {
    fn reversed(mut pline: Polyline<f64>) -> Polyline<f64> {
        pline.invert_direction_mut();
        pline
    }

    fn rotate_closed_start(pline: &Polyline<f64>, shift: usize) -> Polyline<f64> {
        let verts: Vec<_> = pline.iter_vertexes().collect();
        let len = verts.len();
        let shift = shift % len;
        let mut result = Polyline::new_closed();
        for i in 0..len {
            let v = verts[(i + shift) % len];
            result.add(v.x, v.y, v.bulge);
        }
        result
    }

    for (case_prefix, inputs) in [
        (
            "coincident_case1_",
            coincident_case1_inputs as fn() -> (Polyline<f64>, Polyline<f64>),
        ),
        (
            "coincident_case2_",
            coincident_case2_inputs as fn() -> (Polyline<f64>, Polyline<f64>),
        ),
    ] {
        let case_expectations: Vec<_> = cpp_coincident_cases()
            .into_iter()
            .filter(|c| c.name.starts_with(case_prefix))
            .collect();

        let (subject, clip) = inputs();
        let subject_rotated = rotate_closed_start(&subject, 1);
        let clip_rotated = rotate_closed_start(&clip, 2);

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

        for op in [BooleanOp::Or, BooleanOp::And, BooleanOp::Xor] {
            let expected_case = case_expectations
                .iter()
                .find(|c| c.op == op)
                .unwrap_or_else(|| {
                    panic!("missing expected case for prefix={case_prefix} op={op:?}")
                });

            for a in &subject_variants {
                for b in &clip_variants {
                    let ab = create_property_set(
                        a.boolean(b, op).pos_plines.iter().map(|r| &r.pline),
                        false,
                    );
                    let ba = create_property_set(
                        b.boolean(a, op).pos_plines.iter().map(|r| &r.pline),
                        false,
                    );

                    assert!(
                        geometry_sets_match_ignore_vertex_count(&ab, &expected_case.expected),
                        "AB mismatch for case_prefix={case_prefix} op={op:?}, ab={ab:?}, expected={:?}",
                        expected_case.expected
                    );
                    assert!(
                        geometry_sets_match_ignore_vertex_count(&ba, &expected_case.expected),
                        "BA mismatch for case_prefix={case_prefix} op={op:?}, ba={ba:?}, expected={:?}",
                        expected_case.expected
                    );
                    assert!(
                        geometry_sets_match_ignore_vertex_count(&ab, &ba),
                        "AB/BA mismatch for case_prefix={case_prefix} op={op:?}, ab={ab:?}, ba={ba:?}"
                    );
                }
            }
        }
    }
}

#[test]
fn cpp_coincident_not_complementary_role_flip_matrix_parity() {
    fn reversed(mut pline: Polyline<f64>) -> Polyline<f64> {
        pline.invert_direction_mut();
        pline
    }

    for (case_prefix, inputs) in [
        (
            "coincident_case1_",
            coincident_case1_inputs as fn() -> (Polyline<f64>, Polyline<f64>),
        ),
        (
            "coincident_case2_",
            coincident_case2_inputs as fn() -> (Polyline<f64>, Polyline<f64>),
        ),
    ] {
        let case_expectations: Vec<_> = cpp_coincident_cases()
            .into_iter()
            .filter(|c| c.name.starts_with(case_prefix))
            .collect();

        let expected_exclude_a_from_b = &case_expectations
            .iter()
            .find(|c| c.name.contains("excludeAFromB"))
            .unwrap_or_else(|| panic!("missing excludeAFromB case for {case_prefix}"))
            .expected;
        let expected_exclude_b_from_a = &case_expectations
            .iter()
            .find(|c| c.name.contains("excludeBFromA"))
            .unwrap_or_else(|| panic!("missing excludeBFromA case for {case_prefix}"))
            .expected;

        let (subject, clip) = inputs();
        let subject_reversed = reversed(subject.clone());
        let clip_reversed = reversed(clip.clone());

        let orientation_pairs = [
            (&subject, &clip),
            (&subject, &clip_reversed),
            (&subject_reversed, &clip),
            (&subject_reversed, &clip_reversed),
        ];

        for (a, b) in orientation_pairs {
            let ab = create_property_set(
                a.boolean(b, BooleanOp::Not)
                    .pos_plines
                    .iter()
                    .map(|r| &r.pline),
                false,
            );
            let ba = create_property_set(
                b.boolean(a, BooleanOp::Not)
                    .pos_plines
                    .iter()
                    .map(|r| &r.pline),
                false,
            );

            assert!(
                geometry_sets_match_ignore_vertex_count(&ab, expected_exclude_a_from_b),
                "AB NOT mismatch for case_prefix={case_prefix}, ab={ab:?}, expected={expected_exclude_a_from_b:?}"
            );
            assert!(
                geometry_sets_match_ignore_vertex_count(&ba, expected_exclude_b_from_a),
                "BA NOT mismatch for case_prefix={case_prefix}, ba={ba:?}, expected={expected_exclude_b_from_a:?}"
            );
        }
    }
}

#[test]
fn cpp_coincident_case1_intersect_with_collapsed_filter_matches_cpp_empty() {
    let (subject, clip) = coincident_case1_inputs();
    let options = PlineBooleanOptions {
        collapsed_area_eps: Some(EPS),
        ..Default::default()
    };
    let actual = create_property_set(
        subject
            .boolean_opt(&clip, BooleanOp::And, &options)
            .pos_plines
            .iter()
            .map(|r| &r.pline),
        false,
    );
    assert!(
        actual.is_empty(),
        "expected empty intersect with collapsed_area_eps filter, got {actual:?}"
    );
}

#[test]
fn cpp_combine_with_self_reverse_mix_invariants() {
    let pline = pline_closed![
        (27.554688, 1.0, 0.0),
        (27.554688, 0.75, 0.414214),
        (27.804688, 0.5, 0.0),
        (32.195313, 0.5, 0.414214),
        (32.445313, 0.75, 0.0),
        (32.445313, 1.0, 0.414214),
        (32.195313, 1.25, 0.0),
        (31.5, 1.25, -0.414214),
        (31.0, 1.75, 0.0),
        (29.0, 1.75, -0.414214),
        (28.5, 1.25, 0.0),
        (27.804688, 1.25, 0.414214)
    ];

    let mut rev_pline = pline.clone();
    rev_pline.invert_direction_mut();

    let expected_fwd = create_property_set([&pline], false);
    let expected_rev = create_property_set([&rev_pline], false);

    let union_fwd_result = pline.boolean(&pline, BooleanOp::Or);
    assert!(
        union_fwd_result.neg_plines.is_empty(),
        "union self expected empty neg_plines for forward orientation, got {:?}",
        union_fwd_result.neg_plines
    );
    let union_fwd =
        create_property_set(union_fwd_result.pos_plines.iter().map(|r| &r.pline), false);
    assert!(
        geometry_sets_match_ignore_vertex_count(&union_fwd, &expected_fwd),
        "union self mismatch for forward orientation: {union_fwd:?}"
    );

    let union_rev_result = rev_pline.boolean(&rev_pline, BooleanOp::Or);
    assert!(
        union_rev_result.neg_plines.is_empty(),
        "union self expected empty neg_plines for reversed orientation, got {:?}",
        union_rev_result.neg_plines
    );
    let union_rev =
        create_property_set(union_rev_result.pos_plines.iter().map(|r| &r.pline), false);
    assert!(
        geometry_sets_match_ignore_vertex_count(&union_rev, &expected_rev),
        "union self mismatch for reversed orientation: {union_rev:?}"
    );

    let intersect_fwd_result = pline.boolean(&pline, BooleanOp::And);
    assert!(
        intersect_fwd_result.neg_plines.is_empty(),
        "intersect self expected empty neg_plines for forward orientation, got {:?}",
        intersect_fwd_result.neg_plines
    );
    let intersect_fwd = create_property_set(
        intersect_fwd_result.pos_plines.iter().map(|r| &r.pline),
        false,
    );
    assert!(
        geometry_sets_match_ignore_vertex_count(&intersect_fwd, &expected_fwd),
        "intersect self mismatch for forward orientation: {intersect_fwd:?}"
    );

    let intersect_rev_result = rev_pline.boolean(&rev_pline, BooleanOp::And);
    assert!(
        intersect_rev_result.neg_plines.is_empty(),
        "intersect self expected empty neg_plines for reversed orientation, got {:?}",
        intersect_rev_result.neg_plines
    );
    let intersect_rev = create_property_set(
        intersect_rev_result.pos_plines.iter().map(|r| &r.pline),
        false,
    );
    assert!(
        geometry_sets_match_ignore_vertex_count(&intersect_rev, &expected_rev),
        "intersect self mismatch for reversed orientation: {intersect_rev:?}"
    );

    for (lhs, rhs, label) in [
        (&pline, &pline, "fwd/fwd"),
        (&rev_pline, &rev_pline, "rev/rev"),
        (&rev_pline, &pline, "rev/fwd"),
        (&pline, &rev_pline, "fwd/rev"),
    ] {
        let exclude_result = lhs.boolean(rhs, BooleanOp::Not);
        assert!(
            exclude_result.neg_plines.is_empty(),
            "exclude self expected empty neg_plines for {label}, got {:?}",
            exclude_result.neg_plines
        );
        let exclude =
            create_property_set(exclude_result.pos_plines.iter().map(|r| &r.pline), false);
        assert!(
            exclude.is_empty(),
            "exclude self expected empty for {label}, got {exclude:?}"
        );

        let xor_result = lhs.boolean(rhs, BooleanOp::Xor);
        assert!(
            xor_result.neg_plines.is_empty(),
            "xor self expected empty neg_plines for {label}, got {:?}",
            xor_result.neg_plines
        );
        let xor = create_property_set(xor_result.pos_plines.iter().map(|r| &r.pline), false);
        assert!(
            xor.is_empty(),
            "xor self expected empty for {label}, got {xor:?}"
        );
    }
}
