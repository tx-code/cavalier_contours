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
