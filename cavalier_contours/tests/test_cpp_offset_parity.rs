mod test_utils;

use cavalier_contours::pline_closed;
use cavalier_contours::polyline::{PlineSource, Polyline};
use test_utils::{PlineProperties, create_property_set, property_sets_match};

fn closed_rectangle() -> Polyline<f64> {
    pline_closed![
        (0.0, 0.0, 0.0),
        (20.0, 0.0, 0.0),
        (20.0, 10.0, 0.0),
        (0.0, 10.0, 0.0)
    ]
}

fn collapsed_rectangle() -> Polyline<f64> {
    pline_closed![
        (0.0, 0.0, 0.0),
        (120.0, 0.0, 0.0),
        (120.0, 40.0, 0.0),
        (0.0, 40.0, 0.0)
    ]
}

#[test]
fn cpp_closed_rectangle_parallel_offset_parity() {
    let input = closed_rectangle();

    let inward_actual = create_property_set(&input.parallel_offset(2.0), false);
    let inward_expected = vec![PlineProperties::new(
        4,
        96.0,
        44.0,
        2.0,
        2.0,
        18.0,
        8.0,
        vec![],
    )];
    assert!(
        property_sets_match(&inward_actual, &inward_expected),
        "C++ parity mismatch for closed_rectangle_inward"
    );

    let outward_actual = create_property_set(&input.parallel_offset(-2.0), false);
    let outward_expected = vec![PlineProperties::new(
        8,
        332.56637061436,
        72.566370614359,
        -2.0,
        -2.0,
        22.0,
        12.0,
        vec![],
    )];
    assert!(
        property_sets_match(&outward_actual, &outward_expected),
        "C++ parity mismatch for closed_rectangle_outward"
    );
}

#[test]
fn cpp_collapsed_rectangle_parallel_offset_parity() {
    let input = collapsed_rectangle();
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
