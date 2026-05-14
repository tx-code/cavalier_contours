mod test_utils;

use cavalier_contours::core::math::Vector2;
use cavalier_contours::polyline::{BooleanOp, PlineSource, PlineSourceMut, PlineVertex, Polyline};
use cavalier_contours::{pline_closed, pline_closed_userdata, pline_open};
use test_utils::{PlineProperties, create_property_set, property_sets_match};

const EPS: f64 = PlineProperties::PROP_CMP_EPS;
const CLOSEST_QUERY_EPS: f64 = PlineProperties::POS_EQ_EPS;
const HALF_CIRCLE_RADIUS: f64 = 5.0;
const CIRCLE_RADIUS: f64 = 5.0;
const PROBE_DELTA: f64 = 0.01;
const CIRCLE_INSIDE_DIST_FACTOR: f64 = 0.33;
const CIRCLE_OUTSIDE_DIST_FACTOR: f64 = 1.5;

#[derive(Clone, Copy, Debug)]
struct HalfCircleCaseKey {
    center_x: f64,
    center_y: f64,
    direction: i32,
    is_x_aligned: bool,
    is_closed: bool,
}

#[derive(Clone, Copy, Debug)]
struct ClosestCase {
    query: Vector2<f64>,
    expected_point: Vector2<f64>,
    expected_distance: f64,
    expected_index: usize,
}

#[derive(Clone, Copy, Debug)]
enum CircleAlignment {
    XAxis,
    YAxis,
    Diagonal,
}

#[derive(Clone, Copy, Debug)]
struct CircleCaseKey {
    center_x: f64,
    center_y: f64,
    direction: i32,
    alignment: CircleAlignment,
    reverse: bool,
}

type CircleClosestCase = (Vector2<f64>, Vector2<f64>, f64, Option<usize>);
type HalfOffsetExpectations = (f64, Polyline<f64>, f64, Polyline<f64>);

fn assert_near(actual: f64, expected: f64, context: &str) {
    assert!(
        (actual - expected).abs() <= EPS,
        "{context}: expected {expected}, got {actual}"
    );
}

fn case_label(case: HalfCircleCaseKey) -> String {
    let alignment = if case.is_x_aligned {
        "x_aligned"
    } else {
        "y_aligned"
    };
    let closure = if case.is_closed { "closed" } else { "open" };
    let direction = if case.direction > 0 { "ccw" } else { "cw" };
    format!(
        "{direction}/{alignment}/{closure}/center=({:.1},{:.1})",
        case.center_x, case.center_y
    )
}

fn build_half_circle_case(case: HalfCircleCaseKey) -> Polyline<f64> {
    let mut result = if case.is_closed {
        Polyline::new_closed()
    } else {
        Polyline::new()
    };

    let bulge = if case.direction > 0 { 1.0 } else { -1.0 };
    if case.is_x_aligned {
        result.add(case.center_x - HALF_CIRCLE_RADIUS, case.center_y, bulge);
        result.add(case.center_x + HALF_CIRCLE_RADIUS, case.center_y, 0.0);
    } else {
        result.add(case.center_x, case.center_y - HALF_CIRCLE_RADIUS, bulge);
        result.add(case.center_x, case.center_y + HALF_CIRCLE_RADIUS, 0.0);
    }

    result
}

fn expected_half_circle_extents(case: HalfCircleCaseKey) -> (f64, f64, f64, f64) {
    let mut min_x = case.center_x - HALF_CIRCLE_RADIUS;
    let mut min_y = case.center_y - HALF_CIRCLE_RADIUS;
    let mut max_x = case.center_x + HALF_CIRCLE_RADIUS;
    let mut max_y = case.center_y + HALF_CIRCLE_RADIUS;

    // Mirrors old C++ addHalfCircleCases extent adjustment.
    if case.direction > 0 {
        if case.is_x_aligned {
            max_y -= HALF_CIRCLE_RADIUS;
        } else {
            min_x += HALF_CIRCLE_RADIUS;
        }
    } else if case.is_x_aligned {
        min_y += HALF_CIRCLE_RADIUS;
    } else {
        max_x -= HALF_CIRCLE_RADIUS;
    }

    (min_x, min_y, max_x, max_y)
}

