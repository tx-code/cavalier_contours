use cavalier_contours::assert_fuzzy_eq;
use cavalier_contours_ffi::*;
use std::ptr;

fn create_pline(vertexes: &[(f64, f64, f64)], is_closed: bool) -> *mut cavc_pline {
    let mut buffer = Vec::with_capacity(vertexes.len());
    for &(x, y, bulge) in vertexes {
        buffer.push(cavc_vertex::new(x, y, bulge));
    }

    let mut result = ptr::null();
    let err = unsafe {
        cavc_pline_create(
            buffer.as_ptr(),
            buffer.len() as u32,
            if is_closed { 1 } else { 0 },
            &mut result,
        )
    };
    assert_eq!(err, 0);

    result as *mut _
}

fn compare_vertexes(actual: &[cavc_vertex], expected: &[cavc_vertex]) {
    assert_eq!(expected.len(), actual.len());

    for (index, vertex) in actual.iter().enumerate() {
        assert_fuzzy_eq!(vertex.x, expected[index].x);
        assert_fuzzy_eq!(vertex.y, expected[index].y);
        assert_fuzzy_eq!(vertex.bulge, expected[index].bulge);
    }
}

#[derive(Debug, Copy, Clone)]
struct PlineProps {
    vertex_count: u32,
    area: f64,
    path_length: f64,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl PlineProps {
    fn new(
        vertex_count: u32,
        area: f64,
        path_length: f64,
        min_x: f64,
        min_y: f64,
        max_x: f64,
        max_y: f64,
    ) -> Self {
        Self {
            vertex_count,
            area,
            path_length,
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    fn fuzzy_eq_ignore_area_sign(&self, other: &Self, eps: f64) -> bool {
        self.vertex_count == other.vertex_count
            && (self.area.abs() - other.area.abs()).abs() <= eps
            && (self.path_length - other.path_length).abs() <= eps
            && (self.min_x - other.min_x).abs() <= eps
            && (self.min_y - other.min_y).abs() <= eps
            && (self.max_x - other.max_x).abs() <= eps
            && (self.max_y - other.max_y).abs() <= eps
    }
}

fn pline_props(pline: *const cavc_pline) -> PlineProps {
    let mut vertex_count = u32::MAX;
    let mut area = f64::NAN;
    let mut path_length = f64::NAN;
    let mut min_x = f64::NAN;
    let mut min_y = f64::NAN;
    let mut max_x = f64::NAN;
    let mut max_y = f64::NAN;

    unsafe {
        assert_eq!(cavc_pline_get_vertex_count(pline, &mut vertex_count), 0);
        assert_eq!(cavc_pline_eval_area(pline, &mut area), 0);
        assert_eq!(cavc_pline_eval_path_length(pline, &mut path_length), 0);
        assert_eq!(
            cavc_pline_eval_extents(pline, &mut min_x, &mut min_y, &mut max_x, &mut max_y),
            0
        );
    }

    PlineProps::new(vertex_count, area, path_length, min_x, min_y, max_x, max_y)
}

fn plinelist_props(plinelist: *const cavc_plinelist) -> Vec<PlineProps> {
    let mut count = u32::MAX;
    unsafe {
        assert_eq!(cavc_plinelist_get_count(plinelist, &mut count), 0);
    }

    let mut result = Vec::with_capacity(count as usize);
    for i in 0..count {
        let mut pline = ptr::null();
        unsafe {
            assert_eq!(cavc_plinelist_get_pline(plinelist, i, &mut pline), 0);
        }
        result.push(pline_props(pline));
    }

    result
}

fn props_set_match_ignore_area_sign(
    actual: &[PlineProps],
    expected: &[PlineProps],
    eps: f64,
) -> bool {
    if actual.len() != expected.len() {
        return false;
    }

    expected.iter().all(|exp| {
        actual
            .iter()
            .filter(|act| act.fuzzy_eq_ignore_area_sign(exp, eps))
            .count()
            == 1
    })
}

fn run_boolean_props(
    pline1: *const cavc_pline,
    pline2: *const cavc_pline,
    operation: u32,
) -> (Vec<PlineProps>, Vec<PlineProps>) {
    let mut pos_plines = ptr::null();
    let mut neg_plines = ptr::null();

    unsafe {
        assert_eq!(
            cavc_pline_boolean(
                pline1,
                pline2,
                operation,
                ptr::null(),
                &mut pos_plines,
                &mut neg_plines
            ),
            0
        );

        let pos = plinelist_props(pos_plines);
        let neg = plinelist_props(neg_plines);
        cavc_plinelist_f(pos_plines as *mut _);
        cavc_plinelist_f(neg_plines as *mut _);
        (pos, neg)
    }
}

fn run_boolean_props_with_options(
    pline1: *const cavc_pline,
    pline2: *const cavc_pline,
    operation: u32,
    options: *const cavc_pline_boolean_o,
) -> (Vec<PlineProps>, Vec<PlineProps>) {
    let mut pos_plines = ptr::null();
    let mut neg_plines = ptr::null();

    unsafe {
        assert_eq!(
            cavc_pline_boolean(
                pline1,
                pline2,
                operation,
                options,
                &mut pos_plines,
                &mut neg_plines
            ),
            0
        );

        let pos = plinelist_props(pos_plines);
        let neg = plinelist_props(neg_plines);
        cavc_plinelist_f(pos_plines as *mut _);
        cavc_plinelist_f(neg_plines as *mut _);
        (pos, neg)
    }
}

struct BooleanCase {
    name: &'static str,
    operation: u32,
    expected_remaining: Vec<PlineProps>,
    expected_subtracted: Vec<PlineProps>,
}

struct BooleanCaseWithInputs {
    name: &'static str,
    subject: *const cavc_pline,
    clip: *const cavc_pline,
    operation: u32,
    expected_remaining: Vec<PlineProps>,
    expected_subtracted: Vec<PlineProps>,
}

struct OffsetCase {
    name: &'static str,
    delta: f64,
    is_closed: bool,
    input: Vec<(f64, f64, f64)>,
    expected: Vec<PlineProps>,
}

type PlineInput = Vec<(f64, f64, f64)>;

fn run_parallel_offset_props(pline: *const cavc_pline, delta: f64) -> Vec<PlineProps> {
    let mut results = ptr::null();
    unsafe {
        assert_eq!(
            cavc_pline_parallel_offset(pline, delta, ptr::null(), &mut results),
            0
        );
        let props = plinelist_props(results);
        cavc_plinelist_f(results as *mut _);
        props
    }
}

fn run_parallel_offset_props_with_options(
    pline: *const cavc_pline,
    delta: f64,
    options: *const cavc_pline_parallel_offset_o,
) -> Vec<PlineProps> {
    let mut results = ptr::null();
    unsafe {
        assert_eq!(
            cavc_pline_parallel_offset(pline, delta, options, &mut results),
            0
        );
        let props = plinelist_props(results);
        cavc_plinelist_f(results as *mut _);
        props
    }
}

fn read_vertices(pline: *const cavc_pline) -> Vec<cavc_vertex> {
    let mut count = u32::MAX;
    unsafe {
        assert_eq!(cavc_pline_get_vertex_count(pline, &mut count), 0);
    }
    let mut result = Vec::with_capacity(count as usize);
    for i in 0..count {
        let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
        unsafe {
            assert_eq!(cavc_pline_get_vertex(pline, i, &mut v), 0);
        }
        result.push(v);
    }
    result
}

fn cpp_offset_simple_cases() -> Vec<OffsetCase> {
    vec![
        OffsetCase {
            name: "closed_rectangle_inward",
            delta: 2.0,
            is_closed: true,
            input: vec![
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0),
            ],
            expected: vec![PlineProps::new(4, 96.0, 44.0, 2.0, 2.0, 18.0, 8.0)],
        },
        OffsetCase {
            name: "open_rectangle_inward",
            delta: 2.0,
            is_closed: false,
            input: vec![
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0),
                (0.0, 0.0, 0.0),
            ],
            expected: vec![PlineProps::new(5, 0.0, 44.0, 2.0, 2.0, 18.0, 8.0)],
        },
        OffsetCase {
            name: "closed_rectangle_outward",
            delta: -2.0,
            is_closed: true,
            input: vec![
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0),
            ],
            expected: vec![PlineProps::new(
                8,
                332.56637061436,
                72.566370614359,
                -2.0,
                -2.0,
                22.0,
                12.0,
            )],
        },
        OffsetCase {
            name: "open_rectangle_outward",
            delta: -2.0,
            is_closed: false,
            input: vec![
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0),
                (0.0, 0.0, 0.0),
            ],
            expected: vec![PlineProps::new(
                8,
                0.0,
                69.424777960769,
                -2.0,
                -2.0,
                22.0,
                12.0,
            )],
        },
        OffsetCase {
            name: "closed_rectangle_coincident",
            delta: 5.0,
            is_closed: true,
            input: vec![
                (0.0, 0.0, 0.0),
                (20.0, 0.0, 0.0),
                (20.0, 10.0, 0.0),
                (0.0, 10.0, 0.0),
            ],
            expected: vec![PlineProps::new(2, 0.0, 20.0, 5.0, 5.0, 15.0, 5.0)],
        },
        OffsetCase {
            name: "closed_diamond_inward",
            delta: -5.0,
            is_closed: true,
            input: vec![
                (-10.0, 0.0, 0.0),
                (0.0, 10.0, 0.0),
                (10.0, 0.0, 0.0),
                (0.0, -10.0, 0.0),
            ],
            expected: vec![PlineProps::new(
                4,
                -17.157287525381,
                16.568542494924,
                -2.9289321881345,
                -2.9289321881345,
                2.9289321881345,
                2.9289321881345,
            )],
        },
        OffsetCase {
            name: "open_diamond_inward",
            delta: -5.0,
            is_closed: false,
            input: vec![
                (-10.0, 0.0, 0.0),
                (0.0, 10.0, 0.0),
                (10.0, 0.0, 0.0),
                (0.0, -10.0, 0.0),
                (-10.0, 0.0, 0.0),
            ],
            expected: vec![PlineProps::new(
                5,
                0.0,
                16.568542494924,
                -2.9289321881345,
                -2.9289321881345,
                2.9289321881345,
                2.9289321881345,
            )],
        },
        OffsetCase {
            name: "closed_diamond_outward",
            delta: 5.0,
            is_closed: true,
            input: vec![
                (-10.0, 0.0, 0.0),
                (0.0, 10.0, 0.0),
                (10.0, 0.0, 0.0),
                (0.0, -10.0, 0.0),
            ],
            expected: vec![PlineProps::new(
                8,
                -561.38252881436,
                87.984469030822,
                -15.0,
                -15.0,
                15.0,
                15.0,
            )],
        },
        OffsetCase {
            name: "open_diamond_outward",
            delta: 5.0,
            is_closed: false,
            input: vec![
                (-10.0, 0.0, 0.0),
                (0.0, 10.0, 0.0),
                (10.0, 0.0, 0.0),
                (0.0, -10.0, 0.0),
                (-10.0, 0.0, 0.0),
            ],
            expected: vec![PlineProps::new(
                8,
                0.0,
                80.130487396847,
                -13.535533905933,
                -15.0,
                15.0,
                15.0,
            )],
        },
    ]
}

fn cpp_offset_specific_cases() -> Vec<OffsetCase> {
    vec![
        OffsetCase {
            name: "offset_arc_just_past_line1",
            delta: 0.1,
            is_closed: true,
            input: vec![
                (27.804688, 1.0, 0.0),
                (28.46842055794889, 0.3429054695163245, 0.0),
                (32.34577133994935, 0.9269762697003898, 0.0),
                (32.38116957207762, 1.451312562563487, 0.0),
                (31.5, 1.0, -0.31783751349740424),
                (30.79289310940682, 1.5, 0.0),
                (29.20710689059337, 1.5, -0.31783754777018053),
                (28.49999981323106, 1.00000000000007, 0.0),
            ],
            expected: vec![
                PlineProps::new(
                    4,
                    0.094833810726263,
                    1.8213211761499,
                    31.533345690439,
                    0.90572346564886,
                    32.26949555256,
                    1.2817628453883,
                ),
                PlineProps::new(
                    6,
                    1.7197931450343,
                    7.5140262005179,
                    28.047835685678,
                    0.44926177903859,
                    31.495431966272,
                    1.4,
                ),
            ],
        },
        OffsetCase {
            name: "intersect_ontop_first_vertex",
            delta: 0.25,
            is_closed: true,
            input: vec![
                (27.804688, 1.0, 0.0),
                (27.804688, 0.75, 0.0),
                (32.195313, 0.75, 0.0),
                (32.195313, 1.0, 0.0),
                (31.5, 1.0, -0.3178375134974),
                (30.792893109407, 1.5, 0.0),
                (29.207106890593, 1.5, -0.31783754777018),
                (28.499999813231, 1.0000000000001, 0.0),
            ],
            expected: vec![PlineProps::new(
                4,
                0.36247092523069,
                3.593999211522,
                29.16143806012,
                1.0,
                30.838561906052,
                1.25,
            )],
        },
        OffsetCase {
            name: "collapsed_rectangle",
            delta: 30.0,
            is_closed: true,
            input: vec![
                (0.0, 0.0, 0.0),
                (120.0, 0.0, 0.0),
                (120.0, 40.0, 0.0),
                (0.0, 40.0, 0.0),
            ],
            expected: vec![],
        },
    ]
}

