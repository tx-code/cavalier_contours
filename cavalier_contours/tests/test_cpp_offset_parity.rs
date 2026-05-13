mod test_utils;

use cavalier_contours::polyline::{PlineSource, PlineSourceMut, Polyline};
use cavalier_contours::{pline_closed, pline_open};
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
    let case = &simple_cases()[0];
    let before: Vec<_> = case.input.iter_vertexes().collect();

    let _ = case.input.parallel_offset(case.delta);

    let after: Vec<_> = case.input.iter_vertexes().collect();
    assert_eq!(
        before.len(),
        after.len(),
        "input vertex count changed after offset"
    );
    for (idx, (v0, v1)) in before.iter().zip(after.iter()).enumerate() {
        assert!(
            (v0.x - v1.x).abs() <= EPS
                && (v0.y - v1.y).abs() <= EPS
                && (v0.bulge - v1.bulge).abs() <= EPS,
            "input vertex changed at index {idx}: before={v0:?}, after={v1:?}"
        );
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
    let subject = pline_closed![(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)];
    let clip = pline_closed![
        (3.0, -10.0, 0.0),
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0)
    ];

    let intersects = subject.find_intersects(&clip);
    assert_eq!(intersects.basic_intersects.len(), 4);
    assert!(
        intersects.overlapping_intersects.is_empty(),
        "circle/rectangle parity snapshot expected no overlapping intersections"
    );
}
