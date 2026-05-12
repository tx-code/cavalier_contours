mod test_utils;

use cavalier_contours::core::math::Vector2;
use cavalier_contours::polyline::{BooleanOp, PlineSource, Polyline};
use cavalier_contours::{pline_closed, pline_closed_userdata};
use test_utils::{create_property_set, property_sets_match};

const EPS: f64 = 1e-4;

fn cpp_ccw_circle_x_aligned() -> Polyline<f64> {
    pline_closed![(-4.0, 1.0, 1.0), (6.0, 1.0, 1.0)]
}

fn cpp_cw_circle_x_aligned() -> Polyline<f64> {
    pline_closed![(-4.0, 1.0, -1.0), (6.0, 1.0, -1.0)]
}

#[test]
fn cpp_circle_function_metrics_parity() {
    let ccw = cpp_ccw_circle_x_aligned();
    let cw = cpp_cw_circle_x_aligned();

    assert!((ccw.area() - std::f64::consts::PI * 25.0).abs() <= EPS);
    assert!((cw.area() + std::f64::consts::PI * 25.0).abs() <= EPS);
    assert!((ccw.path_length() - std::f64::consts::PI * 10.0).abs() <= EPS);
    assert!((cw.path_length() - std::f64::consts::PI * 10.0).abs() <= EPS);

    let ccw_extents = ccw.extents().unwrap();
    let cw_extents = cw.extents().unwrap();
    let expected = (-4.0, -4.0, 6.0, 6.0);

    assert!((ccw_extents.min_x - expected.0).abs() <= EPS);
    assert!((ccw_extents.min_y - expected.1).abs() <= EPS);
    assert!((ccw_extents.max_x - expected.2).abs() <= EPS);
    assert!((ccw_extents.max_y - expected.3).abs() <= EPS);

    assert!((cw_extents.min_x - expected.0).abs() <= EPS);
    assert!((cw_extents.min_y - expected.1).abs() <= EPS);
    assert!((cw_extents.max_x - expected.2).abs() <= EPS);
    assert!((cw_extents.max_y - expected.3).abs() <= EPS);
}

#[test]
fn cpp_circle_winding_number_parity() {
    let ccw = cpp_ccw_circle_x_aligned();
    let cw = cpp_cw_circle_x_aligned();

    let outside = [
        Vector2::new(-4.01, 1.0),
        Vector2::new(6.01, 1.0),
        Vector2::new(1.0, -4.01),
        Vector2::new(1.0, 6.01),
    ];
    let inside = [
        Vector2::new(1.0, 1.0),
        Vector2::new(-3.99, 1.0),
        Vector2::new(5.99, 1.0),
        Vector2::new(1.0, -3.99),
        Vector2::new(1.0, 5.99),
    ];

    for pt in outside {
        assert_eq!(ccw.winding_number(pt), 0);
        assert_eq!(cw.winding_number(pt), 0);
    }

    for pt in inside {
        assert_eq!(ccw.winding_number(pt), 1);
        assert_eq!(cw.winding_number(pt), -1);
    }
}

fn assert_combine_with_self_invariants(input: &Polyline<f64>) {
    let expected = create_property_set([input], false);

    for op in [BooleanOp::Or, BooleanOp::And] {
        let result = input.boolean(input, op);
        assert_eq!(result.pos_plines.len(), 1);
        assert!(result.neg_plines.is_empty());
        let actual = create_property_set(result.pos_plines.iter().map(|p| &p.pline), false);
        assert!(
            property_sets_match(&actual, &expected),
            "combine-with-self parity mismatch for op={op:?}"
        );
    }

    for op in [BooleanOp::Not, BooleanOp::Xor] {
        let result = input.boolean(input, op);
        assert!(result.pos_plines.is_empty());
        assert!(result.neg_plines.is_empty());
    }
}

#[test]
fn cpp_combine_with_self_invariants_parity() {
    let circle = cpp_ccw_circle_x_aligned();
    let rect = pline_closed_userdata![
        [7],
        (0.0, 0.0, 0.0),
        (20.0, 0.0, 0.0),
        (20.0, 10.0, 0.0),
        (0.0, 10.0, 0.0)
    ];

    assert_combine_with_self_invariants(&circle);
    assert_combine_with_self_invariants(&rect);
}