fn cpp_coincident_case1_inputs() -> (PlineInput, PlineInput) {
    (
        vec![
            (-0.105, 0.235, 0.0),
            (-0.095, 0.235, 0.0),
            (-0.095, 0.0, -1.0),
            (-0.105, 0.0, 0.0),
        ],
        vec![
            (-0.25, 0.235, -0.414214),
            (-0.255, 0.24, 0.0),
            (-0.255, 0.29, -0.414214),
            (-0.25, 0.295, 0.0),
            (0.25, 0.295, -0.414214),
            (0.255, 0.29, 0.0),
            (0.255, 0.24, -0.414214),
            (0.25, 0.235, 0.0),
        ],
    )
}

fn cpp_coincident_case2_inputs() -> (PlineInput, PlineInput) {
    (
        vec![
            (0.0, 0.0, 0.0),
            (0.0, 20.0, 0.0),
            (20.0, 20.0, 0.0),
            (20.0, 0.0, 0.0),
        ],
        vec![
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
            (30.0, 20.0, 0.0),
        ],
    )
}

const CPP_MATRIX_EPS: f64 = 1e-4;
const CPP_PROBE_DELTA: f64 = 0.01;
const CPP_CIRCLE_RADIUS: f64 = 5.0;
const CPP_CIRCLE_INSIDE_DIST_FACTOR: f64 = 0.33;
const CPP_CIRCLE_OUTSIDE_DIST_FACTOR: f64 = 1.5;

#[derive(Copy, Clone)]
enum CircleAlignment {
    XAxis,
    YAxis,
    Diagonal,
}

#[derive(Copy, Clone)]
struct CircleCaseKey {
    center_x: f64,
    center_y: f64,
    direction: i32,
    alignment: CircleAlignment,
    reverse: bool,
}

#[derive(Copy, Clone)]
struct HalfCircleCaseKey {
    center_x: f64,
    center_y: f64,
    direction: i32,
    is_x_aligned: bool,
    is_closed: bool,
}

fn assert_near_ctx(actual: f64, expected: f64, context: &str) {
    assert!(
        (actual - expected).abs() <= CPP_MATRIX_EPS,
        "{context}: expected {expected}, got {actual}"
    );
}

fn eval_wn(pline: *const cavc_pline, x: f64, y: f64) -> i32 {
    let mut wn = i32::MIN;
    unsafe {
        assert_eq!(cavc_pline_eval_wn(pline, x, y, &mut wn), 0);
    }
    wn
}

fn eval_closest_point_result(
    pline: *const cavc_pline,
    x: f64,
    y: f64,
    pos_equal_eps: f64,
) -> (u32, cavc_point, f64) {
    let mut seg_index = u32::MAX;
    let mut point = cavc_point::new(f64::NAN, f64::NAN);
    let mut distance = f64::NAN;
    unsafe {
        assert_eq!(
            cavc_pline_eval_closest_point(
                pline,
                x,
                y,
                pos_equal_eps,
                &mut seg_index,
                &mut point,
                &mut distance
            ),
            0
        );
    }
    (seg_index, point, distance)
}