fn build_half_circle_closest_cases(case: HalfCircleCaseKey) -> Vec<ClosestCase> {
    let (min_x, min_y, max_x, max_y) = expected_half_circle_extents(case);
    let cx = case.center_x;
    let cy = case.center_y;
    let end_point_index = if case.is_closed { 1usize } else { 0usize };

    let mut result = Vec::new();

    // addClosestPointOnVertexes in old C++ source.
    if case.is_x_aligned {
        result.push(ClosestCase {
            query: Vector2::new(min_x, cy),
            expected_point: Vector2::new(min_x, cy),
            expected_distance: 0.0,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(max_x, cy),
            expected_point: Vector2::new(max_x, cy),
            expected_distance: 0.0,
            expected_index: end_point_index,
        });
    } else {
        result.push(ClosestCase {
            query: Vector2::new(cx, min_y),
            expected_point: Vector2::new(cx, min_y),
            expected_distance: 0.0,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(cx, max_y),
            expected_point: Vector2::new(cx, max_y),
            expected_distance: 0.0,
            expected_index: end_point_index,
        });
    }

    if case.is_x_aligned {
        let arc_midpoint_y = if case.direction > 0 { min_y } else { max_y };
        result.push(ClosestCase {
            query: Vector2::new(min_x - PROBE_DELTA, cy),
            expected_point: Vector2::new(min_x, cy),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(max_x + PROBE_DELTA, cy),
            expected_point: Vector2::new(max_x, cy),
            expected_distance: PROBE_DELTA,
            expected_index: end_point_index,
        });
        result.push(ClosestCase {
            query: Vector2::new(cx, arc_midpoint_y - PROBE_DELTA),
            expected_point: Vector2::new(cx, arc_midpoint_y),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(cx, arc_midpoint_y + PROBE_DELTA),
            expected_point: Vector2::new(cx, arc_midpoint_y),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        if case.is_closed {
            result.push(ClosestCase {
                query: Vector2::new(cx, cy - PROBE_DELTA),
                expected_point: Vector2::new(cx, cy),
                expected_distance: PROBE_DELTA,
                expected_index: 1,
            });
            result.push(ClosestCase {
                query: Vector2::new(cx, cy + PROBE_DELTA),
                expected_point: Vector2::new(cx, cy),
                expected_distance: PROBE_DELTA,
                expected_index: 1,
            });
        }
    } else {
        let arc_midpoint_x = if case.direction > 0 { max_x } else { min_x };
        result.push(ClosestCase {
            query: Vector2::new(cx, min_y - PROBE_DELTA),
            expected_point: Vector2::new(cx, min_y),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(cx, max_y + PROBE_DELTA),
            expected_point: Vector2::new(cx, max_y),
            expected_distance: PROBE_DELTA,
            expected_index: end_point_index,
        });
        result.push(ClosestCase {
            query: Vector2::new(arc_midpoint_x - PROBE_DELTA, cy),
            expected_point: Vector2::new(arc_midpoint_x, cy),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        result.push(ClosestCase {
            query: Vector2::new(arc_midpoint_x + PROBE_DELTA, cy),
            expected_point: Vector2::new(arc_midpoint_x, cy),
            expected_distance: PROBE_DELTA,
            expected_index: 0,
        });
        if case.is_closed {
            result.push(ClosestCase {
                query: Vector2::new(cx - PROBE_DELTA, cy),
                expected_point: Vector2::new(cx, cy),
                expected_distance: PROBE_DELTA,
                expected_index: 1,
            });
            result.push(ClosestCase {
                query: Vector2::new(cx + PROBE_DELTA, cy),
                expected_point: Vector2::new(cx, cy),
                expected_distance: PROBE_DELTA,
                expected_index: 1,
            });
        }
    }

    result
}

fn half_circle_matrix_cases() -> Vec<HalfCircleCaseKey> {
    let centers = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];
    let mut result = Vec::new();

    for is_closed in [false, true] {
        for (center_x, center_y) in centers {
            for (is_x_aligned, direction) in [(true, 1), (true, -1), (false, 1), (false, -1)] {
                result.push(HalfCircleCaseKey {
                    center_x,
                    center_y,
                    direction,
                    is_x_aligned,
                    is_closed,
                });
            }
        }
    }

    result
}

fn scale_vertex_from_center(
    vertex: PlineVertex<f64>,
    center_x: f64,
    center_y: f64,
    magnitude: f64,
) -> PlineVertex<f64> {
    let dir_x = vertex.x - center_x;
    let dir_y = vertex.y - center_y;
    let dir_len = (dir_x * dir_x + dir_y * dir_y).sqrt();
    PlineVertex::new(
        magnitude * dir_x / dir_len + center_x,
        magnitude * dir_y / dir_len + center_y,
        vertex.bulge,
    )
}

fn polyline_from_vertices(vertices: &[PlineVertex<f64>], is_closed: bool) -> Polyline<f64> {
    let mut result = if is_closed {
        Polyline::new_closed()
    } else {
        Polyline::new()
    };
    for vertex in vertices {
        result.add_vertex(*vertex);
    }
    result
}

fn intersects_at_y(
    center_x: f64,
    center_y: f64,
    radius: f64,
    y: f64,
) -> (Vector2<f64>, Vector2<f64>) {
    let y_term = y - center_y;
    let root = (radius * radius - y_term * y_term).sqrt();
    (
        Vector2::new(center_x + root, y),
        Vector2::new(center_x - root, y),
    )
}

fn intersects_at_x(
    center_x: f64,
    center_y: f64,
    radius: f64,
    x: f64,
) -> (Vector2<f64>, Vector2<f64>) {
    let x_term = x - center_x;
    let root = (radius * radius - x_term * x_term).sqrt();
    (
        Vector2::new(x, center_y + root),
        Vector2::new(x, center_y - root),
    )
}

fn abs_bulge_between_points(
    center_x: f64,
    center_y: f64,
    p1: Vector2<f64>,
    p2: Vector2<f64>,
) -> f64 {
    let a1 = (p1.y - center_y).atan2(p1.x - center_x);
    let a2 = (p2.y - center_y).atan2(p2.x - center_x);
    let mut a_diff = a1 - a2;
    a_diff = (a_diff + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI)
        - std::f64::consts::PI;
    (a_diff / 4.0).tan().abs()
}

fn build_half_circle_offset_expectations(case: HalfCircleCaseKey) -> HalfOffsetExpectations {
    let input = build_half_circle_case(case);
    let (min_x, min_y, max_x, max_y) = expected_half_circle_extents(case);

    let outward_delta = -(case.direction as f64) * 0.25 * HALF_CIRCLE_RADIUS;
    let inward_delta = (case.direction as f64) * 0.4 * HALF_CIRCLE_RADIUS;

    let abs_outward_delta = outward_delta.abs();
    let abs_inward_delta = inward_delta.abs();
    let outward_magnitude = HALF_CIRCLE_RADIUS + abs_outward_delta;
    let inward_magnitude = HALF_CIRCLE_RADIUS - abs_inward_delta;

    let mut outward_vertices: Vec<_> = input
        .iter_vertexes()
        .map(|v| scale_vertex_from_center(v, case.center_x, case.center_y, outward_magnitude))
        .collect();
    let mut inward_vertices: Vec<_> = input
        .iter_vertexes()
        .map(|v| scale_vertex_from_center(v, case.center_x, case.center_y, inward_magnitude))
        .collect();

    if case.is_closed {
        let right_angle_bulge = (std::f64::consts::PI / 8.0).tan();
        if case.is_x_aligned {
            if case.direction > 0 {
                if let Some(last) = outward_vertices.last_mut() {
                    *last = last.with_bulge(right_angle_bulge);
                }
                outward_vertices.push(PlineVertex::new(
                    max_x,
                    case.center_y + abs_outward_delta,
                    0.0,
                ));
                outward_vertices.push(PlineVertex::new(
                    min_x,
                    case.center_y + abs_outward_delta,
                    right_angle_bulge,
                ));

                let y_intr = case.center_y - abs_inward_delta;
                let (intr1, intr2) =
                    intersects_at_y(case.center_x, case.center_y, inward_magnitude, y_intr);
                let abs_bulge =
                    abs_bulge_between_points(case.center_x, case.center_y, intr1, intr2);
                inward_vertices[0] = PlineVertex::new(intr1.x, intr1.y, 0.0);
                inward_vertices[1] = PlineVertex::new(intr2.x, intr2.y, abs_bulge);
            } else {
                if let Some(last) = outward_vertices.last_mut() {
                    *last = last.with_bulge(-right_angle_bulge);
                }
                outward_vertices.push(PlineVertex::new(
                    max_x,
                    case.center_y - abs_outward_delta,
                    0.0,
                ));
                outward_vertices.push(PlineVertex::new(
                    min_x,
                    case.center_y - abs_outward_delta,
                    -right_angle_bulge,
                ));

                let y_intr = case.center_y + abs_inward_delta;
                let (intr1, intr2) =
                    intersects_at_y(case.center_x, case.center_y, inward_magnitude, y_intr);
                let abs_bulge =
                    abs_bulge_between_points(case.center_x, case.center_y, intr1, intr2);
                inward_vertices[0] = PlineVertex::new(intr1.x, intr1.y, 0.0);
                inward_vertices[1] = PlineVertex::new(intr2.x, intr2.y, -abs_bulge);
            }
        } else if case.direction > 0 {
            if let Some(last) = outward_vertices.last_mut() {
                *last = last.with_bulge(right_angle_bulge);
            }
            outward_vertices.push(PlineVertex::new(
                case.center_x - abs_outward_delta,
                max_y,
                0.0,
            ));
            outward_vertices.push(PlineVertex::new(
                case.center_x - abs_outward_delta,
                min_y,
                right_angle_bulge,
            ));

            let x_intr = case.center_x + abs_inward_delta;
            let (intr1, intr2) =
                intersects_at_x(case.center_x, case.center_y, inward_magnitude, x_intr);
            let abs_bulge = abs_bulge_between_points(case.center_x, case.center_y, intr1, intr2);
            inward_vertices[0] = PlineVertex::new(intr1.x, intr1.y, 0.0);
            inward_vertices[1] = PlineVertex::new(intr2.x, intr2.y, abs_bulge);
        } else {
            if let Some(last) = outward_vertices.last_mut() {
                *last = last.with_bulge(-right_angle_bulge);
            }
            outward_vertices.push(PlineVertex::new(
                case.center_x + abs_outward_delta,
                max_y,
                0.0,
            ));
            outward_vertices.push(PlineVertex::new(
                case.center_x + abs_outward_delta,
                min_y,
                -right_angle_bulge,
            ));

            let x_intr = case.center_x - abs_inward_delta;
            let (intr1, intr2) =
                intersects_at_x(case.center_x, case.center_y, inward_magnitude, x_intr);
            let abs_bulge = abs_bulge_between_points(case.center_x, case.center_y, intr1, intr2);
            inward_vertices[0] = PlineVertex::new(intr1.x, intr1.y, 0.0);
            inward_vertices[1] = PlineVertex::new(intr2.x, intr2.y, -abs_bulge);
        }
    }

    (
        outward_delta,
        polyline_from_vertices(&outward_vertices, case.is_closed),
        inward_delta,
        polyline_from_vertices(&inward_vertices, case.is_closed),
    )
}

fn half_circle_collapse_deltas(case: HalfCircleCaseKey) -> [f64; 3] {
    let direction = case.direction as f64;
    let first = if case.is_closed {
        direction * 0.5 * HALF_CIRCLE_RADIUS
    } else {
        direction * HALF_CIRCLE_RADIUS
    };
    [
        first,
        direction * 1.5 * HALF_CIRCLE_RADIUS,
        direction * 2.0 * HALF_CIRCLE_RADIUS,
    ]
}

fn circle_case_label(case: CircleCaseKey) -> String {
    let alignment = match case.alignment {
        CircleAlignment::XAxis => "x_aligned",
        CircleAlignment::YAxis => "y_aligned",
        CircleAlignment::Diagonal => "not_axis_aligned",
    };
    let reverse = if case.reverse { "rev" } else { "fwd" };
    let direction = if case.direction > 0 { "ccw" } else { "cw" };
    format!(
        "{direction}/{alignment}/{reverse}/center=({:.1},{:.1})",
        case.center_x, case.center_y
    )
}

fn build_circle_case(case: CircleCaseKey) -> Polyline<f64> {
    build_circle_case_with_radius(case, CIRCLE_RADIUS)
}

fn build_circle_case_with_radius(case: CircleCaseKey, radius: f64) -> Polyline<f64> {
    let (mut p0, mut p1) = match case.alignment {
        CircleAlignment::XAxis => (
            Vector2::new(case.center_x - radius, case.center_y),
            Vector2::new(case.center_x + radius, case.center_y),
        ),
        CircleAlignment::YAxis => (
            Vector2::new(case.center_x, case.center_y - radius),
            Vector2::new(case.center_x, case.center_y + radius),
        ),
        CircleAlignment::Diagonal => (
            Vector2::new(
                case.center_x + radius * (std::f64::consts::PI / 4.0).cos(),
                case.center_y + radius * (std::f64::consts::PI / 4.0).sin(),
            ),
            Vector2::new(
                case.center_x + radius * (5.0 * std::f64::consts::PI / 4.0).cos(),
                case.center_y + radius * (5.0 * std::f64::consts::PI / 4.0).sin(),
            ),
        ),
    };

    if case.reverse {
        std::mem::swap(&mut p0, &mut p1);
    }

    let bulge = if case.direction > 0 { 1.0 } else { -1.0 };
    let mut pline = Polyline::new_closed();
    pline.add(p0.x, p0.y, bulge);
    pline.add(p1.x, p1.y, bulge);
    pline
}

fn vertex_matches(a: PlineVertex<f64>, b: PlineVertex<f64>) -> bool {
    (a.x - b.x).abs() <= EPS && (a.y - b.y).abs() <= EPS && (a.bulge - b.bulge).abs() <= EPS
}

fn open_vertices_match_exact_order(actual: &Polyline<f64>, expected: &Polyline<f64>) -> bool {
    if actual.vertex_count() != expected.vertex_count()
        || actual.is_closed()
        || expected.is_closed()
    {
        return false;
    }

    for i in 0..expected.vertex_count() {
        if !vertex_matches(actual.at(i), expected.at(i)) {
            return false;
        }
    }

    true
}

fn closed_vertices_match_with_rotation(actual: &Polyline<f64>, expected: &Polyline<f64>) -> bool {
    if actual.vertex_count() != expected.vertex_count()
        || !actual.is_closed()
        || !expected.is_closed()
    {
        return false;
    }

    let n = expected.vertex_count();
    for shift in 0..n {
        let mut all_match = true;
        for i in 0..n {
            let actual_i = (i + shift) % n;
            if !vertex_matches(actual.at(actual_i), expected.at(i)) {
                all_match = false;
                break;
            }
        }
        if all_match {
            return true;
        }
    }
    false
}

fn assert_single_offset_match(actual: &[Polyline<f64>], expected: &Polyline<f64>, context: &str) {
    assert_eq!(
        actual.len(),
        1,
        "{context}: expected exactly one offset polyline, got {}",
        actual.len()
    );
    let actual = &actual[0];
    let actual_props = create_property_set([actual], false);
    let expected_props = create_property_set([expected], false);
    assert!(
        property_sets_match(&actual_props, &expected_props),
        "{context}: offset property mismatch"
    );
    let vertex_match = if expected.is_closed() {
        closed_vertices_match_with_rotation(actual, expected)
    } else {
        open_vertices_match_exact_order(actual, expected)
    };
    assert!(vertex_match, "{context}: offset vertex mismatch");
}

fn circle_matrix_cases() -> Vec<CircleCaseKey> {
    let centers = [(1.0, 1.0), (-1.0, 1.0), (-1.0, -1.0), (1.0, -1.0)];
    let mut result = Vec::new();

    for reverse in [false, true] {
        for (center_x, center_y) in centers {
            for alignment in [
                CircleAlignment::XAxis,
                CircleAlignment::YAxis,
                CircleAlignment::Diagonal,
            ] {
                for direction in [1, -1] {
                    result.push(CircleCaseKey {
                        center_x,
                        center_y,
                        direction,
                        alignment,
                        reverse,
                    });
                }
            }
        }
    }

    result
}

fn build_circle_closest_cases(
    pline: &Polyline<f64>,
    case: CircleCaseKey,
) -> Vec<CircleClosestCase> {
    let mut result = Vec::new();
    let v0 = pline.at(0).pos();
    let v1 = pline.at(1).pos();

    // addClosestPointOnVertexes in old C++ source.
    result.push((v0, v0, 0.0, Some(0)));
    result.push((v1, v1, 0.0, Some(1)));

    // Axis-aligned probes around center in old C++ source.
    result.push((
        Vector2::new(case.center_x - 0.1, case.center_y),
        Vector2::new(case.center_x - CIRCLE_RADIUS, case.center_y),
        CIRCLE_RADIUS - 0.1,
        None,
    ));
    result.push((
        Vector2::new(case.center_x + 0.1, case.center_y),
        Vector2::new(case.center_x + CIRCLE_RADIUS, case.center_y),
        CIRCLE_RADIUS - 0.1,
        None,
    ));
    result.push((
        Vector2::new(case.center_x, case.center_y - 0.1),
        Vector2::new(case.center_x, case.center_y - CIRCLE_RADIUS),
        CIRCLE_RADIUS - 0.1,
        None,
    ));
    result.push((
        Vector2::new(case.center_x, case.center_y + 0.1),
        Vector2::new(case.center_x, case.center_y + CIRCLE_RADIUS),
        CIRCLE_RADIUS - 0.1,
        None,
    ));

    // 45-degree inside/outside probes in old C++ source.
    let inside_dist = CIRCLE_INSIDE_DIST_FACTOR * CIRCLE_RADIUS;
    let outside_dist = CIRCLE_OUTSIDE_DIST_FACTOR * CIRCLE_RADIUS;
    for i in 0..4 {
        let theta = std::f64::consts::PI / 4.0 + (i as f64) * std::f64::consts::PI / 2.0;
        let unit = Vector2::new(theta.cos(), theta.sin());
        let inside = Vector2::new(
            case.center_x + inside_dist * unit.x,
            case.center_y + inside_dist * unit.y,
        );
        let outside = Vector2::new(
            case.center_x + outside_dist * unit.x,
            case.center_y + outside_dist * unit.y,
        );
        let on_circle = Vector2::new(
            case.center_x + CIRCLE_RADIUS * unit.x,
            case.center_y + CIRCLE_RADIUS * unit.y,
        );
        result.push((inside, on_circle, CIRCLE_RADIUS - inside_dist, None));
        result.push((outside, on_circle, outside_dist - CIRCLE_RADIUS, None));
    }

    result
}

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

fn assert_vertex_sequence_match_exact(
    actual: &Polyline<f64>,
    expected: &Polyline<f64>,
    context: &str,
) {
    assert_eq!(
        actual.is_closed(),
        expected.is_closed(),
        "{context}: closure mismatch"
    );
    assert_eq!(
        actual.vertex_count(),
        expected.vertex_count(),
        "{context}: vertex count mismatch"
    );
    for i in 0..expected.vertex_count() {
        assert!(
            vertex_matches(actual.at(i), expected.at(i)),
            "{context}: vertex mismatch at index {i}, actual={:?}, expected={:?}",
            actual.at(i),
            expected.at(i)
        );
    }
}

fn assert_combine_with_self_invariants(input: &Polyline<f64>, context: &str) {
    let expected = create_property_set([input], false);

    for op in [BooleanOp::Or, BooleanOp::And] {
        let result = input.boolean(input, op);
        assert_eq!(
            result.pos_plines.len(),
            1,
            "{context}: expected one positive result for op={op:?}"
        );
        assert!(
            result.neg_plines.is_empty(),
            "{context}: expected empty negative result for op={op:?}"
        );
        let actual = create_property_set(result.pos_plines.iter().map(|p| &p.pline), false);
        assert!(
            property_sets_match(&actual, &expected),
            "{context}: combine-with-self property parity mismatch for op={op:?}"
        );
        assert_vertex_sequence_match_exact(
            &result.pos_plines[0].pline,
            input,
            &format!("{context}: combine-with-self strict vertex parity op={op:?}"),
        );
    }

    for op in [BooleanOp::Not, BooleanOp::Xor] {
        let result = input.boolean(input, op);
        assert!(
            result.pos_plines.is_empty(),
            "{context}: expected empty positive result for op={op:?}"
        );
        assert!(
            result.neg_plines.is_empty(),
            "{context}: expected empty negative result for op={op:?}"
        );
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

    assert_combine_with_self_invariants(&circle, "single-case circle");
    assert_combine_with_self_invariants(&rect, "single-case rectangle");
}

#[test]
fn cpp_generated_closed_shape_matrix_combine_with_self_invariants_parity() {
    for case in circle_matrix_cases() {
        let pline = build_circle_case(case);
        let context = format!("circle matrix {}", circle_case_label(case));
        assert_combine_with_self_invariants(&pline, &context);
    }

    for case in half_circle_matrix_cases()
        .into_iter()
        .filter(|c| c.is_closed)
    {
        let pline = build_half_circle_case(case);
        let context = format!("half-circle matrix {}", case_label(case));
        assert_combine_with_self_invariants(&pline, &context);
    }
}

#[test]
fn cpp_circle_closest_point_parity() {
    let circle = cpp_ccw_circle_x_aligned();

    let cases = [
        // Matches old C++ `addClosestPointTestPt` center +/- 0.1 on Y axis.
        (Vector2::new(1.0, 1.1), Vector2::new(1.0, 6.0), 4.9, None),
        (Vector2::new(1.0, 0.9), Vector2::new(1.0, -4.0), 4.9, None),
    ];

    for (query, expected_point, expected_distance, expected_index) in cases {
        let result = circle.closest_point(query, CLOSEST_QUERY_EPS).unwrap();
        assert!(
            result.seg_point.fuzzy_eq_eps(expected_point, EPS),
            "closest point mismatch for query={query:?}"
        );
        assert!(
            (result.distance - expected_distance).abs() <= EPS,
            "closest distance mismatch for query={query:?}"
        );
        if let Some(i) = expected_index {
            assert_eq!(result.seg_start_index, i, "closest index mismatch");
        }
    }
}

fn cpp_ccw_half_circle_x_aligned_open() -> Polyline<f64> {
    pline_open![(-4.0, 1.0, 1.0), (6.0, 1.0, 0.0)]
}

fn cpp_ccw_half_circle_x_aligned_closed() -> Polyline<f64> {
    pline_closed![(-4.0, 1.0, 1.0), (6.0, 1.0, 0.0)]
}

#[test]
fn cpp_generated_half_circle_matrix_subset_parity() {
    // These expectations follow the generated formulas used in old C++
    // TEST_cavc_pline_function.cpp::addHalfCircleCases for radius=5, center=(1,1),
    // direction=+1, x-aligned.
    let open = cpp_ccw_half_circle_x_aligned_open();
    let closed = cpp_ccw_half_circle_x_aligned_closed();

    let expected_open_length = std::f64::consts::PI * 5.0;
    let expected_closed_length = std::f64::consts::PI * 5.0 + 10.0;
    let expected_closed_area = std::f64::consts::PI * 25.0 / 2.0;

    assert!(open.area().abs() <= EPS);
    assert!((closed.area() - expected_closed_area).abs() <= EPS);
    assert!((open.path_length() - expected_open_length).abs() <= EPS);
    assert!((closed.path_length() - expected_closed_length).abs() <= EPS);

    let open_ext = open.extents().unwrap();
    let closed_ext = closed.extents().unwrap();
    let expected_ext = (-4.0, -4.0, 6.0, 1.0);

    assert!((open_ext.min_x - expected_ext.0).abs() <= EPS);
    assert!((open_ext.min_y - expected_ext.1).abs() <= EPS);
    assert!((open_ext.max_x - expected_ext.2).abs() <= EPS);
    assert!((open_ext.max_y - expected_ext.3).abs() <= EPS);

    assert!((closed_ext.min_x - expected_ext.0).abs() <= EPS);
    assert!((closed_ext.min_y - expected_ext.1).abs() <= EPS);
    assert!((closed_ext.max_x - expected_ext.2).abs() <= EPS);
    assert!((closed_ext.max_y - expected_ext.3).abs() <= EPS);

    assert_eq!(open.winding_number(Vector2::new(1.0, 0.0)), 0);
    assert_eq!(closed.winding_number(Vector2::new(1.0, 0.0)), 1);
}

#[test]
fn cpp_generated_half_circle_full_matrix_metrics_winding_parity() {
    for case in half_circle_matrix_cases() {
        let context = case_label(case);
        let pline = build_half_circle_case(case);
        let (min_x, min_y, max_x, max_y) = expected_half_circle_extents(case);

        let expected_area = if case.is_closed {
            (case.direction as f64) * std::f64::consts::PI * HALF_CIRCLE_RADIUS * HALF_CIRCLE_RADIUS
                / 2.0
        } else {
            0.0
        };
        let expected_path_length = std::f64::consts::PI * HALF_CIRCLE_RADIUS
            + if case.is_closed {
                2.0 * HALF_CIRCLE_RADIUS
            } else {
                0.0
            };

        assert_near(pline.area(), expected_area, &format!("{context}: area"));
        assert_near(
            pline.path_length(),
            expected_path_length,
            &format!("{context}: path_length"),
        );

        let ext = pline.extents().unwrap();
        assert_near(ext.min_x, min_x, &format!("{context}: extents.min_x"));
        assert_near(ext.min_y, min_y, &format!("{context}: extents.min_y"));
        assert_near(ext.max_x, max_x, &format!("{context}: extents.max_x"));
        assert_near(ext.max_y, max_y, &format!("{context}: extents.max_y"));

        let expected_inside_winding = if case.is_closed { case.direction } else { 0 };
        let outside_cases = [
            (Vector2::new(min_x - PROBE_DELTA, case.center_y), 0),
            (Vector2::new(max_x + PROBE_DELTA, case.center_y), 0),
            (Vector2::new(case.center_x, min_y - PROBE_DELTA), 0),
            (Vector2::new(case.center_x, max_y + PROBE_DELTA), 0),
        ];
        for (query, expected) in outside_cases {
            assert_eq!(
                pline.winding_number(query),
                expected,
                "{context}: winding_number outside mismatch for query={query:?}"
            );
        }

        let inside_cases = if case.is_x_aligned {
            [
                Vector2::new(case.center_x, min_y + PROBE_DELTA),
                Vector2::new(case.center_x, max_y - PROBE_DELTA),
            ]
        } else {
            [
                Vector2::new(min_x + PROBE_DELTA, case.center_y),
                Vector2::new(max_x - PROBE_DELTA, case.center_y),
            ]
        };
        for query in inside_cases {
            assert_eq!(
                pline.winding_number(query),
                expected_inside_winding,
                "{context}: winding_number inside mismatch for query={query:?}"
            );
        }
    }
}

#[test]
fn cpp_generated_half_circle_full_matrix_closest_point_strict_index_parity() {
    for case in half_circle_matrix_cases() {
        let context = case_label(case);
        let pline = build_half_circle_case(case);
        let closest_cases = build_half_circle_closest_cases(case);
        for (idx, expected) in closest_cases.iter().enumerate() {
            let result = pline
                .closest_point(expected.query, CLOSEST_QUERY_EPS)
                .unwrap();
            assert!(
                result.seg_point.fuzzy_eq_eps(expected.expected_point, EPS),
                "{context}: closest point mismatch at case #{idx} query={:?}",
                expected.query
            );
            assert_near(
                result.distance,
                expected.expected_distance,
                &format!("{context}: closest distance case #{idx}"),
            );
            assert_eq!(
                result.seg_start_index, expected.expected_index,
                "{context}: closest index mismatch at case #{idx} query={:?}",
                expected.query
            );
        }
    }
}

#[test]
fn cpp_generated_circle_full_matrix_metrics_winding_parity() {
    for case in circle_matrix_cases() {
        let context = circle_case_label(case);
        let pline = build_circle_case(case);

        let expected_area =
            (case.direction as f64) * std::f64::consts::PI * CIRCLE_RADIUS * CIRCLE_RADIUS;
        let expected_path_length = 2.0 * std::f64::consts::PI * CIRCLE_RADIUS;
        assert_near(pline.area(), expected_area, &format!("{context}: area"));
        assert_near(
            pline.path_length(),
            expected_path_length,
            &format!("{context}: path_length"),
        );

        let ext = pline.extents().unwrap();
        assert_near(
            ext.min_x,
            case.center_x - CIRCLE_RADIUS,
            &format!("{context}: extents.min_x"),
        );
        assert_near(
            ext.min_y,
            case.center_y - CIRCLE_RADIUS,
            &format!("{context}: extents.min_y"),
        );
        assert_near(
            ext.max_x,
            case.center_x + CIRCLE_RADIUS,
            &format!("{context}: extents.max_x"),
        );
        assert_near(
            ext.max_y,
            case.center_y + CIRCLE_RADIUS,
            &format!("{context}: extents.max_y"),
        );

        let outside = [
            Vector2::new(case.center_x - CIRCLE_RADIUS - PROBE_DELTA, case.center_y),
            Vector2::new(case.center_x + CIRCLE_RADIUS + PROBE_DELTA, case.center_y),
            Vector2::new(case.center_x, case.center_y - CIRCLE_RADIUS - PROBE_DELTA),
            Vector2::new(case.center_x, case.center_y + CIRCLE_RADIUS + PROBE_DELTA),
        ];
        for query in outside {
            assert_eq!(
                pline.winding_number(query),
                0,
                "{context}: winding_number outside mismatch for query={query:?}"
            );
        }

        let inside_axis = [
            Vector2::new(case.center_x, case.center_y),
            Vector2::new(case.center_x - CIRCLE_RADIUS + PROBE_DELTA, case.center_y),
            Vector2::new(case.center_x + CIRCLE_RADIUS - PROBE_DELTA, case.center_y),
            Vector2::new(case.center_x, case.center_y - CIRCLE_RADIUS + PROBE_DELTA),
            Vector2::new(case.center_x, case.center_y + CIRCLE_RADIUS - PROBE_DELTA),
        ];
        for query in inside_axis {
            assert_eq!(
                pline.winding_number(query),
                case.direction,
                "{context}: winding_number inside-axis mismatch for query={query:?}"
            );
        }

        let inside_dist = CIRCLE_INSIDE_DIST_FACTOR * CIRCLE_RADIUS;
        let outside_dist = CIRCLE_OUTSIDE_DIST_FACTOR * CIRCLE_RADIUS;
        for i in 0..4 {
            let theta = std::f64::consts::PI / 4.0 + (i as f64) * std::f64::consts::PI / 2.0;
            let unit = Vector2::new(theta.cos(), theta.sin());
            let inside = Vector2::new(
                case.center_x + inside_dist * unit.x,
                case.center_y + inside_dist * unit.y,
            );
            let outside = Vector2::new(
                case.center_x + outside_dist * unit.x,
                case.center_y + outside_dist * unit.y,
            );

            assert_eq!(
                pline.winding_number(inside),
                case.direction,
                "{context}: winding_number inside-45 mismatch for query={inside:?}"
            );
            assert_eq!(
                pline.winding_number(outside),
                0,
                "{context}: winding_number outside-45 mismatch for query={outside:?}"
            );
        }
    }
}

#[test]
fn cpp_generated_circle_full_matrix_closest_point_parity() {
    for case in circle_matrix_cases() {
        let context = circle_case_label(case);
        let pline = build_circle_case(case);
        let closest_cases = build_circle_closest_cases(&pline, case);
        for (idx, (query, expected_point, expected_distance, expected_index)) in
            closest_cases.iter().enumerate()
        {
            let result = pline.closest_point(*query, CLOSEST_QUERY_EPS).unwrap();
            assert!(
                result.seg_point.fuzzy_eq_eps(*expected_point, EPS),
                "{context}: closest point mismatch at case #{idx} query={query:?}"
            );
            assert_near(
                result.distance,
                *expected_distance,
                &format!("{context}: closest distance case #{idx}"),
            );
            if let Some(expected_idx) = expected_index {
                assert_eq!(
                    result.seg_start_index, *expected_idx,
                    "{context}: closest index mismatch at case #{idx} query={query:?}"
                );
            }
        }
    }
}

#[test]
fn cpp_generated_circle_full_matrix_parallel_offset_parity() {
    for case in circle_matrix_cases() {
        let context = circle_case_label(case);
        let input = build_circle_case(case);

        let outward_delta = -(case.direction as f64) * 0.25 * CIRCLE_RADIUS;
        let inward_delta = (case.direction as f64) * 0.5 * CIRCLE_RADIUS;

        let outward_expected =
            build_circle_case_with_radius(case, CIRCLE_RADIUS + outward_delta.abs());
        let inward_expected =
            build_circle_case_with_radius(case, CIRCLE_RADIUS - inward_delta.abs());

        let outward_actual = input.parallel_offset(outward_delta);
        assert_single_offset_match(
            &outward_actual,
            &outward_expected,
            &format!("{context}: parallel_offset outward"),
        );

        let inward_actual = input.parallel_offset(inward_delta);
        assert_single_offset_match(
            &inward_actual,
            &inward_expected,
            &format!("{context}: parallel_offset inward"),
        );
    }
}

#[test]
fn cpp_generated_circle_full_matrix_collapsed_offset_parity() {
    for case in circle_matrix_cases() {
        let context = circle_case_label(case);
        let input = build_circle_case(case);

        let collapse_deltas = [
            (case.direction as f64) * CIRCLE_RADIUS,
            (case.direction as f64) * 1.5 * CIRCLE_RADIUS,
            (case.direction as f64) * 2.0 * CIRCLE_RADIUS,
        ];
        for (i, delta) in collapse_deltas.iter().enumerate() {
            let result = input.parallel_offset(*delta);
            assert!(
                result.is_empty(),
                "{context}: collapsed offset case #{i} expected empty, got {} result(s)",
                result.len()
            );
        }
    }
}

#[test]
fn cpp_generated_half_circle_full_matrix_parallel_offset_parity() {
    for case in half_circle_matrix_cases() {
        let context = case_label(case);
        let input = build_half_circle_case(case);
        let (outward_delta, outward_expected, inward_delta, inward_expected) =
            build_half_circle_offset_expectations(case);

        let outward_actual = input.parallel_offset(outward_delta);
        assert_single_offset_match(
            &outward_actual,
            &outward_expected,
            &format!("{context}: parallel_offset outward"),
        );

        let inward_actual = input.parallel_offset(inward_delta);
        assert_single_offset_match(
            &inward_actual,
            &inward_expected,
            &format!("{context}: parallel_offset inward"),
        );
    }
}

#[test]
fn cpp_generated_half_circle_full_matrix_collapsed_offset_parity() {
    for case in half_circle_matrix_cases() {
        let context = case_label(case);
        let input = build_half_circle_case(case);
        let collapse_deltas = half_circle_collapse_deltas(case);

        for (i, delta) in collapse_deltas.iter().enumerate() {
            let result = input.parallel_offset(*delta);
            assert!(
                result.is_empty(),
                "{context}: collapsed offset case #{i} expected empty, got {} result(s)",
                result.len()
            );
        }
    }
}
