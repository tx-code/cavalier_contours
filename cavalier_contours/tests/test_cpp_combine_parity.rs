mod test_utils;

use cavalier_contours::pline_closed;
use cavalier_contours::polyline::{BooleanOp, PlineSource, Polyline};
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