fn cpp_circle_matrix_cases() -> Vec<CircleCaseKey> {
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

fn cpp_circle_case_vertices(case: CircleCaseKey) -> Vec<(f64, f64, f64)> {
    let mut p0 = match case.alignment {
        CircleAlignment::XAxis => (case.center_x - CPP_CIRCLE_RADIUS, case.center_y),
        CircleAlignment::YAxis => (case.center_x, case.center_y - CPP_CIRCLE_RADIUS),
        CircleAlignment::Diagonal => (
            case.center_x + CPP_CIRCLE_RADIUS * (std::f64::consts::PI / 4.0).cos(),
            case.center_y + CPP_CIRCLE_RADIUS * (std::f64::consts::PI / 4.0).sin(),
        ),
    };
    let mut p1 = match case.alignment {
        CircleAlignment::XAxis => (case.center_x + CPP_CIRCLE_RADIUS, case.center_y),
        CircleAlignment::YAxis => (case.center_x, case.center_y + CPP_CIRCLE_RADIUS),
        CircleAlignment::Diagonal => (
            case.center_x + CPP_CIRCLE_RADIUS * (5.0 * std::f64::consts::PI / 4.0).cos(),
            case.center_y + CPP_CIRCLE_RADIUS * (5.0 * std::f64::consts::PI / 4.0).sin(),
        ),
    };

    if case.reverse {
        std::mem::swap(&mut p0, &mut p1);
    }

    let bulge = if case.direction > 0 { 1.0 } else { -1.0 };
    vec![(p0.0, p0.1, bulge), (p1.0, p1.1, bulge)]
}

fn cpp_half_circle_matrix_cases() -> Vec<HalfCircleCaseKey> {
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

fn cpp_half_circle_case_vertices(case: HalfCircleCaseKey) -> Vec<(f64, f64, f64)> {
    let bulge = if case.direction > 0 { 1.0 } else { -1.0 };
    if case.is_x_aligned {
        vec![
            (case.center_x - CPP_CIRCLE_RADIUS, case.center_y, bulge),
            (case.center_x + CPP_CIRCLE_RADIUS, case.center_y, 0.0),
        ]
    } else {
        vec![
            (case.center_x, case.center_y - CPP_CIRCLE_RADIUS, bulge),
            (case.center_x, case.center_y + CPP_CIRCLE_RADIUS, 0.0),
        ]
    }
}

fn cpp_expected_half_circle_extents(case: HalfCircleCaseKey) -> (f64, f64, f64, f64) {
    let mut min_x = case.center_x - CPP_CIRCLE_RADIUS;
    let mut min_y = case.center_y - CPP_CIRCLE_RADIUS;
    let mut max_x = case.center_x + CPP_CIRCLE_RADIUS;
    let mut max_y = case.center_y + CPP_CIRCLE_RADIUS;

    if case.direction > 0 {
        if case.is_x_aligned {
            max_y -= CPP_CIRCLE_RADIUS;
        } else {
            min_x += CPP_CIRCLE_RADIUS;
        }
    } else if case.is_x_aligned {
        min_y += CPP_CIRCLE_RADIUS;
    } else {
        max_x -= CPP_CIRCLE_RADIUS;
    }

    (min_x, min_y, max_x, max_y)
}

fn circle_expected_closest_point(
    center_x: f64,
    center_y: f64,
    qx: f64,
    qy: f64,
) -> (f64, f64, f64) {
    let vx = qx - center_x;
    let vy = qy - center_y;
    let len = (vx * vx + vy * vy).sqrt();
    assert!(len > 0.0);
    let ux = vx / len;
    let uy = vy / len;
    let px = center_x + CPP_CIRCLE_RADIUS * ux;
    let py = center_y + CPP_CIRCLE_RADIUS * uy;
    let dist = (len - CPP_CIRCLE_RADIUS).abs();
    (px, py, dist)
}

#[test]
fn pline_function_surface_circle_metrics_winding_cpp_matrix_parity() {
    for case in cpp_circle_matrix_cases() {
        let pline = create_pline(&cpp_circle_case_vertices(case), true);
        let props = pline_props(pline);

        let expected_area =
            (case.direction as f64) * std::f64::consts::PI * CPP_CIRCLE_RADIUS * CPP_CIRCLE_RADIUS;
        let expected_path_length = 2.0 * std::f64::consts::PI * CPP_CIRCLE_RADIUS;

        assert_near_ctx(props.area, expected_area, "circle area");
        assert_near_ctx(
            props.path_length,
            expected_path_length,
            "circle path_length",
        );
        assert_near_ctx(
            props.min_x,
            case.center_x - CPP_CIRCLE_RADIUS,
            "circle extents min_x",
        );
        assert_near_ctx(
            props.min_y,
            case.center_y - CPP_CIRCLE_RADIUS,
            "circle extents min_y",
        );
        assert_near_ctx(
            props.max_x,
            case.center_x + CPP_CIRCLE_RADIUS,
            "circle extents max_x",
        );
        assert_near_ctx(
            props.max_y,
            case.center_y + CPP_CIRCLE_RADIUS,
            "circle extents max_y",
        );

        let outside = [
            (
                case.center_x - CPP_CIRCLE_RADIUS - CPP_PROBE_DELTA,
                case.center_y,
            ),
            (
                case.center_x + CPP_CIRCLE_RADIUS + CPP_PROBE_DELTA,
                case.center_y,
            ),
            (
                case.center_x,
                case.center_y - CPP_CIRCLE_RADIUS - CPP_PROBE_DELTA,
            ),
            (
                case.center_x,
                case.center_y + CPP_CIRCLE_RADIUS + CPP_PROBE_DELTA,
            ),
        ];
        for (x, y) in outside {
            assert_eq!(eval_wn(pline, x, y), 0);
        }

        let inside_axis = [
            (case.center_x, case.center_y),
            (
                case.center_x - CPP_CIRCLE_RADIUS + CPP_PROBE_DELTA,
                case.center_y,
            ),
            (
                case.center_x + CPP_CIRCLE_RADIUS - CPP_PROBE_DELTA,
                case.center_y,
            ),
            (
                case.center_x,
                case.center_y - CPP_CIRCLE_RADIUS + CPP_PROBE_DELTA,
            ),
            (
                case.center_x,
                case.center_y + CPP_CIRCLE_RADIUS - CPP_PROBE_DELTA,
            ),
        ];
        for (x, y) in inside_axis {
            assert_eq!(eval_wn(pline, x, y), case.direction);
        }

        let inside_dist = CPP_CIRCLE_INSIDE_DIST_FACTOR * CPP_CIRCLE_RADIUS;
        let outside_dist = CPP_CIRCLE_OUTSIDE_DIST_FACTOR * CPP_CIRCLE_RADIUS;
        for i in 0..4 {
            let theta = std::f64::consts::PI / 4.0 + (i as f64) * std::f64::consts::PI / 2.0;
            let cos_t = theta.cos();
            let sin_t = theta.sin();
            let inside = (
                case.center_x + inside_dist * cos_t,
                case.center_y + inside_dist * sin_t,
            );
            let outside = (
                case.center_x + outside_dist * cos_t,
                case.center_y + outside_dist * sin_t,
            );
            assert_eq!(eval_wn(pline, inside.0, inside.1), case.direction);
            assert_eq!(eval_wn(pline, outside.0, outside.1), 0);
        }

        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_function_surface_circle_closest_point_cpp_matrix_parity() {
    for case in cpp_circle_matrix_cases() {
        let pline = create_pline(&cpp_circle_case_vertices(case), true);
        let vertices = read_vertices(pline);

        // addClosestPointOnVertexes parity anchors from old C++ source.
        let (i0, p0, d0) = eval_closest_point_result(pline, vertices[0].x, vertices[0].y, 1e-5);
        assert_eq!(i0, 0);
        assert_near_ctx(p0.x, vertices[0].x, "circle closest vertex0 x");
        assert_near_ctx(p0.y, vertices[0].y, "circle closest vertex0 y");
        assert_near_ctx(d0, 0.0, "circle closest vertex0 distance");

        let (i1, p1, d1) = eval_closest_point_result(pline, vertices[1].x, vertices[1].y, 1e-5);
        assert_eq!(i1, 1);
        assert_near_ctx(p1.x, vertices[1].x, "circle closest vertex1 x");
        assert_near_ctx(p1.y, vertices[1].y, "circle closest vertex1 y");
        assert_near_ctx(d1, 0.0, "circle closest vertex1 distance");

        // axis probes from old C++ source.
        let axis_queries = [
            (case.center_x - 0.1, case.center_y),
            (case.center_x + 0.1, case.center_y),
            (case.center_x, case.center_y - 0.1),
            (case.center_x, case.center_y + 0.1),
        ];
        for (qx, qy) in axis_queries {
            let (_idx, p, d) = eval_closest_point_result(pline, qx, qy, 1e-5);
            let (ex, ey, ed) = circle_expected_closest_point(case.center_x, case.center_y, qx, qy);
            assert_near_ctx(p.x, ex, "circle closest axis x");
            assert_near_ctx(p.y, ey, "circle closest axis y");
            assert_near_ctx(d, ed, "circle closest axis distance");
        }

        // 45-degree inside/outside probes from old C++ source.
        let inside_dist = CPP_CIRCLE_INSIDE_DIST_FACTOR * CPP_CIRCLE_RADIUS;
        let outside_dist = CPP_CIRCLE_OUTSIDE_DIST_FACTOR * CPP_CIRCLE_RADIUS;
        for i in 0..4 {
            let theta = std::f64::consts::PI / 4.0 + (i as f64) * std::f64::consts::PI / 2.0;
            let cos_t = theta.cos();
            let sin_t = theta.sin();
            let inside_q = (
                case.center_x + inside_dist * cos_t,
                case.center_y + inside_dist * sin_t,
            );
            let outside_q = (
                case.center_x + outside_dist * cos_t,
                case.center_y + outside_dist * sin_t,
            );

            let (_idx_i, p_i, d_i) = eval_closest_point_result(pline, inside_q.0, inside_q.1, 1e-5);
            let (ex_i, ey_i, ed_i) =
                circle_expected_closest_point(case.center_x, case.center_y, inside_q.0, inside_q.1);
            assert_near_ctx(p_i.x, ex_i, "circle closest inside45 x");
            assert_near_ctx(p_i.y, ey_i, "circle closest inside45 y");
            assert_near_ctx(d_i, ed_i, "circle closest inside45 distance");

            let (_idx_o, p_o, d_o) =
                eval_closest_point_result(pline, outside_q.0, outside_q.1, 1e-5);
            let (ex_o, ey_o, ed_o) = circle_expected_closest_point(
                case.center_x,
                case.center_y,
                outside_q.0,
                outside_q.1,
            );
            assert_near_ctx(p_o.x, ex_o, "circle closest outside45 x");
            assert_near_ctx(p_o.y, ey_o, "circle closest outside45 y");
            assert_near_ctx(d_o, ed_o, "circle closest outside45 distance");
        }

        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_function_surface_half_circle_metrics_winding_cpp_matrix_parity() {
    for case in cpp_half_circle_matrix_cases() {
        let pline = create_pline(&cpp_half_circle_case_vertices(case), case.is_closed);
        let props = pline_props(pline);
        let (min_x, min_y, max_x, max_y) = cpp_expected_half_circle_extents(case);

        let expected_area = if case.is_closed {
            (case.direction as f64) * std::f64::consts::PI * CPP_CIRCLE_RADIUS * CPP_CIRCLE_RADIUS
                / 2.0
        } else {
            0.0
        };
        let expected_path_length = std::f64::consts::PI * CPP_CIRCLE_RADIUS
            + if case.is_closed {
                2.0 * CPP_CIRCLE_RADIUS
            } else {
                0.0
            };

        assert_near_ctx(props.area, expected_area, "half-circle area");
        assert_near_ctx(
            props.path_length,
            expected_path_length,
            "half-circle path_length",
        );
        assert_near_ctx(props.min_x, min_x, "half-circle extents min_x");
        assert_near_ctx(props.min_y, min_y, "half-circle extents min_y");
        assert_near_ctx(props.max_x, max_x, "half-circle extents max_x");
        assert_near_ctx(props.max_y, max_y, "half-circle extents max_y");

        let outside = [
            (min_x - CPP_PROBE_DELTA, case.center_y),
            (max_x + CPP_PROBE_DELTA, case.center_y),
            (case.center_x, min_y - CPP_PROBE_DELTA),
            (case.center_x, max_y + CPP_PROBE_DELTA),
        ];
        for (x, y) in outside {
            assert_eq!(eval_wn(pline, x, y), 0);
        }

        let expected_inside_winding = if case.is_closed { case.direction } else { 0 };
        let inside = if case.is_x_aligned {
            [
                (case.center_x, min_y + CPP_PROBE_DELTA),
                (case.center_x, max_y - CPP_PROBE_DELTA),
            ]
        } else {
            [
                (min_x + CPP_PROBE_DELTA, case.center_y),
                (max_x - CPP_PROBE_DELTA, case.center_y),
            ]
        };
        for (x, y) in inside {
            assert_eq!(eval_wn(pline, x, y), expected_inside_winding);
        }

        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_data_manipulation() {
    let pline = create_pline(&[], true);
    let null_ptr = ptr::null_mut();
    unsafe {
        // test reserve
        assert_eq!(cavc_pline_reserve(pline, 5), 0);
        assert_eq!(cavc_pline_reserve(null_ptr, 5), 1);

        // test pline is closed
        let mut is_closed: u8 = 0;
        assert_eq!(cavc_pline_get_is_closed(pline, &mut is_closed), 0);
        assert_ne!(is_closed, 0);

        // set pline to be open
        assert_eq!(cavc_pline_set_is_closed(pline, 0), 0);
        assert_eq!(cavc_pline_get_is_closed(pline, &mut is_closed), 0);
        assert_eq!(is_closed, 0);

        // set vertex data
        let vertex_data = [
            cavc_vertex::new(-1.0, -2.0, 0.0),
            cavc_vertex::new(-3.0, -4.0, -1.0),
        ];

        assert_eq!(
            cavc_pline_set_vertex_data(pline, vertex_data.as_ptr(), vertex_data.len() as u32),
            0
        );

        // read all vertex data
        let mut data_out = [cavc_vertex::new(0.0, 0.0, 0.0); 2];
        assert_eq!(cavc_pline_get_vertex_data(pline, data_out.as_mut_ptr()), 0);
        assert_eq!(data_out[0].x, -1.0);
        assert_eq!(data_out[0].y, -2.0);
        assert_eq!(data_out[0].bulge, 0.0);
        assert_eq!(data_out[1].x, -3.0);
        assert_eq!(data_out[1].y, -4.0);
        assert_eq!(data_out[1].bulge, -1.0);

        // clone
        let mut cloned = ptr::null();
        assert_eq!(cavc_pline_clone(pline, &mut cloned), 0);
        let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
        assert_eq!(cavc_pline_get_vertex(cloned, 1, &mut v), 0);
        assert_eq!(v.x, -3.0);
        assert_eq!(v.y, -4.0);
        assert_eq!(v.bulge, -1.0);
        assert_eq!(cavc_pline_clone(null_ptr, &mut cloned), 1);
        cavc_pline_f(cloned as *mut _);

        // clear vertexes
        assert_eq!(cavc_pline_clear(pline), 0);
        let mut count: u32 = 0;
        assert_eq!(cavc_pline_get_vertex_count(pline, &mut count), 0);
        assert_eq!(count, 0);

        // add vertexes
        assert_eq!(cavc_pline_add(null_ptr, 0.0, 0.0, 0.0), 1);
        assert_eq!(cavc_pline_add(pline, 1.0, 2.0, 0.0), 0);
        assert_eq!(cavc_pline_add(pline, 3.0, 4.0, 1.0), 0);

        // get vertex count
        let mut count: u32 = 0;
        assert_eq!(cavc_pline_get_vertex_count(null_ptr, &mut count), 1);
        assert_eq!(cavc_pline_get_vertex_count(pline, &mut count), 0);
        assert_eq!(count, 2);

        // read vertex at positions
        let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
        assert_eq!(cavc_pline_get_vertex(null_ptr, 0, &mut v), 1);
        assert_eq!(cavc_pline_get_vertex(pline, 0, &mut v), 0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.bulge, 0.0);

        assert_eq!(cavc_pline_get_vertex(pline, 1, &mut v), 0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
        assert_eq!(v.bulge, 1.0);

        // get index position out of bounds
        assert_eq!(cavc_pline_get_vertex(pline, 3, &mut v), 2);

        // set vertex at position
        assert_eq!(
            cavc_pline_set_vertex(pline, 0, cavc_vertex::new(8.0, 8.0, 0.55)),
            0
        );

        assert_eq!(cavc_pline_get_vertex(pline, 0, &mut v), 0);
        assert_eq!(v.x, 8.0);
        assert_eq!(v.y, 8.0);
        assert_eq!(v.bulge, 0.55);

        // set index position out of bounds
        assert_eq!(
            cavc_pline_set_vertex(pline, 3, cavc_vertex::new(0.0, 0.0, 0.0)),
            2
        );

        // remove vertex at position
        assert_eq!(cavc_pline_remove(pline, 0), 0);
        let mut count: u32 = 0;
        assert_eq!(cavc_pline_get_vertex_count(pline, &mut count), 0);
        assert_eq!(count, 1);
        let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
        assert_eq!(cavc_pline_get_vertex(pline, 0, &mut v), 0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 4.0);
        assert_eq!(v.bulge, 1.0);

        cavc_pline_f(pline);
    }
}

#[test]
fn pline_eval_path_length() {
    let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
    let mut l = f64::NAN;
    unsafe {
        assert_eq!(cavc_pline_eval_path_length(pline, &mut l), 0);
        assert_eq!(cavc_pline_eval_path_length(ptr::null_mut(), &mut l), 1);
    }
    assert_fuzzy_eq!(l, std::f64::consts::TAU);
    unsafe { cavc_pline_f(pline) }
}

#[test]
fn pline_eval_area() {
    let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
    let mut a = f64::NAN;
    unsafe {
        assert_eq!(cavc_pline_eval_area(pline, &mut a), 0);
        assert_eq!(cavc_pline_eval_area(ptr::null_mut(), &mut a), 1);
    }
    assert_fuzzy_eq!(a, std::f64::consts::PI);
    unsafe { cavc_pline_f(pline) }
}

#[test]
fn pline_eval_wn() {
    let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
    let (x, y) = (1.0, 0.0);
    let mut wn = i32::MAX;
    unsafe {
        assert_eq!(cavc_pline_eval_wn(pline, x, y, &mut wn), 0);
        assert_eq!(cavc_pline_eval_wn(ptr::null_mut(), x, y, &mut wn), 1);
    }
    assert_eq!(wn, 1);
    unsafe { cavc_pline_f(pline) }
}

#[test]
fn pline_eval_closest_point() {
    // empty pline -> undefined closest point (error code 2)
    let empty_pline = create_pline(&[], true);
    let mut seg_start_index = u32::MAX;
    let mut closest_point = cavc_point::new(f64::NAN, f64::NAN);
    let mut distance = f64::NAN;
    unsafe {
        assert_eq!(
            cavc_pline_eval_closest_point(
                empty_pline,
                0.0,
                0.0,
                1e-5,
                &mut seg_start_index,
                &mut closest_point,
                &mut distance
            ),
            2
        );
        assert_eq!(
            cavc_pline_eval_closest_point(
                ptr::null_mut(),
                0.0,
                0.0,
                1e-5,
                &mut seg_start_index,
                &mut closest_point,
                &mut distance
            ),
            1
        );
        cavc_pline_f(empty_pline);
    }

    // non-empty sanity check
    let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
    let (seg_index, p, d) = eval_closest_point_result(pline, 0.0, 0.0, 1e-5);
    assert_eq!(seg_index, 0);
    assert_fuzzy_eq!(p.x, 0.0);
    assert_fuzzy_eq!(p.y, 0.0);
    assert_fuzzy_eq!(d, 0.0);
    unsafe {
        cavc_pline_f(pline);
    }
}

#[test]
fn pline_invert_direction() {
    let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
    unsafe {
        assert_eq!(cavc_pline_invert_direction(pline), 0);
        assert_eq!(cavc_pline_invert_direction(ptr::null_mut()), 1);
        let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
        assert_eq!(cavc_pline_get_vertex(pline, 0, &mut v), 0);
        assert_eq!(v.x, 2.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.bulge, -1.0);
        assert_eq!(cavc_pline_get_vertex(pline, 1, &mut v), 0);
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.bulge, -1.0);

        cavc_pline_f(pline)
    }
}

#[test]
fn pline_scale() {
    let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
    let scale_factor = 2.0;
    unsafe {
        assert_eq!(cavc_pline_scale(pline, scale_factor), 0);
        assert_eq!(cavc_pline_scale(ptr::null_mut(), scale_factor), 1);
        let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
        assert_eq!(cavc_pline_get_vertex(pline, 0, &mut v), 0);
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.bulge, 1.0);
        assert_eq!(cavc_pline_get_vertex(pline, 1, &mut v), 0);
        assert_eq!(v.x, 4.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.bulge, 1.0);

        cavc_pline_f(pline)
    }
}

#[test]
fn pline_translate() {
    let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
    let (x_offset, y_offset) = (1.0, 1.0);
    unsafe {
        assert_eq!(cavc_pline_translate(pline, x_offset, y_offset), 0);
        assert_eq!(cavc_pline_translate(ptr::null_mut(), x_offset, y_offset), 1);
        let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
        assert_eq!(cavc_pline_get_vertex(pline, 0, &mut v), 0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.bulge, 1.0);
        assert_eq!(cavc_pline_get_vertex(pline, 1, &mut v), 0);
        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 1.0);
        assert_eq!(v.bulge, 1.0);

        cavc_pline_f(pline)
    }
}

#[test]
fn pline_remove_repeat_pos() {
    // no vertexes removed
    {
        let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
        let pos_equal_eps = 1e-5;
        unsafe {
            assert_eq!(cavc_pline_remove_repeat_pos(pline, pos_equal_eps), 0);
            assert_eq!(
                cavc_pline_remove_repeat_pos(ptr::null_mut(), pos_equal_eps),
                1
            );
            let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
            assert_eq!(cavc_pline_get_vertex(pline, 0, &mut v), 0);
            assert_eq!(v.x, 0.0);
            assert_eq!(v.y, 0.0);
            assert_eq!(v.bulge, 1.0);
            assert_eq!(cavc_pline_get_vertex(pline, 1, &mut v), 0);
            assert_eq!(v.x, 2.0);
            assert_eq!(v.y, 0.0);
            assert_eq!(v.bulge, 1.0);

            cavc_pline_f(pline)
        }
    }

    // vertex removed
    {
        let pline = create_pline(&[(0.0, 0.0, 1.0), (0.0, 0.0, 0.5), (2.0, 0.0, 1.0)], true);
        let pos_equal_eps = 1e-5;
        unsafe {
            assert_eq!(cavc_pline_remove_repeat_pos(pline, pos_equal_eps), 0);
            let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
            assert_eq!(cavc_pline_get_vertex(pline, 0, &mut v), 0);
            assert_eq!(v.x, 0.0);
            assert_eq!(v.y, 0.0);
            assert_eq!(v.bulge, 0.5);
            assert_eq!(cavc_pline_get_vertex(pline, 1, &mut v), 0);
            assert_eq!(v.x, 2.0);
            assert_eq!(v.y, 0.0);
            assert_eq!(v.bulge, 1.0);

            cavc_pline_f(pline)
        }
    }
}

#[test]
fn pline_remove_redundant() {
    // no vertexes removed
    {
        let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
        let pos_equal_eps = 1e-5;
        unsafe {
            assert_eq!(cavc_pline_remove_redundant(pline, pos_equal_eps), 0);
            assert_eq!(
                cavc_pline_remove_redundant(ptr::null_mut(), pos_equal_eps),
                1
            );
            let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
            assert_eq!(cavc_pline_get_vertex(pline, 0, &mut v), 0);
            assert_eq!(v.x, 0.0);
            assert_eq!(v.y, 0.0);
            assert_eq!(v.bulge, 1.0);
            assert_eq!(cavc_pline_get_vertex(pline, 1, &mut v), 0);
            assert_eq!(v.x, 2.0);
            assert_eq!(v.y, 0.0);
            assert_eq!(v.bulge, 1.0);

            cavc_pline_f(pline)
        }
    }

    // vertex removed
    {
        let bulge = (std::f64::consts::FRAC_PI_2 / 4.0).tan();
        let pline = create_pline(
            &[(0.0, 0.0, bulge), (1.0, -1.0, bulge), (2.0, 0.0, 1.0)],
            true,
        );
        let pos_equal_eps = 1e-5;
        unsafe {
            assert_eq!(cavc_pline_remove_redundant(pline, pos_equal_eps), 0);
            let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
            assert_eq!(cavc_pline_get_vertex(pline, 0, &mut v), 0);
            assert_fuzzy_eq!(v.x, 0.0);
            assert_fuzzy_eq!(v.y, 0.0);
            assert_fuzzy_eq!(v.bulge, 1.0);
            assert_eq!(cavc_pline_get_vertex(pline, 1, &mut v), 0);
            assert_fuzzy_eq!(v.x, 2.0);
            assert_fuzzy_eq!(v.y, 0.0);
            assert_fuzzy_eq!(v.bulge, 1.0);

            cavc_pline_f(pline)
        }
    }
}

#[test]
fn pline_eval_extents() {
    let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (f64::NAN, f64::NAN, f64::NAN, f64::NAN);
    unsafe {
        assert_eq!(
            cavc_pline_eval_extents(pline, &mut min_x, &mut min_y, &mut max_x, &mut max_y),
            0
        );
        assert_eq!(
            cavc_pline_eval_extents(
                ptr::null_mut(),
                &mut min_x,
                &mut min_y,
                &mut max_x,
                &mut max_y
            ),
            1
        );
    }
    assert_fuzzy_eq!(min_x, 0.0);
    assert_fuzzy_eq!(min_y, -1.0);
    assert_fuzzy_eq!(max_x, 2.0);
    assert_fuzzy_eq!(max_y, 1.0);

    unsafe { cavc_pline_f(pline) }
}

#[test]
fn pline_eval_parallel_offset() {
    // null options
    {
        let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
        let offset = -1.0;
        let mut results = ptr::null();
        unsafe {
            assert_eq!(
                cavc_pline_parallel_offset(pline, offset, ptr::null_mut(), &mut results),
                0
            );

            assert_eq!(
                cavc_pline_parallel_offset(ptr::null_mut(), offset, ptr::null_mut(), &mut results),
                1
            );

            let mut results_count = u32::MAX;
            assert_eq!(cavc_plinelist_get_count(results, &mut results_count), 0);
            assert_eq!(results_count, 1);

            let mut result_pline = ptr::null();
            assert_eq!(cavc_plinelist_get_pline(results, 0, &mut result_pline), 0);
            let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
            assert_eq!(cavc_pline_get_vertex(result_pline, 0, &mut v), 0);
            assert_fuzzy_eq!(v.x, -1.0);
            assert_fuzzy_eq!(v.y, 0.0);
            assert_fuzzy_eq!(v.bulge, 1.0);
            assert_eq!(cavc_pline_get_vertex(result_pline, 1, &mut v), 0);
            assert_fuzzy_eq!(v.x, 3.0);
            assert_fuzzy_eq!(v.y, 0.0);
            assert_fuzzy_eq!(v.bulge, 1.0);

            cavc_plinelist_f(results as *mut _);

            cavc_pline_f(pline);
        }
    }

    // with options
    {
        let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
        let offset = -1.0;
        let mut options = cavc_pline_parallel_offset_o {
            aabb_index: std::ptr::null(),
            pos_equal_eps: f64::NAN,
            slice_join_eps: f64::NAN,
            offset_dist_eps: f64::NAN,
            handle_self_intersects: 0,
        };

        let mut results = ptr::null();
        unsafe {
            assert_eq!(cavc_pline_parallel_offset_o_init(&mut options), 0);
            assert!(!options.pos_equal_eps.is_nan());
            assert!(!options.slice_join_eps.is_nan());
            assert!(!options.offset_dist_eps.is_nan());

            let mut aabb_index = ptr::null();

            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
                0
            );

            options.aabb_index = aabb_index;

            assert_eq!(
                cavc_pline_parallel_offset(pline, offset, &options, &mut results),
                0
            );

            assert_eq!(
                cavc_pline_parallel_offset(ptr::null_mut(), offset, &options, &mut results),
                1
            );

            let mut results_count = u32::MAX;
            assert_eq!(cavc_plinelist_get_count(results, &mut results_count), 0);
            assert_eq!(results_count, 1);

            let mut result_pline = ptr::null();
            assert_eq!(cavc_plinelist_get_pline(results, 0, &mut result_pline), 0);
            let mut v = cavc_vertex::new(0.0, 0.0, 0.0);
            assert_eq!(cavc_pline_get_vertex(result_pline, 0, &mut v), 0);
            assert_fuzzy_eq!(v.x, -1.0);
            assert_fuzzy_eq!(v.y, 0.0);
            assert_fuzzy_eq!(v.bulge, 1.0);
            assert_eq!(cavc_pline_get_vertex(result_pline, 1, &mut v), 0);
            assert_fuzzy_eq!(v.x, 3.0);
            assert_fuzzy_eq!(v.y, 0.0);
            assert_fuzzy_eq!(v.bulge, 1.0);

            cavc_plinelist_f(results as *mut _);

            cavc_aabbindex_f(aabb_index as *mut _);

            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_eval_boolean() {
    // null options
    {
        // 4x4 square
        let pline1 = create_pline(
            &[
                (-1.0, -2.0, 0.0),
                (3.0, -2.0, 0.0),
                (3.0, 2.0, 0.0),
                (-1.0, 2.0, 0.0),
            ],
            true,
        );
        // circle inside square with radius 1
        let pline2 = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);

        let mut pos_plines = ptr::null();
        let mut neg_plines = ptr::null();

        unsafe {
            assert_eq!(
                cavc_pline_boolean(
                    pline1,
                    pline2,
                    2,
                    ptr::null_mut(),
                    &mut pos_plines,
                    &mut neg_plines
                ),
                0
            );

            assert_eq!(
                cavc_pline_boolean(
                    ptr::null(),
                    ptr::null(),
                    2,
                    ptr::null_mut(),
                    &mut pos_plines,
                    &mut neg_plines
                ),
                1
            );
            assert_eq!(
                cavc_pline_boolean(
                    pline1,
                    ptr::null(),
                    2,
                    ptr::null_mut(),
                    &mut pos_plines,
                    &mut neg_plines
                ),
                1
            );
            assert_eq!(
                cavc_pline_boolean(
                    ptr::null(),
                    pline2,
                    2,
                    ptr::null_mut(),
                    &mut pos_plines,
                    &mut neg_plines
                ),
                1
            );

            let mut pos_plines_count = u32::MAX;
            assert_eq!(
                cavc_plinelist_get_count(pos_plines, &mut pos_plines_count),
                0
            );
            assert_eq!(pos_plines_count, 1);

            let mut neg_plines_count = u32::MAX;
            assert_eq!(
                cavc_plinelist_get_count(pos_plines, &mut neg_plines_count),
                0
            );
            assert_eq!(neg_plines_count, 1);

            let mut output_pline = ptr::null();
            assert_eq!(
                cavc_plinelist_get_pline(pos_plines, 0, &mut output_pline),
                0
            );

            let mut area = f64::NAN;
            assert_eq!(cavc_pline_eval_area(output_pline, &mut area), 0);
            assert_fuzzy_eq!(area, 16.0);

            assert_eq!(
                cavc_plinelist_get_pline(neg_plines, 0, &mut output_pline),
                0
            );
            assert_eq!(cavc_pline_eval_area(output_pline, &mut area), 0);
            assert_fuzzy_eq!(area, std::f64::consts::PI);

            // test take on the plinelist
            // null ptr
            assert_eq!(
                cavc_plinelist_take(ptr::null_mut(), 0, &mut output_pline),
                1
            );
            // index position out of range
            assert_eq!(
                cavc_plinelist_take(neg_plines as *mut _, 1, &mut output_pline),
                2
            );
            assert_eq!(
                cavc_plinelist_take(neg_plines as *mut _, 0, &mut output_pline),
                0
            );

            let mut area = 0.0;
            assert_eq!(cavc_pline_eval_area(output_pline, &mut area), 0);
            assert_fuzzy_eq!(area, std::f64::consts::PI);
            let mut count = u32::MAX;
            assert_eq!(cavc_plinelist_get_count(neg_plines, &mut count), 0);
            assert_eq!(count, 0);
            cavc_pline_f(output_pline as *mut _);

            // test pop on plinelist
            // null ptr
            assert_eq!(cavc_plinelist_pop(ptr::null_mut(), &mut output_pline), 1);

            assert_eq!(
                cavc_plinelist_pop(pos_plines as *mut _, &mut output_pline),
                0
            );
            assert_eq!(cavc_pline_eval_area(output_pline, &mut area), 0);
            assert_fuzzy_eq!(area, 16.0);
            let mut count = u32::MAX;
            assert_eq!(cavc_plinelist_get_count(pos_plines, &mut count), 0);
            assert_eq!(count, 0);
            cavc_pline_f(output_pline as *mut _);

            // empty plinelist
            assert_eq!(
                cavc_plinelist_pop(pos_plines as *mut _, &mut output_pline),
                2
            );

            cavc_plinelist_f(pos_plines as *mut _);
            cavc_plinelist_f(neg_plines as *mut _);

            cavc_pline_f(pline1);
        }
    }

    // with options
    {
        // 4x4 square
        let pline1 = create_pline(
            &[
                (-1.0, -2.0, 0.0),
                (3.0, -2.0, 0.0),
                (3.0, 2.0, 0.0),
                (-1.0, 2.0, 0.0),
            ],
            true,
        );
        // circle inside square with radius 1
        let pline2 = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);

        let mut pos_plines = ptr::null();
        let mut neg_plines = ptr::null();

        let mut options = cavc_pline_boolean_o {
            pline1_aabb_index: std::ptr::null(),
            pos_equal_eps: f64::NAN,
            collapsed_area_eps: f64::NAN,
        };

        unsafe {
            assert_eq!(cavc_pline_boolean_o_init(&mut options), 0);
            assert!(!options.pos_equal_eps.is_nan());

            let mut pline1_aabb_index = ptr::null();

            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline1, &mut pline1_aabb_index),
                0
            );

            options.pline1_aabb_index = pline1_aabb_index;

            assert_eq!(
                cavc_pline_boolean(
                    pline1,
                    pline2,
                    2,
                    &options,
                    &mut pos_plines,
                    &mut neg_plines
                ),
                0
            );

            let mut pos_plines_count = u32::MAX;
            assert_eq!(
                cavc_plinelist_get_count(pos_plines, &mut pos_plines_count),
                0
            );
            assert_eq!(pos_plines_count, 1);

            let mut neg_plines_count = u32::MAX;
            assert_eq!(
                cavc_plinelist_get_count(neg_plines, &mut neg_plines_count),
                0
            );
            assert_eq!(neg_plines_count, 1);

            let mut output_pline = ptr::null();
            assert_eq!(
                cavc_plinelist_get_pline(pos_plines, 0, &mut output_pline),
                0
            );

            let mut area = f64::NAN;
            assert_eq!(cavc_pline_eval_area(output_pline, &mut area), 0);
            assert_fuzzy_eq!(area, 16.0);

            assert_eq!(
                cavc_plinelist_get_pline(neg_plines, 0, &mut output_pline),
                0
            );
            assert_eq!(cavc_pline_eval_area(output_pline, &mut area), 0);
            assert_fuzzy_eq!(area, std::f64::consts::PI);

            cavc_plinelist_f(pos_plines as *mut _);
            cavc_plinelist_f(neg_plines as *mut _);

            cavc_pline_f(pline1);
        }
    }
}

#[test]
fn pline_boolean_coincident_case1_intersect_cpp_parity() {
    // old C++ source: TEST_cavc_combine_plines.cpp -> coincident_case1_intersect
    let pline_a = create_pline(
        &[
            (-0.105, 0.235, 0.0),
            (-0.095, 0.235, 0.0),
            (-0.095, 0.0, -1.0),
            (-0.105, 0.0, 0.0),
        ],
        true,
    );
    let pline_b = create_pline(
        &[
            (-0.25, 0.235, -0.414214),
            (-0.255, 0.24, 0.0),
            (-0.255, 0.29, -0.414214),
            (-0.25, 0.295, 0.0),
            (0.25, 0.295, -0.414214),
            (0.255, 0.29, 0.0),
            (0.255, 0.24, -0.414214),
            (0.25, 0.235, 0.0),
        ],
        true,
    );

    let mut pos_plines = ptr::null();
    let mut neg_plines = ptr::null();

    unsafe {
        // 1 = BooleanOp::And
        assert_eq!(
            cavc_pline_boolean(
                pline_a,
                pline_b,
                1,
                ptr::null(),
                &mut pos_plines,
                &mut neg_plines
            ),
            0
        );

        let mut pos_count = u32::MAX;
        let mut neg_count = u32::MAX;
        assert_eq!(cavc_plinelist_get_count(pos_plines, &mut pos_count), 0);
        assert_eq!(cavc_plinelist_get_count(neg_plines, &mut neg_count), 0);
        assert_eq!(pos_count, 0);
        assert_eq!(neg_count, 0);

        cavc_plinelist_f(pos_plines as *mut _);
        cavc_plinelist_f(neg_plines as *mut _);
        cavc_pline_f(pline_a);
        cavc_pline_f(pline_b);
    }
}

#[test]
fn pline_boolean_circle_rectangle_cpp_matrix_parity() {
    let pline_a = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);
    let pline_b = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );

    // FFI operation mapping:
    // 0 = Or, 1 = And, 2 = Not, 3 = Xor
    let cases = [
        BooleanCase {
            name: "circle_rectangle_union",
            operation: 0,
            expected_remaining: vec![PlineProps::new(
                10,
                109.15381629282,
                52.324068506275,
                0.0,
                -10.0,
                10.0,
                10.0,
            )],
            expected_subtracted: vec![],
        },
        BooleanCase {
            name: "circle_rectangle_exclude",
            operation: 2,
            expected_remaining: vec![
                PlineProps::new(
                    3,
                    29.336980664548,
                    23.492343031178,
                    6.0,
                    -3.8989794855664,
                    10.0,
                    5.898979485566356,
                ),
                PlineProps::new(
                    3,
                    19.816835628274,
                    20.757946197186,
                    0.0,
                    -3.582575694955841,
                    3.0,
                    5.5825756949558,
                ),
            ],
            expected_subtracted: vec![],
        },
        BooleanCase {
            name: "circle_rectangle_intersect",
            operation: 1,
            expected_remaining: vec![PlineProps::new(
                4,
                29.386000046924,
                25.091858029623,
                3.0,
                -4.0,
                6.0,
                6.0,
            )],
            expected_subtracted: vec![],
        },
        BooleanCase {
            name: "circle_rectangle_xor",
            operation: 3,
            expected_remaining: vec![
                PlineProps::new(
                    3,
                    19.816835628274,
                    20.757946197186,
                    0.0,
                    -3.582575694955841,
                    3.0,
                    5.5825756949558,
                ),
                PlineProps::new(
                    4,
                    -18.306999976538,
                    18.582818653767,
                    3.0,
                    -10.0,
                    6.0,
                    -3.5825756949558,
                ),
                PlineProps::new(
                    3,
                    29.336980664548,
                    23.492343031178,
                    6.0,
                    -3.8989794855664,
                    10.0,
                    5.898979485566356,
                ),
                PlineProps::new(
                    4,
                    -12.306999976538,
                    14.582818653767,
                    3.0,
                    5.5825756949558,
                    6.0,
                    10.0,
                ),
            ],
            expected_subtracted: vec![],
        },
    ];

    for case in cases {
        let (remaining, subtracted) = run_boolean_props(pline_a, pline_b, case.operation);
        assert!(
            props_set_match_ignore_area_sign(&remaining, &case.expected_remaining, 1e-4),
            "remaining property mismatch for case={}\nremaining={remaining:?}\nexpected={:?}",
            case.name,
            case.expected_remaining
        );
        assert!(
            props_set_match_ignore_area_sign(&subtracted, &case.expected_subtracted, 1e-4),
            "subtracted property mismatch for case={}\nsubtracted={subtracted:?}\nexpected={:?}",
            case.name,
            case.expected_subtracted
        );
    }

    unsafe {
        cavc_pline_f(pline_a);
        cavc_pline_f(pline_b);
    }
}

#[test]
fn pline_boolean_coincident_case2_cpp_matrix_parity() {
    let pline_a = create_pline(
        &[
            (0.0, 0.0, 0.0),
            (0.0, 20.0, 0.0),
            (20.0, 20.0, 0.0),
            (20.0, 0.0, 0.0),
        ],
        true,
    );
    let pline_b = create_pline(
        &[
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
            (30.0, 20.0, 0.0),
        ],
        true,
    );

    let cases = [
        BooleanCaseWithInputs {
            name: "coincident_case2_union",
            subject: pline_a,
            clip: pline_b,
            operation: 0,
            expected_remaining: vec![PlineProps::new(
                16,
                -865.0,
                150.17204220292,
                -2.0,
                0.0,
                30.0,
                40.0,
            )],
            expected_subtracted: vec![],
        },
        BooleanCaseWithInputs {
            name: "coincident_case2_excludeAFromB",
            subject: pline_a,
            clip: pline_b,
            operation: 2,
            expected_remaining: vec![
                PlineProps::new(4, -275.0, 68.4538182678, 0.0, 0.0, 20.0, 16.875),
                PlineProps::new(4, -10.0, 14.0, 6.0, 15.0, 8.0, 20.0),
            ],
            expected_subtracted: vec![],
        },
        BooleanCaseWithInputs {
            name: "coincident_case2_excludeBFromA",
            subject: pline_b,
            clip: pline_a,
            operation: 2,
            expected_remaining: vec![
                PlineProps::new(4, -19.375, 23.47038182678, -2.0, 10.0, 0.0, 20.0),
                PlineProps::new(6, -435.625, 85.701660376142, 8.0, 16.875, 30.0, 40.0),
                PlineProps::new(4, -10.0, 14.0, 2.0, 20.0, 4.0, 25.0),
            ],
            expected_subtracted: vec![],
        },
        BooleanCaseWithInputs {
            name: "coincident_case2_intersect",
            subject: pline_a,
            clip: pline_b,
            operation: 1,
            expected_remaining: vec![PlineProps::new(
                10,
                -115.0,
                63.4538182678,
                0.0,
                10.625,
                20.0,
                20.0,
            )],
            expected_subtracted: vec![],
        },
        BooleanCaseWithInputs {
            name: "coincident_case2_xor",
            subject: pline_a,
            clip: pline_b,
            operation: 3,
            expected_remaining: vec![
                PlineProps::new(4, -19.375, 23.47038182678, -2.0, 10.0, 0.0, 20.0),
                PlineProps::new(6, -435.625, 85.701660376142, 8.0, 16.875, 30.0, 40.0),
                PlineProps::new(4, -10.0, 14.0, 2.0, 20.0, 4.0, 25.0),
                PlineProps::new(4, 275.0, 68.4538182678, 0.0, 0.0, 20.0, 16.875),
                PlineProps::new(4, 10.0, 14.0, 6.0, 15.0, 8.0, 20.0),
            ],
            expected_subtracted: vec![],
        },
    ];

    for case in cases {
        let (remaining, subtracted) = run_boolean_props(case.subject, case.clip, case.operation);
        assert!(
            props_set_match_ignore_area_sign(&remaining, &case.expected_remaining, 1e-4),
            "remaining property mismatch for case={}\nremaining={remaining:?}\nexpected={:?}",
            case.name,
            case.expected_remaining
        );
        assert!(
            props_set_match_ignore_area_sign(&subtracted, &case.expected_subtracted, 1e-4),
            "subtracted property mismatch for case={}\nsubtracted={subtracted:?}\nexpected={:?}",
            case.name,
            case.expected_subtracted
        );
    }

    unsafe {
        cavc_pline_f(pline_a);
        cavc_pline_f(pline_b);
    }
}

#[test]
fn pline_boolean_does_not_modify_input_cpp_parity() {
    // old C++ source: TEST_cavc_combine_plines.cpp -> combine_plines_does_not_modify_input
    // mirror simple circle/rectangle operation matrix
    let ops = [0_u32, 2_u32, 1_u32, 3_u32]; // Or, Not, And, Xor

    for operation in ops {
        let pline_a = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);
        let pline_b = create_pline(
            &[
                (3.0, -10.0, 0.0),
                (6.0, -10.0, 0.0),
                (6.0, 10.0, 0.0),
                (3.0, 10.0, 0.0),
            ],
            true,
        );

        let before_a = read_vertices(pline_a);
        let before_b = read_vertices(pline_b);

        let _ = run_boolean_props(pline_a, pline_b, operation);

        let after_a = read_vertices(pline_a);
        let after_b = read_vertices(pline_b);
        compare_vertexes(&after_a, &before_a);
        compare_vertexes(&after_b, &before_b);

        unsafe {
            cavc_pline_f(pline_a);
            cavc_pline_f(pline_b);
        }
    }
}

#[test]
fn pline_boolean_coincident_matrices_do_not_modify_input_cpp_parity() {
    struct NoModifyCase {
        name: &'static str,
        operation: u32,
        subject: Vec<(f64, f64, f64)>,
        clip: Vec<(f64, f64, f64)>,
    }

    // FFI operation mapping:
    // 0 = Or, 1 = And, 2 = Not, 3 = Xor
    let (case1_a, case1_b) = cpp_coincident_case1_inputs();
    let (case2_a, case2_b) = cpp_coincident_case2_inputs();
    let cases = vec![
        NoModifyCase {
            name: "coincident_case1_union",
            operation: 0,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case1_exclude_a_from_b",
            operation: 2,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case1_exclude_b_from_a",
            operation: 2,
            subject: case1_b.clone(),
            clip: case1_a.clone(),
        },
        NoModifyCase {
            name: "coincident_case1_intersect",
            operation: 1,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case1_xor",
            operation: 3,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case2_union",
            operation: 0,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case2_exclude_a_from_b",
            operation: 2,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case2_exclude_b_from_a",
            operation: 2,
            subject: case2_b.clone(),
            clip: case2_a.clone(),
        },
        NoModifyCase {
            name: "coincident_case2_intersect",
            operation: 1,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case2_xor",
            operation: 3,
            subject: case2_a,
            clip: case2_b,
        },
    ];

    for case in cases {
        let pline_a = create_pline(&case.subject, true);
        let pline_b = create_pline(&case.clip, true);
        let before_a = read_vertices(pline_a);
        let before_b = read_vertices(pline_b);

        let _ = run_boolean_props(pline_a, pline_b, case.operation);

        let after_a = read_vertices(pline_a);
        let after_b = read_vertices(pline_b);

        assert_eq!(
            before_a.len(),
            after_a.len(),
            "subject vertex count changed for case={}",
            case.name
        );
        assert_eq!(
            before_b.len(),
            after_b.len(),
            "clip vertex count changed for case={}",
            case.name
        );
        compare_vertexes(&after_a, &before_a);
        compare_vertexes(&after_b, &before_b);

        unsafe {
            cavc_pline_f(pline_a);
            cavc_pline_f(pline_b);
        }
    }
}

#[test]
fn pline_boolean_combine_with_self_invariants_cpp_parity() {
    let pline = create_pline(
        &[
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
            (27.804688, 1.25, 0.414214),
        ],
        true,
    );

    let mut rev_pline = ptr::null();
    unsafe {
        assert_eq!(cavc_pline_clone(pline, &mut rev_pline), 0);
        assert_eq!(cavc_pline_invert_direction(rev_pline as *mut _), 0);
    }

    let pline_props_expected = pline_props(pline);
    let rev_pline_props_expected = pline_props(rev_pline);

    // Union (0) with self is self.
    let (remaining, subtracted) = run_boolean_props(pline, pline, 0);
    assert!(props_set_match_ignore_area_sign(
        &remaining,
        &[pline_props_expected],
        1e-4
    ));
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_props(rev_pline, rev_pline, 0);
    assert!(props_set_match_ignore_area_sign(
        &remaining,
        &[rev_pline_props_expected],
        1e-4
    ));
    assert!(subtracted.is_empty());

    // Exclude (Not = 2) with self is empty.
    let (remaining, subtracted) = run_boolean_props(pline, pline, 2);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_props(rev_pline, rev_pline, 2);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_props(rev_pline, pline, 2);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_props(pline, rev_pline, 2);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    // Intersect (And = 1) with self is self.
    let (remaining, subtracted) = run_boolean_props(pline, pline, 1);
    assert!(props_set_match_ignore_area_sign(
        &remaining,
        &[pline_props_expected],
        1e-4
    ));
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_props(rev_pline, rev_pline, 1);
    assert!(props_set_match_ignore_area_sign(
        &remaining,
        &[rev_pline_props_expected],
        1e-4
    ));
    assert!(subtracted.is_empty());

    // XOR (3) with self is empty.
    let (remaining, subtracted) = run_boolean_props(pline, pline, 3);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_props(rev_pline, rev_pline, 3);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_props(rev_pline, pline, 3);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_props(pline, rev_pline, 3);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    unsafe {
        cavc_pline_f(pline);
        cavc_pline_f(rev_pline as *mut _);
    }
}

#[test]
fn pline_parallel_offset_cpp_simple_matrix_parity() {
    for case in cpp_offset_simple_cases() {
        let pline = create_pline(&case.input, case.is_closed);
        let actual = run_parallel_offset_props(pline, case.delta);
        assert!(
            props_set_match_ignore_area_sign(&actual, &case.expected, 1e-4),
            "parallel offset simple-case mismatch for {}\nactual={actual:?}\nexpected={:?}",
            case.name,
            case.expected
        );
        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_parallel_offset_cpp_specific_matrix_parity() {
    for case in cpp_offset_specific_cases() {
        let pline = create_pline(&case.input, case.is_closed);
        let actual = run_parallel_offset_props(pline, case.delta);
        assert!(
            props_set_match_ignore_area_sign(&actual, &case.expected, 1e-4),
            "parallel offset specific-case mismatch for {}\nactual={actual:?}\nexpected={:?}",
            case.name,
            case.expected
        );
        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_parallel_offset_cpp_reversed_matrix_parity() {
    for case in cpp_offset_simple_cases()
        .into_iter()
        .chain(cpp_offset_specific_cases())
    {
        let pline = create_pline(&case.input, case.is_closed);
        unsafe {
            assert_eq!(cavc_pline_invert_direction(pline), 0);
        }
        let delta = -case.delta;
        let expected: Vec<PlineProps> = case
            .expected
            .into_iter()
            .map(|p| {
                PlineProps::new(
                    p.vertex_count,
                    -p.area,
                    p.path_length,
                    p.min_x,
                    p.min_y,
                    p.max_x,
                    p.max_y,
                )
            })
            .collect();
        let actual = run_parallel_offset_props(pline, delta);
        assert!(
            props_set_match_ignore_area_sign(&actual, &expected, 1e-4),
            "parallel offset reversed-case mismatch for {}\nactual={actual:?}\nexpected={expected:?}",
            case.name
        );
        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_parallel_offset_does_not_modify_input_cpp_parity() {
    let case = &cpp_offset_simple_cases()[0];
    let pline = create_pline(&case.input, case.is_closed);
    let before = read_vertices(pline);
    let _ = run_parallel_offset_props(pline, case.delta);
    let after = read_vertices(pline);
    compare_vertexes(&after, &before);
    unsafe {
        cavc_pline_f(pline);
    }
}

#[test]
fn pline_boolean_options_path_circle_rectangle_cpp_parity() {
    let pline_a = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);
    let pline_b = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );

    let mut options = cavc_pline_boolean_o {
        pline1_aabb_index: std::ptr::null(),
        pos_equal_eps: f64::NAN,
        collapsed_area_eps: f64::NAN,
    };

    unsafe {
        assert_eq!(cavc_pline_boolean_o_init(&mut options), 0);
        let mut aabb1 = ptr::null();
        assert_eq!(cavc_pline_create_approx_aabbindex(pline_a, &mut aabb1), 0);
        options.pline1_aabb_index = aabb1;

        for operation in [0_u32, 2_u32, 1_u32, 3_u32] {
            let (default_remaining, default_subtracted) =
                run_boolean_props(pline_a, pline_b, operation);
            let (opt_remaining, opt_subtracted) =
                run_boolean_props_with_options(pline_a, pline_b, operation, &options);

            assert!(
                props_set_match_ignore_area_sign(&opt_remaining, &default_remaining, 1e-4)
                    && props_set_match_ignore_area_sign(&default_remaining, &opt_remaining, 1e-4),
                "boolean options remaining mismatch for op={operation}\ndefault={default_remaining:?}\noptions={opt_remaining:?}"
            );
            assert!(
                props_set_match_ignore_area_sign(&opt_subtracted, &default_subtracted, 1e-4)
                    && props_set_match_ignore_area_sign(&default_subtracted, &opt_subtracted, 1e-4),
                "boolean options subtracted mismatch for op={operation}\ndefault={default_subtracted:?}\noptions={opt_subtracted:?}"
            );
        }

        cavc_aabbindex_f(aabb1 as *mut _);
        cavc_pline_f(pline_a);
        cavc_pline_f(pline_b);
    }
}

#[test]
fn pline_boolean_options_coincident_case1_intersect_collapsed_filter_cpp_parity() {
    let (subject, clip) = cpp_coincident_case1_inputs();
    let pline_a = create_pline(&subject, true);
    let pline_b = create_pline(&clip, true);
    let mut options = cavc_pline_boolean_o {
        pline1_aabb_index: std::ptr::null(),
        pos_equal_eps: f64::NAN,
        collapsed_area_eps: f64::NAN,
    };

    unsafe {
        assert_eq!(cavc_pline_boolean_o_init(&mut options), 0);
        let mut aabb_index = ptr::null();
        assert_eq!(
            cavc_pline_create_approx_aabbindex(pline_a, &mut aabb_index),
            0
        );
        options.pline1_aabb_index = aabb_index;
        options.collapsed_area_eps = 1e-4;

        let (remaining, subtracted) = run_boolean_props_with_options(pline_a, pline_b, 1, &options);
        assert!(
            remaining.is_empty() && subtracted.is_empty(),
            "expected empty intersect with collapsed-area filter\nremaining={remaining:?}\nsubtracted={subtracted:?}"
        );

        cavc_aabbindex_f(aabb_index as *mut _);
        cavc_pline_f(pline_a);
        cavc_pline_f(pline_b);
    }
}

#[test]
fn pline_boolean_options_coincident_matrices_do_not_modify_input_cpp_parity() {
    struct NoModifyCase {
        name: &'static str,
        operation: u32,
        subject: PlineInput,
        clip: PlineInput,
    }

    let (case1_a, case1_b) = cpp_coincident_case1_inputs();
    let (case2_a, case2_b) = cpp_coincident_case2_inputs();
    let cases = vec![
        NoModifyCase {
            name: "coincident_case1_union",
            operation: 0,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case1_exclude_a_from_b",
            operation: 2,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case1_exclude_b_from_a",
            operation: 2,
            subject: case1_b.clone(),
            clip: case1_a.clone(),
        },
        NoModifyCase {
            name: "coincident_case1_intersect",
            operation: 1,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case1_xor",
            operation: 3,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case2_union",
            operation: 0,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case2_exclude_a_from_b",
            operation: 2,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case2_exclude_b_from_a",
            operation: 2,
            subject: case2_b.clone(),
            clip: case2_a.clone(),
        },
        NoModifyCase {
            name: "coincident_case2_intersect",
            operation: 1,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        NoModifyCase {
            name: "coincident_case2_xor",
            operation: 3,
            subject: case2_a,
            clip: case2_b,
        },
    ];

    for case in cases {
        let pline_a = create_pline(&case.subject, true);
        let pline_b = create_pline(&case.clip, true);
        let before_a = read_vertices(pline_a);
        let before_b = read_vertices(pline_b);

        let mut options = cavc_pline_boolean_o {
            pline1_aabb_index: std::ptr::null(),
            pos_equal_eps: f64::NAN,
            collapsed_area_eps: f64::NAN,
        };

        unsafe {
            assert_eq!(cavc_pline_boolean_o_init(&mut options), 0);
            let mut aabb_index = ptr::null();
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline_a, &mut aabb_index),
                0
            );
            options.pline1_aabb_index = aabb_index;
            let _ = run_boolean_props_with_options(pline_a, pline_b, case.operation, &options);

            let after_a = read_vertices(pline_a);
            let after_b = read_vertices(pline_b);
            assert_eq!(
                before_a.len(),
                after_a.len(),
                "subject vertex count changed for case={}",
                case.name
            );
            assert_eq!(
                before_b.len(),
                after_b.len(),
                "clip vertex count changed for case={}",
                case.name
            );
            compare_vertexes(&after_a, &before_a);
            compare_vertexes(&after_b, &before_b);

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline_a);
            cavc_pline_f(pline_b);
        }
    }
}

#[test]
fn pline_boolean_options_coincident_matrices_output_cpp_parity() {
    struct OutputParityCase {
        name: &'static str,
        operation: u32,
        subject: PlineInput,
        clip: PlineInput,
    }

    let (case1_a, case1_b) = cpp_coincident_case1_inputs();
    let (case2_a, case2_b) = cpp_coincident_case2_inputs();
    let cases = vec![
        OutputParityCase {
            name: "coincident_case1_union",
            operation: 0,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        OutputParityCase {
            name: "coincident_case1_exclude_a_from_b",
            operation: 2,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        OutputParityCase {
            name: "coincident_case1_exclude_b_from_a",
            operation: 2,
            subject: case1_b.clone(),
            clip: case1_a.clone(),
        },
        OutputParityCase {
            name: "coincident_case1_intersect",
            operation: 1,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        OutputParityCase {
            name: "coincident_case1_xor",
            operation: 3,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        OutputParityCase {
            name: "coincident_case2_union",
            operation: 0,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        OutputParityCase {
            name: "coincident_case2_exclude_a_from_b",
            operation: 2,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        OutputParityCase {
            name: "coincident_case2_exclude_b_from_a",
            operation: 2,
            subject: case2_b.clone(),
            clip: case2_a.clone(),
        },
        OutputParityCase {
            name: "coincident_case2_intersect",
            operation: 1,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        OutputParityCase {
            name: "coincident_case2_xor",
            operation: 3,
            subject: case2_a,
            clip: case2_b,
        },
    ];

    for case in cases {
        let pline_a = create_pline(&case.subject, true);
        let pline_b = create_pline(&case.clip, true);
        let (default_remaining, default_subtracted) =
            run_boolean_props(pline_a, pline_b, case.operation);

        let mut options = cavc_pline_boolean_o {
            pline1_aabb_index: std::ptr::null(),
            pos_equal_eps: f64::NAN,
            collapsed_area_eps: f64::NAN,
        };

        unsafe {
            assert_eq!(cavc_pline_boolean_o_init(&mut options), 0);
            let mut aabb_index = ptr::null();
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline_a, &mut aabb_index),
                0
            );
            options.pline1_aabb_index = aabb_index;

            let (opt_remaining, opt_subtracted) =
                run_boolean_props_with_options(pline_a, pline_b, case.operation, &options);

            assert!(
                props_set_match_ignore_area_sign(&opt_remaining, &default_remaining, 1e-4)
                    && props_set_match_ignore_area_sign(&default_remaining, &opt_remaining, 1e-4),
                "coincident options output remaining mismatch for {}\ndefault={default_remaining:?}\noptions={opt_remaining:?}",
                case.name
            );
            assert!(
                props_set_match_ignore_area_sign(&opt_subtracted, &default_subtracted, 1e-4)
                    && props_set_match_ignore_area_sign(&default_subtracted, &opt_subtracted, 1e-4),
                "coincident options output subtracted mismatch for {}\ndefault={default_subtracted:?}\noptions={opt_subtracted:?}",
                case.name
            );

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline_a);
            cavc_pline_f(pline_b);
        }
    }
}

#[test]
fn pline_parallel_offset_options_path_cpp_matrix_parity() {
    for case in cpp_offset_simple_cases()
        .into_iter()
        .chain(cpp_offset_specific_cases())
    {
        let pline = create_pline(&case.input, case.is_closed);
        let default_props = run_parallel_offset_props(pline, case.delta);

        let mut options = cavc_pline_parallel_offset_o {
            aabb_index: std::ptr::null(),
            pos_equal_eps: f64::NAN,
            slice_join_eps: f64::NAN,
            offset_dist_eps: f64::NAN,
            handle_self_intersects: 0,
        };

        unsafe {
            assert_eq!(cavc_pline_parallel_offset_o_init(&mut options), 0);
            let mut aabb_index = ptr::null();
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
                0
            );
            options.aabb_index = aabb_index;

            let option_props = run_parallel_offset_props_with_options(pline, case.delta, &options);
            assert!(
                props_set_match_ignore_area_sign(&option_props, &default_props, 1e-4)
                    && props_set_match_ignore_area_sign(&default_props, &option_props, 1e-4),
                "parallel offset options mismatch for {}\ndefault={default_props:?}\noptions={option_props:?}",
                case.name
            );

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn shape_eval_ffi() {
    let outer_pline = create_pline(
        &[
            (-200.0, 200.0, 0.0),
            (-200.0, -200.0, 0.0),
            (200.0, -200.0, 0.0),
            (200.0, 200.0, 0.0),
        ],
        true,
    );

    let inner_pline = create_pline(
        &[
            (-100.0, 0.0, 0.0),
            (0.0, 100.0, 0.0),
            (100.0, 0.0, 0.0),
            (0.0, -100.0, 0.0),
        ],
        true,
    );

    let expected_65 = [
        vec![
            cavc_vertex {
                x: -135.0,
                y: -56.92388155425118,
                bulge: 0.0,
            },
            cavc_vertex {
                x: -135.0,
                y: -135.0,
                bulge: 0.0,
            },
            cavc_vertex {
                x: -56.92388155425118,
                y: -135.0,
                bulge: 0.0,
            },
        ],
        vec![
            cavc_vertex {
                x: 56.92388155425118,
                y: -135.0,
                bulge: 0.0,
            },
            cavc_vertex {
                x: 135.0,
                y: -135.0,
                bulge: 0.0,
            },
            cavc_vertex {
                x: 135.0,
                y: -56.92388155425118,
                bulge: 0.0,
            },
        ],
        vec![
            cavc_vertex {
                x: 135.0,
                y: 56.92388155425118,
                bulge: 0.0,
            },
            cavc_vertex {
                x: 135.0,
                y: 135.0,
                bulge: 0.0,
            },
            cavc_vertex {
                x: 56.92388155425118,
                y: 135.0,
                bulge: 0.0,
            },
        ],
        vec![
            cavc_vertex {
                x: -56.92388155425118,
                y: 135.0,
                bulge: 0.0,
            },
            cavc_vertex {
                x: -135.0,
                y: 135.0,
                bulge: 0.0,
            },
            cavc_vertex {
                x: -135.0,
                y: 56.92388155425118,
                bulge: 0.0,
            },
        ],
    ];

    let expected_40_ccw = vec![
        cavc_vertex {
            x: -160.0,
            y: 160.0,
            bulge: 0.0,
        },
        cavc_vertex {
            x: -160.0,
            y: -160.0,
            bulge: 0.0,
        },
        cavc_vertex {
            x: 160.0,
            y: -160.0,
            bulge: 0.0,
        },
        cavc_vertex {
            x: 160.0,
            y: 160.0,
            bulge: 0.0,
        },
    ];

    let expected_40_cw = vec![
        cavc_vertex {
            x: -128.2842712474619,
            y: 28.284271247461902,
            bulge: 0.0,
        },
        cavc_vertex {
            x: -28.284271247461902,
            y: 128.2842712474619,
            bulge: -0.4142135623730951,
        },
        cavc_vertex {
            x: 28.284271247461902,
            y: 128.2842712474619,
            bulge: 0.0,
        },
        cavc_vertex {
            x: 128.2842712474619,
            y: 28.284271247461902,
            bulge: -0.4142135623730951,
        },
        cavc_vertex {
            x: 128.2842712474619,
            y: -28.284271247461902,
            bulge: 0.0,
        },
        cavc_vertex {
            x: 28.284271247461902,
            y: -128.2842712474619,
            bulge: -0.4142135623730951,
        },
        cavc_vertex {
            x: -28.284271247461902,
            y: -128.2842712474619,
            bulge: 0.0,
        },
        cavc_vertex {
            x: -128.2842712474619,
            y: -28.284271247461902,
            bulge: -0.4142135623730951,
        },
    ];

    unsafe {
        {
            assert_eq!(
                cavc_pline_set_userdata_values(ptr::null_mut(), ptr::null(), 0),
                1
            );
            assert_eq!(
                cavc_pline_get_userdata_count(ptr::null(), ptr::null_mut()),
                1
            );
            assert_eq!(
                cavc_pline_get_userdata_values(ptr::null(), ptr::null_mut()),
                1
            );

            assert_eq!(
                cavc_shape_set_ccw_pline_userdata_values(ptr::null_mut(), 0, ptr::null(), 0),
                1
            );
            assert_eq!(
                cavc_shape_get_ccw_pline_userdata_count(ptr::null(), 0, ptr::null_mut()),
                1
            );

            assert_eq!(cavc_pline_set_userdata_values(outer_pline, &117u64, 1), 0);
            assert_eq!(cavc_pline_set_userdata_values(inner_pline, &4u64, 1), 0);

            let mut count: u32 = 0xDEAD;
            assert_eq!(cavc_pline_get_userdata_count(outer_pline, &mut count), 0);
            assert_eq!(count, 1);
            assert_eq!(cavc_pline_get_userdata_count(inner_pline, &mut count), 0);
            assert_eq!(count, 1);

            let mut userdata = 0xDEADBEEF_u64;
            assert_eq!(
                cavc_pline_get_userdata_values(outer_pline, &mut userdata),
                0
            );
            assert_eq!(userdata, 117);
            assert_eq!(
                cavc_pline_get_userdata_values(inner_pline, &mut userdata),
                0
            );
            assert_eq!(userdata, 4);
        }

        {
            // Full-stack shape offset operation check with default offset options
            let mut list = ptr::null_mut();
            assert_eq!(cavc_plinelist_create(0, &mut list), 0);

            assert_eq!(cavc_plinelist_push(ptr::null_mut(), outer_pline), 1);
            assert_eq!(cavc_plinelist_push(list, ptr::null_mut()), 2);
            assert_eq!(cavc_plinelist_push(list, outer_pline), 0);
            assert_eq!(cavc_plinelist_push(list, inner_pline), 0);

            let mut shape = ptr::null_mut();
            assert_eq!(cavc_shape_create(ptr::null(), &mut shape), 1);
            assert_eq!(cavc_shape_create(list, &mut shape), 0);

            assert_eq!(cavc_plinelist_pop(list, ptr::null_mut()), 0); // The plines in the list (pointed to by outer_pline and inner_pline) will be re-used later.
            assert_eq!(cavc_plinelist_pop(list, ptr::null_mut()), 0);
            cavc_plinelist_f(list);

            let mut ccw_count: u32 = 0xDEAD;
            assert_eq!(cavc_shape_get_ccw_count(shape, &mut ccw_count), 0);
            assert_eq!(ccw_count, 1);

            let mut cw_count: u32 = 0xDEAD;
            assert_eq!(cavc_shape_get_cw_count(shape, &mut cw_count), 0);
            assert_eq!(cw_count, 1);

            let mut result_shape = ptr::null_mut();
            assert_eq!(
                cavc_shape_parallel_offset(ptr::null(), 65.0, ptr::null(), &mut result_shape),
                1
            );
            assert_eq!(
                cavc_shape_parallel_offset(shape, 65.0, ptr::null(), &mut result_shape),
                0
            );

            ccw_count = 0xDEAD;
            assert_eq!(cavc_shape_get_ccw_count(result_shape, &mut ccw_count), 0);
            assert_eq!(ccw_count, 4);

            cw_count = 0xDEAD;
            assert_eq!(cavc_shape_get_cw_count(result_shape, &mut cw_count), 0);
            assert_eq!(cw_count, 0);

            for index in 0..4 {
                let mut is_closed: u8 = 0;
                assert_eq!(
                    cavc_shape_get_ccw_polyline_is_closed(result_shape, index, &mut is_closed),
                    0
                );
                assert_ne!(is_closed, 0);

                let mut ccw_vertex_count: u32 = 0;
                assert_eq!(
                    cavc_shape_get_ccw_polyline_count(result_shape, index, &mut ccw_vertex_count),
                    0
                );
                assert_eq!(ccw_vertex_count, 3);

                let mut vertexes: Vec<cavc_vertex> = Vec::with_capacity(3);
                vertexes.resize(3, cavc_vertex::new(0.0, 0.0, 0.0));

                assert_eq!(
                    cavc_shape_get_ccw_polyline_vertex_data(
                        result_shape,
                        index,
                        vertexes.as_mut_ptr()
                    ),
                    0
                );

                compare_vertexes(&vertexes, &(expected_65[index as usize]));

                let mut userdata_count: u32 = 0xDEAD;
                assert_eq!(
                    cavc_shape_get_ccw_pline_userdata_count(
                        result_shape,
                        index,
                        &mut userdata_count
                    ),
                    0
                );
                assert_eq!(userdata_count, 2);

                let mut userdata = [0xDEADBEEF_u64, 0xDEADBEEF_u64];
                assert_eq!(
                    cavc_shape_get_ccw_pline_userdata_values(
                        result_shape,
                        index,
                        &mut (userdata[0])
                    ),
                    0
                );

                assert!(userdata.contains(&117));
                assert!(userdata.contains(&4));
            }

            cavc_shape_f(shape);
            cavc_shape_f(result_shape);
        }

        {
            // Full-stack shape offset operation check with custom offset options
            let mut list = ptr::null_mut();
            assert_eq!(cavc_plinelist_create(0, &mut list), 0);

            assert_eq!(cavc_plinelist_push(list, outer_pline), 0);
            assert_eq!(cavc_plinelist_push(list, inner_pline), 0);

            let mut shape = ptr::null_mut();
            assert_eq!(cavc_shape_create(list, &mut shape), 0);

            assert_eq!(cavc_plinelist_pop(list, ptr::null_mut()), 0); // The plines in the list (pointed to by outer_pline and inner_pline) will be re-used later.
            assert_eq!(cavc_plinelist_pop(list, ptr::null_mut()), 0);
            cavc_plinelist_f(list);

            let mut ccw_count: u32 = 0xDEAD;
            assert_eq!(cavc_shape_get_ccw_count(shape, &mut ccw_count), 0);
            assert_eq!(ccw_count, 1);

            let mut cw_count: u32 = 0xDEAD;
            assert_eq!(cavc_shape_get_cw_count(shape, &mut cw_count), 0);
            assert_eq!(cw_count, 1);

            let offset_options = cavc_shape_offset_o {
                pos_equal_eps: 0.0001,
                offset_dist_eps: 0.001,
                slice_join_eps: 0.001,
            };
            let mut result_shape = ptr::null_mut();
            assert_eq!(
                cavc_shape_parallel_offset(shape, 65.0, &offset_options, &mut result_shape),
                0
            );

            ccw_count = 0xDEAD;
            assert_eq!(cavc_shape_get_ccw_count(result_shape, &mut ccw_count), 0);
            assert_eq!(ccw_count, 4);

            cw_count = 0xDEAD;
            assert_eq!(cavc_shape_get_cw_count(result_shape, &mut cw_count), 0);
            assert_eq!(cw_count, 0);

            for index in 0..4 {
                let mut is_closed: u8 = 0;
                assert_eq!(
                    cavc_shape_get_ccw_polyline_is_closed(result_shape, index, &mut is_closed),
                    0
                );
                assert_ne!(is_closed, 0);

                let mut ccw_vertex_count: u32 = 0;
                assert_eq!(
                    cavc_shape_get_ccw_polyline_count(result_shape, index, &mut ccw_vertex_count),
                    0
                );
                assert_eq!(ccw_vertex_count, 3);

                let mut vertexes: Vec<cavc_vertex> = Vec::with_capacity(3);
                vertexes.resize(3, cavc_vertex::new(0.0, 0.0, 0.0));

                assert_eq!(
                    cavc_shape_get_ccw_polyline_vertex_data(
                        result_shape,
                        index,
                        vertexes.as_mut_ptr()
                    ),
                    0
                );

                compare_vertexes(&vertexes, &(expected_65[index as usize]));

                let mut userdata_count: u32 = 0xDEAD;
                assert_eq!(
                    cavc_shape_get_ccw_pline_userdata_count(
                        result_shape,
                        index,
                        &mut userdata_count
                    ),
                    0
                );
                assert_eq!(userdata_count, 2);

                let mut userdata = [0xDEADBEEF_u64, 0xDEADBEEF_u64];
                assert_eq!(
                    cavc_shape_get_ccw_pline_userdata_values(
                        result_shape,
                        index,
                        (&mut userdata) as *mut u64
                    ),
                    0
                );

                assert!(userdata.contains(&117));
                assert!(userdata.contains(&4));
            }

            cavc_shape_f(shape);
            cavc_shape_f(result_shape);
        }

        {
            // Full-stack shape offset operation with no intersection (generates one CCW path and one CW path)
            let mut list = ptr::null_mut();
            assert_eq!(cavc_plinelist_create(0, &mut list), 0);

            assert_eq!(cavc_plinelist_push(list, outer_pline), 0);
            assert_eq!(cavc_plinelist_push(list, inner_pline), 0);

            let mut shape = ptr::null_mut();
            assert_eq!(cavc_shape_create(list, &mut shape), 0);

            cavc_plinelist_f(list); // As this is the last use of outer_pline and inner_pline; we won't pop them before freeing the plinelist.

            let mut result_shape = ptr::null_mut();
            assert_eq!(
                cavc_shape_parallel_offset(shape, 40.0, ptr::null(), &mut result_shape),
                0
            );

            let mut ccw_count: u32 = 0;
            assert_eq!(cavc_shape_get_ccw_count(result_shape, &mut ccw_count), 0);
            assert_eq!(ccw_count, 1);

            let mut cw_count: u32 = 0;
            assert_eq!(cavc_shape_get_cw_count(result_shape, &mut cw_count), 0);
            assert_eq!(cw_count, 1);

            {
                // CCW result
                let mut is_closed: u8 = 0;
                assert_eq!(
                    cavc_shape_get_ccw_polyline_is_closed(result_shape, 0, &mut is_closed),
                    0
                );
                assert_ne!(is_closed, 0);

                let mut ccw_vertex_count: u32 = 0;
                assert_eq!(
                    cavc_shape_get_ccw_polyline_count(result_shape, 0, &mut ccw_vertex_count),
                    0
                );
                assert_eq!(ccw_vertex_count, 4);

                let mut vertexes: Vec<cavc_vertex> = Vec::with_capacity(4);
                vertexes.resize(4, cavc_vertex::new(0.0, 0.0, 0.0));

                assert_eq!(
                    cavc_shape_get_ccw_polyline_vertex_data(result_shape, 0, vertexes.as_mut_ptr()),
                    0
                );

                compare_vertexes(&vertexes, &expected_40_ccw);

                let mut userdata_count: u32 = 0xDEAD;
                assert_eq!(
                    cavc_shape_get_ccw_pline_userdata_count(result_shape, 0, &mut userdata_count),
                    0
                );
                assert_eq!(userdata_count, 1);

                let mut userdata = 0xDEADBEEF_u64;
                assert_eq!(
                    cavc_shape_get_ccw_pline_userdata_values(result_shape, 0, &mut userdata),
                    0
                );
                assert_eq!(userdata, 117);
            }

            {
                // CW result
                let mut is_closed: u8 = 0;
                assert_eq!(
                    cavc_shape_get_cw_polyline_is_closed(result_shape, 0, &mut is_closed),
                    0
                );
                assert_ne!(is_closed, 0);

                let mut cw_vertex_count: u32 = 0;
                assert_eq!(
                    cavc_shape_get_cw_polyline_count(result_shape, 0, &mut cw_vertex_count),
                    0
                );
                assert_eq!(cw_vertex_count, 8);

                let mut vertexes: Vec<cavc_vertex> = Vec::with_capacity(8);
                vertexes.resize(8, cavc_vertex::new(0.0, 0.0, 0.0));

                assert_eq!(
                    cavc_shape_get_cw_polyline_vertex_data(result_shape, 0, vertexes.as_mut_ptr()),
                    0
                );

                compare_vertexes(&vertexes, &expected_40_cw);

                let mut userdata_count: u32 = 0xDEAD;
                assert_eq!(
                    cavc_shape_get_cw_pline_userdata_count(result_shape, 0, &mut userdata_count),
                    0
                );
                assert_eq!(userdata_count, 1);

                let mut userdata = 0xDEADBEEF_u64;
                assert_eq!(
                    cavc_shape_get_cw_pline_userdata_values(result_shape, 0, &mut userdata),
                    0
                );
                assert_eq!(userdata, 4);
            }

            cavc_shape_f(shape);
            cavc_shape_f(result_shape);
        }
    }
}

#[test]
fn self_intersect_scan_ffi() {
    let hourglass = create_pline(
        &[
            (0.0, 2.0, 0.0),
            (1.0, 1.0, 0.0),
            (0.0, 1.0, 0.0),
            (1.0, 2.0, 0.0),
        ],
        true,
    );

    let rectangle = create_pline(
        &[
            (-2.0, -2.0, 0.0),
            (2.0, -2.0, 0.0),
            (2.0, 2.0, 0.0),
            (-2.0, 2.0, 0.0),
        ],
        true,
    );

    unsafe {
        let mut is_self_intersecting: u8 = 0;

        assert_eq!(
            cavc_pline_scan_for_self_intersect(hourglass, ptr::null(), &mut is_self_intersecting),
            0
        );
        assert_ne!(is_self_intersecting, 0);

        assert_eq!(
            cavc_pline_scan_for_self_intersect(rectangle, ptr::null(), &mut is_self_intersecting),
            0
        );
        assert_eq!(is_self_intersecting, 0);

        let mut hourglass_options: *mut cavc_pline_self_intersect_o = ptr::null_mut();
        let mut hourglass_index: *const cavc_aabbindex = ptr::null_mut();

        assert_eq!(
            cavc_pline_self_intersect_o_create(&mut hourglass_options),
            0
        );
        assert_eq!(
            cavc_pline_create_approx_aabbindex(hourglass, &mut hourglass_index),
            0
        );
        (*hourglass_options).pline_aabb_index = hourglass_index;

        assert_eq!(
            cavc_pline_scan_for_self_intersect(
                hourglass,
                hourglass_options,
                &mut is_self_intersecting
            ),
            0
        );
        assert_ne!(is_self_intersecting, 0);

        let mut rectangle_options: *mut cavc_pline_self_intersect_o = ptr::null_mut();
        let mut rectangle_index: *const cavc_aabbindex = ptr::null_mut();

        assert_eq!(
            cavc_pline_self_intersect_o_create(&mut rectangle_options),
            0
        );
        assert_eq!(
            cavc_pline_create_approx_aabbindex(rectangle, &mut rectangle_index),
            0
        );
        (*rectangle_options).pline_aabb_index = rectangle_index;

        assert_eq!(
            cavc_pline_scan_for_self_intersect(
                rectangle,
                rectangle_options,
                &mut is_self_intersecting
            ),
            0
        );
        assert_eq!(is_self_intersecting, 0);

        cavc_aabbindex_f(hourglass_index as *mut _);
        cavc_aabbindex_f(rectangle_index as *mut _);

        cavc_pline_self_intersect_o_f(hourglass_options);
        cavc_pline_self_intersect_o_f(rectangle_options);

        cavc_pline_f(hourglass);
        cavc_pline_f(rectangle);
    }
}

#[test]
fn pline_contains_ffi() {
    let rectangle = create_pline(
        &[
            (-2.0, -2.0, 0.0),
            (2.0, -2.0, 0.0),
            (2.0, 2.0, 0.0),
            (-2.0, 2.0, 0.0),
        ],
        true,
    );

    let circle = create_pline(&[(-1.0, 0.0, 1.0), (1.0, 0.0, 1.0)], true);

    let triangle = create_pline(
        &[(3.1340, 4.5, 0.0), (4.0, 3.0, 0.0), (4.8660, 4.5, 0.0)],
        true,
    );

    unsafe {
        let mut result: u32 = 0;

        assert_eq!(
            cavc_pline_contains(rectangle, circle, ptr::null(), &mut result as *mut u32),
            0
        );
        assert_eq!(result, CAVC_CONTAINS_RESULT_PLINE2_INSIDE_PLINE1);

        assert_eq!(
            cavc_pline_contains(circle, rectangle, ptr::null(), &mut result as *mut u32),
            0
        );
        assert_eq!(result, CAVC_CONTAINS_RESULT_PLINE1_INSIDE_PLINE2);

        assert_eq!(
            cavc_pline_contains(rectangle, triangle, ptr::null(), &mut result as *mut u32),
            0
        );
        assert_eq!(result, CAVC_CONTAINS_RESULT_DISJOINT);

        let mut rectangle_options: *mut cavc_pline_contains_o = ptr::null_mut();
        let mut rectangle_index: *const cavc_aabbindex = ptr::null_mut();

        assert_eq!(cavc_pline_contains_o_create(&mut rectangle_options), 0);
        assert_eq!(
            cavc_pline_create_approx_aabbindex(rectangle, &mut rectangle_index),
            0
        );
        (*rectangle_options).pline1_aabb_index = rectangle_index;

        assert_eq!(
            cavc_pline_contains(
                rectangle,
                circle,
                rectangle_options,
                &mut result as *mut u32
            ),
            0
        );
        assert_eq!(result, CAVC_CONTAINS_RESULT_PLINE2_INSIDE_PLINE1);

        let mut circle_options: *mut cavc_pline_contains_o = ptr::null_mut();
        let mut circle_index: *const cavc_aabbindex = ptr::null_mut();

        assert_eq!(cavc_pline_contains_o_create(&mut circle_options), 0);
        assert_eq!(
            cavc_pline_create_approx_aabbindex(circle, &mut circle_index),
            0
        );
        (*circle_options).pline1_aabb_index = circle_index;

        assert_eq!(
            cavc_pline_contains(circle, rectangle, circle_options, &mut result as *mut u32),
            0
        );
        assert_eq!(result, CAVC_CONTAINS_RESULT_PLINE1_INSIDE_PLINE2);

        assert_eq!(
            cavc_pline_contains(
                rectangle,
                triangle,
                rectangle_options,
                &mut result as *mut u32
            ),
            0
        );
        assert_eq!(result, CAVC_CONTAINS_RESULT_DISJOINT);

        cavc_aabbindex_f(rectangle_index as *mut _);
        cavc_aabbindex_f(circle_index as *mut _);

        cavc_pline_contains_o_f(rectangle_options);
        cavc_pline_contains_o_f(circle_options);

        cavc_pline_f(rectangle);
        cavc_pline_f(circle);
        cavc_pline_f(triangle);
    }
}
