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

fn plinelist_vertexes(plinelist: *const cavc_plinelist) -> Vec<Vec<cavc_vertex>> {
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
        result.push(read_vertices(pline));
    }

    result
}

fn run_boolean_vertexes(
    pline1: *const cavc_pline,
    pline2: *const cavc_pline,
    operation: u32,
) -> (Vec<Vec<cavc_vertex>>, Vec<Vec<cavc_vertex>>) {
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

        let pos = plinelist_vertexes(pos_plines);
        let neg = plinelist_vertexes(neg_plines);
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

fn run_boolean_vertexes_with_options(
    pline1: *const cavc_pline,
    pline2: *const cavc_pline,
    operation: u32,
    options: *const cavc_pline_boolean_o,
) -> (Vec<Vec<cavc_vertex>>, Vec<Vec<cavc_vertex>>) {
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

        let pos = plinelist_vertexes(pos_plines);
        let neg = plinelist_vertexes(neg_plines);
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

fn run_parallel_offset_vertexes(pline: *const cavc_pline, delta: f64) -> Vec<Vec<cavc_vertex>> {
    let mut results = ptr::null();
    unsafe {
        assert_eq!(
            cavc_pline_parallel_offset(pline, delta, ptr::null(), &mut results),
            0
        );
        let mut count = u32::MAX;
        assert_eq!(cavc_plinelist_get_count(results, &mut count), 0);
        let mut out = Vec::with_capacity(count as usize);
        for i in 0..count {
            let mut pline_out = ptr::null();
            assert_eq!(cavc_plinelist_get_pline(results, i, &mut pline_out), 0);
            out.push(read_vertices(pline_out));
        }
        cavc_plinelist_f(results as *mut _);
        out
    }
}

fn run_parallel_offset_vertexes_with_options(
    pline: *const cavc_pline,
    delta: f64,
    options: *const cavc_pline_parallel_offset_o,
) -> Vec<Vec<cavc_vertex>> {
    let mut results = ptr::null();
    unsafe {
        assert_eq!(
            cavc_pline_parallel_offset(pline, delta, options, &mut results),
            0
        );
        let mut count = u32::MAX;
        assert_eq!(cavc_plinelist_get_count(results, &mut count), 0);
        let mut out = Vec::with_capacity(count as usize);
        for i in 0..count {
            let mut pline_out = ptr::null();
            assert_eq!(cavc_plinelist_get_pline(results, i, &mut pline_out), 0);
            out.push(read_vertices(pline_out));
        }
        cavc_plinelist_f(results as *mut _);
        out
    }
}

fn init_parallel_offset_options() -> cavc_pline_parallel_offset_o {
    cavc_pline_parallel_offset_o {
        aabb_index: std::ptr::null(),
        pos_equal_eps: f64::NAN,
        slice_join_eps: f64::NAN,
        offset_dist_eps: f64::NAN,
        handle_self_intersects: 0,
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

fn vertex_fuzzy_eq(a: cavc_vertex, b: cavc_vertex) -> bool {
    (a.x - b.x).abs() <= CPP_MATRIX_EPS
        && (a.y - b.y).abs() <= CPP_MATRIX_EPS
        && (a.bulge - b.bulge).abs() <= CPP_MATRIX_EPS
}

fn open_vertexes_match_exact(actual: &[cavc_vertex], expected: &[cavc_vertex]) -> bool {
    actual.len() == expected.len()
        && actual
            .iter()
            .zip(expected.iter())
            .all(|(a, e)| vertex_fuzzy_eq(*a, *e))
}

fn closed_vertexes_match_with_rotation(actual: &[cavc_vertex], expected: &[cavc_vertex]) -> bool {
    if actual.len() != expected.len() {
        return false;
    }
    let n = expected.len();
    for shift in 0..n {
        let mut all_match = true;
        for (i, e) in expected.iter().enumerate() {
            let actual_i = (i + shift) % n;
            if !vertex_fuzzy_eq(actual[actual_i], *e) {
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

fn assert_single_offset_vertex_match(
    actual: &[Vec<cavc_vertex>],
    expected: &[cavc_vertex],
    is_closed: bool,
    context: &str,
) {
    assert_eq!(
        actual.len(),
        1,
        "{context}: expected one offset polyline, got {}",
        actual.len()
    );
    let matches = if is_closed {
        closed_vertexes_match_with_rotation(&actual[0], expected)
    } else {
        open_vertexes_match_exact(&actual[0], expected)
    };
    assert!(matches, "{context}: offset vertex mismatch");
}

fn vertex_lists_match_unordered(
    actual: &[Vec<cavc_vertex>],
    expected: &[Vec<cavc_vertex>],
    is_closed: bool,
) -> bool {
    if actual.len() != expected.len() {
        return false;
    }

    let mut used = vec![false; actual.len()];
    for expected_pline in expected {
        let mut found = false;
        for (i, actual_pline) in actual.iter().enumerate() {
            if used[i] {
                continue;
            }
            let matches = if is_closed {
                closed_vertexes_match_with_rotation(actual_pline, expected_pline)
            } else {
                open_vertexes_match_exact(actual_pline, expected_pline)
            };
            if matches {
                used[i] = true;
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }

    true
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

fn cpp_offset_specific_edge_matrix_cases() -> Vec<OffsetCase> {
    let mut cases = cpp_offset_specific_cases();
    let mut simple_cases = cpp_offset_simple_cases();
    for case_name in [
        "closed_rectangle_inward",
        "closed_rectangle_coincident",
        "open_rectangle_inward",
        "closed_rectangle_outward",
        "open_rectangle_outward",
        "closed_diamond_outward",
        "closed_diamond_inward",
        "open_diamond_inward",
        "open_diamond_outward",
    ] {
        let case_index = simple_cases
            .iter()
            .position(|case| case.name == case_name)
            .unwrap_or_else(|| panic!("missing source-backed edge case: {case_name}"));
        cases.push(simple_cases.remove(case_index));
    }
    assert!(
        simple_cases.is_empty(),
        "specific-edge matrix omitted source-backed simple case(s): {}",
        simple_cases
            .iter()
            .map(|case| case.name)
            .collect::<Vec<_>>()
            .join(", ")
    );
    cases
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

fn rotate_closed_input(input: &PlineInput, shift: usize) -> PlineInput {
    if input.is_empty() {
        return Vec::new();
    }

    let shift = shift % input.len();
    if shift == 0 {
        return input.clone();
    }

    let mut rotated = Vec::with_capacity(input.len());
    rotated.extend_from_slice(&input[shift..]);
    rotated.extend_from_slice(&input[..shift]);
    rotated
}

struct CoincidentMatrixCase {
    name: &'static str,
    operation: u32,
    subject: PlineInput,
    clip: PlineInput,
}

const CPP_COINCIDENT_CASE1_SOURCE_MATRIX: [(&str, u32); 5] = [
    ("coincident_case1_union", 0),
    ("coincident_case1_excludeAFromB", 2),
    ("coincident_case1_excludeBFromA", 2),
    ("coincident_case1_intersect", 1),
    ("coincident_case1_xor", 3),
];

const CPP_COINCIDENT_CASE2_SOURCE_MATRIX: [(&str, u32); 5] = [
    ("coincident_case2_union", 0),
    ("coincident_case2_excludeAFromB", 2),
    ("coincident_case2_excludeBFromA", 2),
    ("coincident_case2_intersect", 1),
    ("coincident_case2_xor", 3),
];

const CPP_CIRCLE_RECT_SOURCE_MATRIX: [(&str, u32); 4] = [
    ("circle_rectangle_union", 0),
    ("circle_rectangle_exclude", 2),
    ("circle_rectangle_intersect", 1),
    ("circle_rectangle_xor", 3),
];

const CPP_CIRCLE_RECT_SOURCE_OPS: [u32; 4] = [0_u32, 2_u32, 1_u32, 3_u32];

const CPP_PLINE_CORE_SOURCE_CASES: [&str; 6] = [
    "cavc_pline_new",
    "cavc_pline_set_capacity",
    "cavc_pline_set_vertex_data",
    "cavc_pline_add_vertex",
    "cavc_pline_remove_range",
    "cavc_pline_clear",
];

const CPP_AABBINDEX_EXTENTS_SOURCE_CASES: [&str; 2] = [
    "StaticSpatialIndexTests.index",
    "StaticSpatialIndexTests.skip_sorting_small_index",
];

const CPP_COINCIDENT_SOURCE_MATRIX: [(&str, u32); 10] = [
    ("coincident_case1_union", 0),
    ("coincident_case1_excludeAFromB", 2),
    ("coincident_case1_excludeBFromA", 2),
    ("coincident_case1_intersect", 1),
    ("coincident_case1_xor", 3),
    ("coincident_case2_union", 0),
    ("coincident_case2_excludeAFromB", 2),
    ("coincident_case2_excludeBFromA", 2),
    ("coincident_case2_intersect", 1),
    ("coincident_case2_xor", 3),
];

fn assert_boolean_case_source_mapping(
    actual: &[(&str, u32)],
    expected: &[(&str, u32)],
    context: &str,
) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "{context} case count drifted: actual={}, expected={}",
        actual.len(),
        expected.len()
    );

    for (expected_name, expected_operation) in expected {
        let case = actual
            .iter()
            .find(|(name, _)| *name == *expected_name)
            .unwrap_or_else(|| panic!("{context} missing source-backed case: {expected_name}"));
        assert_eq!(
            case.1, *expected_operation,
            "{context} operation drift for case={expected_name}"
        );
    }
}

fn assert_source_case_coverage(actual: &[&str], expected: &[&str], context: &str) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "{context} source-case count drifted: actual={}, expected={}",
        actual.len(),
        expected.len()
    );

    for (index, name) in actual.iter().enumerate() {
        assert!(
            actual[..index].iter().all(|prior| prior != name),
            "{context} duplicate source case coverage entry: {name}"
        );
    }

    for expected_name in expected {
        assert!(
            actual.iter().any(|name| name == expected_name),
            "{context} missing source-backed case: {expected_name}"
        );
    }
}

fn read_aabbindex_extents(aabbindex: *const cavc_aabbindex) -> (f64, f64, f64, f64) {
    let mut min_x = f64::NAN;
    let mut min_y = f64::NAN;
    let mut max_x = f64::NAN;
    let mut max_y = f64::NAN;
    unsafe {
        assert_eq!(
            cavc_aabbindex_get_extents(aabbindex, &mut min_x, &mut min_y, &mut max_x, &mut max_y),
            0
        );
    }
    (min_x, min_y, max_x, max_y)
}

fn cpp_coincident_boolean_matrix_cases() -> Vec<CoincidentMatrixCase> {
    let (case1_a, case1_b) = cpp_coincident_case1_inputs();
    let (case2_a, case2_b) = cpp_coincident_case2_inputs();
    let cases = vec![
        CoincidentMatrixCase {
            name: "coincident_case1_union",
            operation: 0,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        CoincidentMatrixCase {
            name: "coincident_case1_excludeAFromB",
            operation: 2,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        CoincidentMatrixCase {
            name: "coincident_case1_excludeBFromA",
            operation: 2,
            subject: case1_b.clone(),
            clip: case1_a.clone(),
        },
        CoincidentMatrixCase {
            name: "coincident_case1_intersect",
            operation: 1,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        CoincidentMatrixCase {
            name: "coincident_case1_xor",
            operation: 3,
            subject: case1_a.clone(),
            clip: case1_b.clone(),
        },
        CoincidentMatrixCase {
            name: "coincident_case2_union",
            operation: 0,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        CoincidentMatrixCase {
            name: "coincident_case2_excludeAFromB",
            operation: 2,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        CoincidentMatrixCase {
            name: "coincident_case2_excludeBFromA",
            operation: 2,
            subject: case2_b.clone(),
            clip: case2_a.clone(),
        },
        CoincidentMatrixCase {
            name: "coincident_case2_intersect",
            operation: 1,
            subject: case2_a.clone(),
            clip: case2_b.clone(),
        },
        CoincidentMatrixCase {
            name: "coincident_case2_xor",
            operation: 3,
            subject: case2_a,
            clip: case2_b,
        },
    ];

    let actual: Vec<(&str, u32)> = cases
        .iter()
        .map(|case| (case.name, case.operation))
        .collect();
    assert_boolean_case_source_mapping(
        &actual,
        &CPP_COINCIDENT_SOURCE_MATRIX,
        "coincident matrix helper",
    );

    cases
}

const CPP_MATRIX_EPS: f64 = 1e-4;
const CPP_PROBE_DELTA: f64 = 0.01;
const CPP_CIRCLE_RADIUS: f64 = 5.0;
const CPP_CIRCLE_INSIDE_DIST_FACTOR: f64 = 0.33;
const CPP_CIRCLE_OUTSIDE_DIST_FACTOR: f64 = 1.5;
const CPP_CLOSEST_EPS_MATRIX: [f64; 4] = [1e-9, 1e-7, 1e-5, 1e-4];
const CPP_TOLERANCE_SCALE_MATRIX: [f64; 3] = [0.5_f64, 1.0_f64, 2.0_f64];
const CPP_SELF_INTERSECTS_INCLUDE_MODES: [u32; 3] = [
    CAVC_SELF_INTERSECTS_INCLUDE_ALL,
    CAVC_SELF_INTERSECTS_INCLUDE_LOCAL,
    CAVC_SELF_INTERSECTS_INCLUDE_GLOBAL,
];

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

#[derive(Copy, Clone)]
struct HalfClosestCase {
    query: (f64, f64),
    expected_point: (f64, f64),
    expected_distance: f64,
    expected_index: u32,
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
    cpp_circle_case_vertices_with_radius(case, CPP_CIRCLE_RADIUS)
}

fn cpp_circle_case_vertices_with_radius(case: CircleCaseKey, radius: f64) -> Vec<(f64, f64, f64)> {
    let mut p0 = match case.alignment {
        CircleAlignment::XAxis => (case.center_x - radius, case.center_y),
        CircleAlignment::YAxis => (case.center_x, case.center_y - radius),
        CircleAlignment::Diagonal => (
            case.center_x + radius * (std::f64::consts::PI / 4.0).cos(),
            case.center_y + radius * (std::f64::consts::PI / 4.0).sin(),
        ),
    };
    let mut p1 = match case.alignment {
        CircleAlignment::XAxis => (case.center_x + radius, case.center_y),
        CircleAlignment::YAxis => (case.center_x, case.center_y + radius),
        CircleAlignment::Diagonal => (
            case.center_x + radius * (5.0 * std::f64::consts::PI / 4.0).cos(),
            case.center_y + radius * (5.0 * std::f64::consts::PI / 4.0).sin(),
        ),
    };

    if case.reverse {
        std::mem::swap(&mut p0, &mut p1);
    }

    let bulge = if case.direction > 0 { 1.0 } else { -1.0 };
    vec![(p0.0, p0.1, bulge), (p1.0, p1.1, bulge)]
}

fn tuple_vertices_to_cavc(vertices: &[(f64, f64, f64)]) -> Vec<cavc_vertex> {
    vertices
        .iter()
        .map(|(x, y, bulge)| cavc_vertex::new(*x, *y, *bulge))
        .collect()
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

fn scale_vertex_from_center(
    vertex: cavc_vertex,
    center_x: f64,
    center_y: f64,
    magnitude: f64,
) -> cavc_vertex {
    let dir_x = vertex.x - center_x;
    let dir_y = vertex.y - center_y;
    let dir_len = (dir_x * dir_x + dir_y * dir_y).sqrt();
    cavc_vertex::new(
        magnitude * dir_x / dir_len + center_x,
        magnitude * dir_y / dir_len + center_y,
        vertex.bulge,
    )
}

fn intersects_at_y(center_x: f64, center_y: f64, radius: f64, y: f64) -> ((f64, f64), (f64, f64)) {
    let y_term = y - center_y;
    let root = (radius * radius - y_term * y_term).sqrt();
    ((center_x + root, y), (center_x - root, y))
}

fn intersects_at_x(center_x: f64, center_y: f64, radius: f64, x: f64) -> ((f64, f64), (f64, f64)) {
    let x_term = x - center_x;
    let root = (radius * radius - x_term * x_term).sqrt();
    ((x, center_y + root), (x, center_y - root))
}

fn abs_bulge_between_points(center_x: f64, center_y: f64, p1: (f64, f64), p2: (f64, f64)) -> f64 {
    let a1 = (p1.1 - center_y).atan2(p1.0 - center_x);
    let a2 = (p2.1 - center_y).atan2(p2.0 - center_x);
    let mut a_diff = a1 - a2;
    a_diff = (a_diff + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI)
        - std::f64::consts::PI;
    (a_diff / 4.0).tan().abs()
}

fn build_half_circle_offset_expectations(
    case: HalfCircleCaseKey,
) -> (f64, Vec<cavc_vertex>, f64, Vec<cavc_vertex>) {
    let input_vertices = tuple_vertices_to_cavc(&cpp_half_circle_case_vertices(case));
    let (min_x, min_y, max_x, max_y) = cpp_expected_half_circle_extents(case);

    let outward_delta = -(case.direction as f64) * 0.25 * CPP_CIRCLE_RADIUS;
    let inward_delta = (case.direction as f64) * 0.4 * CPP_CIRCLE_RADIUS;

    let abs_outward_delta = outward_delta.abs();
    let abs_inward_delta = inward_delta.abs();
    let outward_magnitude = CPP_CIRCLE_RADIUS + abs_outward_delta;
    let inward_magnitude = CPP_CIRCLE_RADIUS - abs_inward_delta;

    let mut outward_vertices: Vec<_> = input_vertices
        .iter()
        .map(|v| scale_vertex_from_center(*v, case.center_x, case.center_y, outward_magnitude))
        .collect();
    let mut inward_vertices: Vec<_> = input_vertices
        .iter()
        .map(|v| scale_vertex_from_center(*v, case.center_x, case.center_y, inward_magnitude))
        .collect();

    if case.is_closed {
        let right_angle_bulge = (std::f64::consts::PI / 8.0).tan();
        if case.is_x_aligned {
            if case.direction > 0 {
                if let Some(last) = outward_vertices.last_mut() {
                    last.bulge = right_angle_bulge;
                }
                outward_vertices.push(cavc_vertex::new(
                    max_x,
                    case.center_y + abs_outward_delta,
                    0.0,
                ));
                outward_vertices.push(cavc_vertex::new(
                    min_x,
                    case.center_y + abs_outward_delta,
                    right_angle_bulge,
                ));

                let y_intr = case.center_y - abs_inward_delta;
                let (intr1, intr2) =
                    intersects_at_y(case.center_x, case.center_y, inward_magnitude, y_intr);
                let abs_bulge =
                    abs_bulge_between_points(case.center_x, case.center_y, intr1, intr2);
                inward_vertices[0] = cavc_vertex::new(intr1.0, intr1.1, 0.0);
                inward_vertices[1] = cavc_vertex::new(intr2.0, intr2.1, abs_bulge);
            } else {
                if let Some(last) = outward_vertices.last_mut() {
                    last.bulge = -right_angle_bulge;
                }
                outward_vertices.push(cavc_vertex::new(
                    max_x,
                    case.center_y - abs_outward_delta,
                    0.0,
                ));
                outward_vertices.push(cavc_vertex::new(
                    min_x,
                    case.center_y - abs_outward_delta,
                    -right_angle_bulge,
                ));

                let y_intr = case.center_y + abs_inward_delta;
                let (intr1, intr2) =
                    intersects_at_y(case.center_x, case.center_y, inward_magnitude, y_intr);
                let abs_bulge =
                    abs_bulge_between_points(case.center_x, case.center_y, intr1, intr2);
                inward_vertices[0] = cavc_vertex::new(intr1.0, intr1.1, 0.0);
                inward_vertices[1] = cavc_vertex::new(intr2.0, intr2.1, -abs_bulge);
            }
        } else if case.direction > 0 {
            if let Some(last) = outward_vertices.last_mut() {
                last.bulge = right_angle_bulge;
            }
            outward_vertices.push(cavc_vertex::new(
                case.center_x - abs_outward_delta,
                max_y,
                0.0,
            ));
            outward_vertices.push(cavc_vertex::new(
                case.center_x - abs_outward_delta,
                min_y,
                right_angle_bulge,
            ));

            let x_intr = case.center_x + abs_inward_delta;
            let (intr1, intr2) =
                intersects_at_x(case.center_x, case.center_y, inward_magnitude, x_intr);
            let abs_bulge = abs_bulge_between_points(case.center_x, case.center_y, intr1, intr2);
            inward_vertices[0] = cavc_vertex::new(intr1.0, intr1.1, 0.0);
            inward_vertices[1] = cavc_vertex::new(intr2.0, intr2.1, abs_bulge);
        } else {
            if let Some(last) = outward_vertices.last_mut() {
                last.bulge = -right_angle_bulge;
            }
            outward_vertices.push(cavc_vertex::new(
                case.center_x + abs_outward_delta,
                max_y,
                0.0,
            ));
            outward_vertices.push(cavc_vertex::new(
                case.center_x + abs_outward_delta,
                min_y,
                -right_angle_bulge,
            ));

            let x_intr = case.center_x - abs_inward_delta;
            let (intr1, intr2) =
                intersects_at_x(case.center_x, case.center_y, inward_magnitude, x_intr);
            let abs_bulge = abs_bulge_between_points(case.center_x, case.center_y, intr1, intr2);
            inward_vertices[0] = cavc_vertex::new(intr1.0, intr1.1, 0.0);
            inward_vertices[1] = cavc_vertex::new(intr2.0, intr2.1, -abs_bulge);
        }
    }

    (
        outward_delta,
        outward_vertices,
        inward_delta,
        inward_vertices,
    )
}

fn half_circle_collapse_deltas(case: HalfCircleCaseKey) -> [f64; 3] {
    let direction = case.direction as f64;
    let first = if case.is_closed {
        direction * 0.5 * CPP_CIRCLE_RADIUS
    } else {
        direction * CPP_CIRCLE_RADIUS
    };
    [
        first,
        direction * 1.5 * CPP_CIRCLE_RADIUS,
        direction * 2.0 * CPP_CIRCLE_RADIUS,
    ]
}

fn build_half_circle_closest_cases(case: HalfCircleCaseKey) -> Vec<HalfClosestCase> {
    let (min_x, min_y, max_x, max_y) = cpp_expected_half_circle_extents(case);
    let cx = case.center_x;
    let cy = case.center_y;
    let end_point_index = if case.is_closed { 1_u32 } else { 0_u32 };
    let mut result = Vec::new();

    if case.is_x_aligned {
        result.push(HalfClosestCase {
            query: (min_x, cy),
            expected_point: (min_x, cy),
            expected_distance: 0.0,
            expected_index: 0,
        });
        result.push(HalfClosestCase {
            query: (max_x, cy),
            expected_point: (max_x, cy),
            expected_distance: 0.0,
            expected_index: end_point_index,
        });
    } else {
        result.push(HalfClosestCase {
            query: (cx, min_y),
            expected_point: (cx, min_y),
            expected_distance: 0.0,
            expected_index: 0,
        });
        result.push(HalfClosestCase {
            query: (cx, max_y),
            expected_point: (cx, max_y),
            expected_distance: 0.0,
            expected_index: end_point_index,
        });
    }

    if case.is_x_aligned {
        let arc_midpoint_y = if case.direction > 0 { min_y } else { max_y };
        result.push(HalfClosestCase {
            query: (min_x - CPP_PROBE_DELTA, cy),
            expected_point: (min_x, cy),
            expected_distance: CPP_PROBE_DELTA,
            expected_index: 0,
        });
        result.push(HalfClosestCase {
            query: (max_x + CPP_PROBE_DELTA, cy),
            expected_point: (max_x, cy),
            expected_distance: CPP_PROBE_DELTA,
            expected_index: end_point_index,
        });
        result.push(HalfClosestCase {
            query: (cx, arc_midpoint_y - CPP_PROBE_DELTA),
            expected_point: (cx, arc_midpoint_y),
            expected_distance: CPP_PROBE_DELTA,
            expected_index: 0,
        });
        result.push(HalfClosestCase {
            query: (cx, arc_midpoint_y + CPP_PROBE_DELTA),
            expected_point: (cx, arc_midpoint_y),
            expected_distance: CPP_PROBE_DELTA,
            expected_index: 0,
        });
        if case.is_closed {
            result.push(HalfClosestCase {
                query: (cx, cy - CPP_PROBE_DELTA),
                expected_point: (cx, cy),
                expected_distance: CPP_PROBE_DELTA,
                expected_index: 1,
            });
            result.push(HalfClosestCase {
                query: (cx, cy + CPP_PROBE_DELTA),
                expected_point: (cx, cy),
                expected_distance: CPP_PROBE_DELTA,
                expected_index: 1,
            });
        }
    } else {
        let arc_midpoint_x = if case.direction > 0 { max_x } else { min_x };
        result.push(HalfClosestCase {
            query: (cx, min_y - CPP_PROBE_DELTA),
            expected_point: (cx, min_y),
            expected_distance: CPP_PROBE_DELTA,
            expected_index: 0,
        });
        result.push(HalfClosestCase {
            query: (cx, max_y + CPP_PROBE_DELTA),
            expected_point: (cx, max_y),
            expected_distance: CPP_PROBE_DELTA,
            expected_index: end_point_index,
        });
        result.push(HalfClosestCase {
            query: (arc_midpoint_x - CPP_PROBE_DELTA, cy),
            expected_point: (arc_midpoint_x, cy),
            expected_distance: CPP_PROBE_DELTA,
            expected_index: 0,
        });
        result.push(HalfClosestCase {
            query: (arc_midpoint_x + CPP_PROBE_DELTA, cy),
            expected_point: (arc_midpoint_x, cy),
            expected_distance: CPP_PROBE_DELTA,
            expected_index: 0,
        });
        if case.is_closed {
            result.push(HalfClosestCase {
                query: (cx - CPP_PROBE_DELTA, cy),
                expected_point: (cx, cy),
                expected_distance: CPP_PROBE_DELTA,
                expected_index: 1,
            });
            result.push(HalfClosestCase {
                query: (cx + CPP_PROBE_DELTA, cy),
                expected_point: (cx, cy),
                expected_distance: CPP_PROBE_DELTA,
                expected_index: 1,
            });
        }
    }

    result
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
fn pline_function_surface_half_circle_closest_point_strict_index_cpp_matrix_parity() {
    for case in cpp_half_circle_matrix_cases() {
        let pline = create_pline(&cpp_half_circle_case_vertices(case), case.is_closed);
        let closest_cases = build_half_circle_closest_cases(case);
        for (case_idx, expected) in closest_cases.iter().enumerate() {
            let (seg_index, p, d) =
                eval_closest_point_result(pline, expected.query.0, expected.query.1, 1e-5);
            assert_eq!(
                seg_index, expected.expected_index,
                "half-circle closest index mismatch at case #{case_idx} query={:?}",
                expected.query
            );
            assert_near_ctx(
                p.x,
                expected.expected_point.0,
                "half-circle closest point x",
            );
            assert_near_ctx(
                p.y,
                expected.expected_point.1,
                "half-circle closest point y",
            );
            assert_near_ctx(
                d,
                expected.expected_distance,
                "half-circle closest distance",
            );
        }
        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_function_surface_circle_closest_point_eps_tie_break_cpp_parity() {
    for case in cpp_circle_matrix_cases() {
        let pline = create_pline(&cpp_circle_case_vertices(case), true);
        let vertices = read_vertices(pline);

        for pos_equal_eps in CPP_CLOSEST_EPS_MATRIX {
            let (i0, p0, d0) =
                eval_closest_point_result(pline, vertices[0].x, vertices[0].y, pos_equal_eps);
            assert_eq!(i0, 0);
            assert_near_ctx(p0.x, vertices[0].x, "circle eps tie-break vertex0 x");
            assert_near_ctx(p0.y, vertices[0].y, "circle eps tie-break vertex0 y");
            assert_near_ctx(d0, 0.0, "circle eps tie-break vertex0 distance");

            let (i1, p1, d1) =
                eval_closest_point_result(pline, vertices[1].x, vertices[1].y, pos_equal_eps);
            assert_eq!(i1, 1);
            assert_near_ctx(p1.x, vertices[1].x, "circle eps tie-break vertex1 x");
            assert_near_ctx(p1.y, vertices[1].y, "circle eps tie-break vertex1 y");
            assert_near_ctx(d1, 0.0, "circle eps tie-break vertex1 distance");
        }

        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_function_surface_half_circle_closest_point_eps_tie_break_cpp_parity() {
    for case in cpp_half_circle_matrix_cases() {
        let pline = create_pline(&cpp_half_circle_case_vertices(case), case.is_closed);
        let closest_cases = build_half_circle_closest_cases(case);

        for (case_idx, expected) in closest_cases.iter().enumerate() {
            for pos_equal_eps in CPP_CLOSEST_EPS_MATRIX {
                let (seg_index, p, d) = eval_closest_point_result(
                    pline,
                    expected.query.0,
                    expected.query.1,
                    pos_equal_eps,
                );
                assert_eq!(
                    seg_index, expected.expected_index,
                    "half-circle eps tie-break index mismatch at case #{case_idx} query={:?} eps={pos_equal_eps}",
                    expected.query
                );
                assert_near_ctx(
                    p.x,
                    expected.expected_point.0,
                    "half-circle eps tie-break point x",
                );
                assert_near_ctx(
                    p.y,
                    expected.expected_point.1,
                    "half-circle eps tie-break point y",
                );
                assert_near_ctx(
                    d,
                    expected.expected_distance,
                    "half-circle eps tie-break distance",
                );
            }
        }

        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_function_surface_circle_parallel_offset_cpp_matrix_parity() {
    for (case_idx, case) in cpp_circle_matrix_cases().into_iter().enumerate() {
        let pline = create_pline(&cpp_circle_case_vertices(case), true);

        let outward_delta = -(case.direction as f64) * 0.25 * CPP_CIRCLE_RADIUS;
        let inward_delta = (case.direction as f64) * 0.5 * CPP_CIRCLE_RADIUS;

        let outward_expected = tuple_vertices_to_cavc(&cpp_circle_case_vertices_with_radius(
            case,
            CPP_CIRCLE_RADIUS + outward_delta.abs(),
        ));
        let inward_expected = tuple_vertices_to_cavc(&cpp_circle_case_vertices_with_radius(
            case,
            CPP_CIRCLE_RADIUS - inward_delta.abs(),
        ));

        let outward_actual = run_parallel_offset_vertexes(pline, outward_delta);
        assert_single_offset_vertex_match(
            &outward_actual,
            &outward_expected,
            true,
            &format!("circle offset outward case #{case_idx}"),
        );

        let inward_actual = run_parallel_offset_vertexes(pline, inward_delta);
        assert_single_offset_vertex_match(
            &inward_actual,
            &inward_expected,
            true,
            &format!("circle offset inward case #{case_idx}"),
        );

        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_function_surface_circle_collapsed_offset_cpp_matrix_parity() {
    for (case_idx, case) in cpp_circle_matrix_cases().into_iter().enumerate() {
        let pline = create_pline(&cpp_circle_case_vertices(case), true);
        let collapse_deltas = [
            (case.direction as f64) * CPP_CIRCLE_RADIUS,
            (case.direction as f64) * 1.5 * CPP_CIRCLE_RADIUS,
            (case.direction as f64) * 2.0 * CPP_CIRCLE_RADIUS,
        ];
        for (delta_idx, delta) in collapse_deltas.iter().enumerate() {
            let result = run_parallel_offset_vertexes(pline, *delta);
            assert!(
                result.is_empty(),
                "circle collapsed offset expected empty at case #{case_idx} delta#{delta_idx} (delta={delta}), got {} result(s)",
                result.len()
            );
        }

        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_function_surface_half_circle_parallel_offset_cpp_matrix_parity() {
    for (case_idx, case) in cpp_half_circle_matrix_cases().into_iter().enumerate() {
        let pline = create_pline(&cpp_half_circle_case_vertices(case), case.is_closed);
        let (outward_delta, outward_expected, inward_delta, inward_expected) =
            build_half_circle_offset_expectations(case);

        let outward_actual = run_parallel_offset_vertexes(pline, outward_delta);
        assert_single_offset_vertex_match(
            &outward_actual,
            &outward_expected,
            case.is_closed,
            &format!("half-circle offset outward case #{case_idx}"),
        );

        let inward_actual = run_parallel_offset_vertexes(pline, inward_delta);
        assert_single_offset_vertex_match(
            &inward_actual,
            &inward_expected,
            case.is_closed,
            &format!("half-circle offset inward case #{case_idx}"),
        );

        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_function_surface_half_circle_collapsed_offset_cpp_matrix_parity() {
    for (case_idx, case) in cpp_half_circle_matrix_cases().into_iter().enumerate() {
        let pline = create_pline(&cpp_half_circle_case_vertices(case), case.is_closed);
        let collapse_deltas = half_circle_collapse_deltas(case);
        for (delta_idx, delta) in collapse_deltas.iter().enumerate() {
            let result = run_parallel_offset_vertexes(pline, *delta);
            assert!(
                result.is_empty(),
                "half-circle collapsed offset expected empty at case #{case_idx} delta#{delta_idx} (delta={delta}), got {} result(s)",
                result.len()
            );
        }

        unsafe {
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_function_surface_closed_matrix_combine_with_self_cpp_parity() {
    let mut closed_cases = Vec::new();
    for case in cpp_circle_matrix_cases() {
        closed_cases.push(cpp_circle_case_vertices(case));
    }
    for case in cpp_half_circle_matrix_cases() {
        if case.is_closed {
            closed_cases.push(cpp_half_circle_case_vertices(case));
        }
    }

    for (case_idx, case_vertices) in closed_cases.iter().enumerate() {
        let pline = create_pline(case_vertices, true);
        let expected = read_vertices(pline);

        let (remaining, subtracted) = run_boolean_vertexes(pline, pline, 0);
        assert_eq!(
            remaining.len(),
            1,
            "union with self should keep one pline at case #{case_idx}"
        );
        assert!(
            subtracted.is_empty(),
            "union with self should not produce subtracted plines at case #{case_idx}"
        );
        compare_vertexes(&remaining[0], &expected);

        let (remaining, subtracted) = run_boolean_vertexes(pline, pline, 2);
        assert!(
            remaining.is_empty(),
            "exclude with self should be empty at case #{case_idx}"
        );
        assert!(
            subtracted.is_empty(),
            "exclude with self should not produce subtracted plines at case #{case_idx}"
        );

        let (remaining, subtracted) = run_boolean_vertexes(pline, pline, 1);
        assert_eq!(
            remaining.len(),
            1,
            "intersect with self should keep one pline at case #{case_idx}"
        );
        assert!(
            subtracted.is_empty(),
            "intersect with self should not produce subtracted plines at case #{case_idx}"
        );
        compare_vertexes(&remaining[0], &expected);

        let (remaining, subtracted) = run_boolean_vertexes(pline, pline, 3);
        assert!(
            remaining.is_empty(),
            "xor with self should be empty at case #{case_idx}"
        );
        assert!(
            subtracted.is_empty(),
            "xor with self should not produce subtracted plines at case #{case_idx}"
        );

        let after = read_vertices(pline);
        compare_vertexes(&after, &expected);

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
fn pline_mutator_invalid_input_contracts_ffi() {
    let pline = create_pline(&[(1.0, 2.0, 0.0), (3.0, 4.0, 1.0)], false);

    unsafe {
        let vertices = [cavc_vertex::new(8.0, 9.0, 0.25)];
        assert_eq!(
            cavc_pline_set_vertex_data(ptr::null_mut(), vertices.as_ptr(), 1),
            1
        );
        assert_eq!(cavc_pline_set_is_closed(ptr::null_mut(), 1), 1);
        assert_eq!(cavc_pline_clear(ptr::null_mut()), 1);
        assert_eq!(
            cavc_pline_set_vertex(ptr::null_mut(), 0, cavc_vertex::new(0.0, 0.0, 0.0)),
            1
        );
        assert_eq!(
            cavc_pline_set_vertex(pline, 99, cavc_vertex::new(0.0, 0.0, 0.0)),
            2
        );
        assert_eq!(cavc_pline_remove(ptr::null_mut(), 0), 1);
        assert_eq!(cavc_pline_remove(pline, 99), 2);
        cavc_pline_f(pline);
    }
}

#[test]
fn pline_core_suite_cpp_parity() {
    // old C++ source: TEST_cavc_pline.cpp -> cavc_pline_new, cavc_pline_set_capacity,
    // cavc_pline_set_vertex_data, cavc_pline_add_vertex, cavc_pline_remove_range,
    // cavc_pline_clear
    let seed = vec![
        (1.0, 2.0, 0.1),
        (33.0, 3.0, 0.2),
        (34.0, 35.0, 0.3),
        (2.0, 36.0, 0.4),
    ];
    let expected_seed = vec![
        cavc_vertex::new(1.0, 2.0, 0.1),
        cavc_vertex::new(33.0, 3.0, 0.2),
        cavc_vertex::new(34.0, 35.0, 0.3),
        cavc_vertex::new(2.0, 36.0, 0.4),
    ];
    let pline1 = create_pline(&seed, false);
    let pline2 = create_pline(&[], true);
    let mut covered_source_cases = Vec::new();

    unsafe {
        // cavc_pline_new
        let mut pline1_is_closed: u8 = 1;
        let mut pline2_is_closed: u8 = 0;
        assert_eq!(cavc_pline_get_is_closed(pline1, &mut pline1_is_closed), 0);
        assert_eq!(cavc_pline_get_is_closed(pline2, &mut pline2_is_closed), 0);
        assert_eq!(pline1_is_closed, 0);
        assert_ne!(pline2_is_closed, 0);

        let mut pline1_count = u32::MAX;
        let mut pline2_count = u32::MAX;
        assert_eq!(cavc_pline_get_vertex_count(pline1, &mut pline1_count), 0);
        assert_eq!(cavc_pline_get_vertex_count(pline2, &mut pline2_count), 0);
        assert_eq!(pline1_count, 4);
        assert_eq!(pline2_count, 0);
        compare_vertexes(&read_vertices(pline1), &expected_seed);

        let mut read_out = expected_seed.clone();
        let read_before = read_out.clone();
        assert_eq!(cavc_pline_get_vertex_data(pline2, read_out.as_mut_ptr()), 0);
        compare_vertexes(&read_out, &read_before);
        covered_source_cases.push("cavc_pline_new");

        // cavc_pline_set_capacity (reserve equivalence: shrink no-op, then grow)
        assert_eq!(cavc_pline_reserve(pline1, 1), 0);
        assert_eq!(cavc_pline_reserve(pline1, 11), 0);
        compare_vertexes(&read_vertices(pline1), &expected_seed);
        covered_source_cases.push("cavc_pline_set_capacity");

        // cavc_pline_set_vertex_data
        assert_eq!(
            cavc_pline_set_vertex_data(pline2, expected_seed.as_ptr(), expected_seed.len() as u32),
            0
        );
        compare_vertexes(&read_vertices(pline2), &expected_seed);
        covered_source_cases.push("cavc_pline_set_vertex_data");

        // cavc_pline_add_vertex
        assert_eq!(cavc_pline_add(pline1, 555.0, 666.0, 0.777), 0);
        assert_eq!(cavc_pline_add(pline2, 555.0, 666.0, 0.777), 0);
        let mut expected_with_added = expected_seed.clone();
        expected_with_added.push(cavc_vertex::new(555.0, 666.0, 0.777));
        compare_vertexes(&read_vertices(pline1), &expected_with_added);
        compare_vertexes(&read_vertices(pline2), &expected_with_added);
        covered_source_cases.push("cavc_pline_add_vertex");
    }

    // cavc_pline_remove_range (remove-sequence equivalence in current C-API)
    let remove_pline = create_pline(&seed, false);
    unsafe {
        assert_eq!(cavc_pline_remove(remove_pline, 0), 0);
    }
    compare_vertexes(
        &read_vertices(remove_pline),
        &[
            cavc_vertex::new(33.0, 3.0, 0.2),
            cavc_vertex::new(34.0, 35.0, 0.3),
            cavc_vertex::new(2.0, 36.0, 0.4),
        ],
    );

    unsafe {
        assert_eq!(cavc_pline_remove(remove_pline, 1), 0);
        assert_eq!(cavc_pline_remove(remove_pline, 1), 0);
    }
    compare_vertexes(
        &read_vertices(remove_pline),
        &[cavc_vertex::new(33.0, 3.0, 0.2)],
    );

    unsafe {
        assert_eq!(cavc_pline_remove(remove_pline, 0), 0);
        cavc_pline_f(remove_pline);
    }
    covered_source_cases.push("cavc_pline_remove_range");

    // cavc_pline_clear
    unsafe {
        assert_eq!(cavc_pline_clear(pline1), 0);
        assert_eq!(cavc_pline_clear(pline2), 0);
        let mut pline1_count = u32::MAX;
        let mut pline2_count = u32::MAX;
        assert_eq!(cavc_pline_get_vertex_count(pline1, &mut pline1_count), 0);
        assert_eq!(cavc_pline_get_vertex_count(pline2, &mut pline2_count), 0);
        assert_eq!(pline1_count, 0);
        assert_eq!(pline2_count, 0);
        cavc_pline_f(pline1);
        cavc_pline_f(pline2);
    }
    covered_source_cases.push("cavc_pline_clear");

    assert_source_case_coverage(
        &covered_source_cases,
        &CPP_PLINE_CORE_SOURCE_CASES,
        "pline core suite cpp parity",
    );
}

#[test]
fn pline_get_vertex_data_empty_does_not_modify_buffer_cpp_parity() {
    let pline = create_pline(&[], true);
    let mut out = [
        cavc_vertex::new(-1.0, -2.0, -3.0),
        cavc_vertex::new(-4.0, -5.0, -6.0),
    ];
    let before = out;
    unsafe {
        assert_eq!(cavc_pline_get_vertex_data(pline, out.as_mut_ptr()), 0);
        cavc_pline_f(pline);
    }
    compare_vertexes(&out, &before);
}

#[test]
fn pline_reserve_does_not_modify_existing_vertex_data_cpp_parity() {
    let source_vertices = vec![
        (1.0, 2.0, 0.1),
        (33.0, 3.0, 0.2),
        (34.0, 35.0, 0.3),
        (2.0, 36.0, 0.4),
    ];
    let pline = create_pline(&source_vertices, false);
    let before = read_vertices(pline);

    unsafe {
        assert_eq!(cavc_pline_reserve(pline, 1), 0);
        assert_eq!(cavc_pline_reserve(pline, 11), 0);
    }

    let after = read_vertices(pline);
    compare_vertexes(&after, &before);

    unsafe {
        cavc_pline_f(pline);
    }
}

#[test]
fn pline_reserve_equivalence_preserves_prefix_across_growth_and_append_cpp_parity() {
    let source_vertices = vec![
        (1.0, 2.0, 0.1),
        (33.0, 3.0, 0.2),
        (34.0, 35.0, 0.3),
        (2.0, 36.0, 0.4),
    ];
    let pline = create_pline(&source_vertices, false);
    let before = read_vertices(pline);

    unsafe {
        // old set_capacity equivalence zone: shrink no-op, then grow
        assert_eq!(cavc_pline_reserve(pline, 1), 0);
        assert_eq!(cavc_pline_reserve(pline, 11), 0);
        assert_eq!(cavc_pline_add(pline, 555.0, 666.0, 0.777), 0);
        assert_eq!(cavc_pline_add(pline, -9.0, -8.0, -0.25), 0);
    }

    let mut expected = before;
    expected.push(cavc_vertex::new(555.0, 666.0, 0.777));
    expected.push(cavc_vertex::new(-9.0, -8.0, -0.25));
    compare_vertexes(&read_vertices(pline), &expected);

    unsafe {
        cavc_pline_f(pline);
    }
}

#[test]
fn pline_remove_sequence_equivalent_to_cpp_remove_range_parity() {
    let source = vec![
        (1.0, 2.0, 0.1),
        (33.0, 3.0, 0.2),
        (34.0, 35.0, 0.3),
        (2.0, 36.0, 0.4),
    ];
    let pline = create_pline(&source, false);

    // Equivalent to old remove_range(0, 1): remove first vertex.
    unsafe {
        assert_eq!(cavc_pline_remove(pline, 0), 0);
    }
    let expected_after_first = vec![
        cavc_vertex::new(33.0, 3.0, 0.2),
        cavc_vertex::new(34.0, 35.0, 0.3),
        cavc_vertex::new(2.0, 36.0, 0.4),
    ];
    compare_vertexes(&read_vertices(pline), &expected_after_first);

    // Equivalent to old remove_range(1, 2): remove 2nd and 3rd from current list.
    unsafe {
        assert_eq!(cavc_pline_remove(pline, 1), 0);
        assert_eq!(cavc_pline_remove(pline, 1), 0);
    }
    let expected_after_second = vec![cavc_vertex::new(33.0, 3.0, 0.2)];
    compare_vertexes(&read_vertices(pline), &expected_after_second);

    // Equivalent to old final remove_range(0, 1): remove last remaining vertex.
    unsafe {
        assert_eq!(cavc_pline_remove(pline, 0), 0);
    }
    let mut count = u32::MAX;
    let mut out = [cavc_vertex::new(-1.0, -2.0, -3.0); 10];
    let out_before = out;
    unsafe {
        assert_eq!(cavc_pline_get_vertex_count(pline, &mut count), 0);
        assert_eq!(cavc_pline_get_vertex_data(pline, out.as_mut_ptr()), 0);
        cavc_pline_f(pline);
    }
    assert_eq!(count, 0);
    compare_vertexes(&out, &out_before);
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
fn pline_eval_failure_path_output_stability_ffi() {
    let pline = create_pline(&[(0.0, 0.0, 1.0), (2.0, 0.0, 1.0)], true);
    let empty_pline = create_pline(&[], true);

    unsafe {
        let mut path_length = -11.0_f64;
        assert_eq!(
            cavc_pline_eval_path_length(ptr::null(), &mut path_length),
            1
        );
        assert_fuzzy_eq!(path_length, -11.0);

        let mut area = -22.0_f64;
        assert_eq!(cavc_pline_eval_area(ptr::null(), &mut area), 1);
        assert_fuzzy_eq!(area, -22.0);

        let mut wn = 123_i32;
        assert_eq!(cavc_pline_eval_wn(ptr::null(), 0.0, 0.0, &mut wn), 1);
        assert_eq!(wn, 123);

        let (mut min_x, mut min_y, mut max_x, mut max_y) = (1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64);
        assert_eq!(
            cavc_pline_eval_extents(ptr::null(), &mut min_x, &mut min_y, &mut max_x, &mut max_y),
            1
        );
        assert_fuzzy_eq!(min_x, 1.0);
        assert_fuzzy_eq!(min_y, 2.0);
        assert_fuzzy_eq!(max_x, 3.0);
        assert_fuzzy_eq!(max_y, 4.0);

        let mut seg_start_index = 77_u32;
        let mut closest_point = cavc_point::new(8.0, 9.0);
        let mut distance = 10.0_f64;
        assert_eq!(
            cavc_pline_eval_closest_point(
                ptr::null(),
                0.0,
                0.0,
                1e-5,
                &mut seg_start_index,
                &mut closest_point,
                &mut distance
            ),
            1
        );
        assert_eq!(seg_start_index, 77);
        assert_fuzzy_eq!(closest_point.x, 8.0);
        assert_fuzzy_eq!(closest_point.y, 9.0);
        assert_fuzzy_eq!(distance, 10.0);

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
        assert_eq!(seg_start_index, 77);
        assert_fuzzy_eq!(closest_point.x, 8.0);
        assert_fuzzy_eq!(closest_point.y, 9.0);
        assert_fuzzy_eq!(distance, 10.0);

        cavc_pline_f(pline);
        cavc_pline_f(empty_pline);
    }
}

#[test]
fn pline_core_output_stability_ffi() {
    let pline = create_pline(
        &[(10.0, 20.0, 0.0), (30.0, 40.0, 1.0), (50.0, 60.0, 0.0)],
        true,
    );

    unsafe {
        assert_eq!(
            cavc_pline_set_userdata_values(pline, [7_u64, 8_u64].as_ptr(), 2),
            0
        );

        let pline_sentinel =
            std::ptr::NonNull::<cavc_pline>::dangling().as_ptr() as *const cavc_pline;
        let mut cloned = pline_sentinel;
        assert_eq!(cavc_pline_clone(ptr::null(), &mut cloned), 1);
        assert_eq!(cloned, pline_sentinel);

        let mut is_closed = 9_u8;
        assert_eq!(cavc_pline_get_is_closed(ptr::null(), &mut is_closed), 1);
        assert_eq!(is_closed, 9);

        let mut vertex_count = 314_u32;
        assert_eq!(
            cavc_pline_get_vertex_count(ptr::null(), &mut vertex_count),
            1
        );
        assert_eq!(vertex_count, 314);

        let mut vertex_data = [cavc_vertex::new(-1.0, -2.0, -3.0); 3];
        assert_eq!(
            cavc_pline_get_vertex_data(ptr::null(), vertex_data.as_mut_ptr()),
            1
        );
        assert_fuzzy_eq!(vertex_data[0].x, -1.0);
        assert_fuzzy_eq!(vertex_data[0].y, -2.0);
        assert_fuzzy_eq!(vertex_data[0].bulge, -3.0);

        let mut vertex = cavc_vertex::new(1.25, 2.5, 3.75);
        assert_eq!(cavc_pline_get_vertex(ptr::null(), 0, &mut vertex), 1);
        assert_fuzzy_eq!(vertex.x, 1.25);
        assert_fuzzy_eq!(vertex.y, 2.5);
        assert_fuzzy_eq!(vertex.bulge, 3.75);

        assert_eq!(cavc_pline_get_vertex(pline, 99, &mut vertex), 2);
        assert_fuzzy_eq!(vertex.x, 1.25);
        assert_fuzzy_eq!(vertex.y, 2.5);
        assert_fuzzy_eq!(vertex.bulge, 3.75);

        let mut userdata_count = 271_u32;
        assert_eq!(
            cavc_pline_get_userdata_count(ptr::null(), &mut userdata_count),
            1
        );
        assert_eq!(userdata_count, 271);

        let mut userdata_value = 0xABCD_EF01_u64;
        assert_eq!(
            cavc_pline_get_userdata_values(ptr::null(), &mut userdata_value),
            1
        );
        assert_eq!(userdata_value, 0xABCD_EF01);

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
fn pline_eval_extents_degenerate_error_ffi() {
    let pline = create_pline(&[(0.0, 0.0, 0.0)], false);
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (11.0, 22.0, 33.0, 44.0);

    unsafe {
        assert_eq!(
            cavc_pline_eval_extents(pline, &mut min_x, &mut min_y, &mut max_x, &mut max_y),
            2
        );
        cavc_pline_f(pline);
    }

    assert_fuzzy_eq!(min_x, 11.0);
    assert_fuzzy_eq!(min_y, 22.0);
    assert_fuzzy_eq!(max_x, 33.0);
    assert_fuzzy_eq!(max_y, 44.0);
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
        let mut options = init_parallel_offset_options();

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

    let actual: Vec<(&str, u32)> = cases
        .iter()
        .map(|case| (case.name, case.operation))
        .collect();
    assert_boolean_case_source_mapping(
        &actual,
        &CPP_CIRCLE_RECT_SOURCE_MATRIX,
        "circle_rectangle default matrix",
    );

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
fn pline_boolean_circle_rectangle_not_start_index_rotation_matrix_parity() {
    let expected_remaining_ab = vec![
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
    ];

    let subject_base = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);
    let subject_rotated = create_pline(&[(10.0, 1.0, 1.0), (0.0, 1.0, 1.0)], true);
    let subject_reversed = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);
    let subject_rotated_reversed = create_pline(&[(10.0, 1.0, 1.0), (0.0, 1.0, 1.0)], true);

    let clip_base = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );
    let clip_rotated = create_pline(
        &[
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
            (3.0, -10.0, 0.0),
        ],
        true,
    );
    let clip_reversed = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );
    let clip_rotated_reversed = create_pline(
        &[
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
            (3.0, -10.0, 0.0),
        ],
        true,
    );

    unsafe {
        assert_eq!(cavc_pline_invert_direction(subject_reversed), 0);
        assert_eq!(cavc_pline_invert_direction(subject_rotated_reversed), 0);
        assert_eq!(cavc_pline_invert_direction(clip_reversed), 0);
        assert_eq!(cavc_pline_invert_direction(clip_rotated_reversed), 0);
    }

    let subject_variants = [
        subject_base,
        subject_rotated,
        subject_reversed,
        subject_rotated_reversed,
    ];
    let clip_variants = [
        clip_base,
        clip_rotated,
        clip_reversed,
        clip_rotated_reversed,
    ];

    let (baseline_ba_remaining, baseline_ba_subtracted) =
        run_boolean_props(clip_base, subject_base, 2);
    assert!(
        baseline_ba_subtracted.is_empty(),
        "circle-rectangle BA baseline expected empty subtracted, got {baseline_ba_subtracted:?}"
    );

    for a in subject_variants {
        for b in clip_variants {
            let (ab_remaining, ab_subtracted) = run_boolean_props(a, b, 2);
            let (ba_remaining, ba_subtracted) = run_boolean_props(b, a, 2);

            assert!(
                props_set_match_ignore_area_sign(&ab_remaining, &expected_remaining_ab, 1e-4),
                "AB NOT mismatch for circle-rectangle rotation/reversal variant\nab={ab_remaining:?}\nexpected={expected_remaining_ab:?}"
            );
            assert!(
                ab_subtracted.is_empty(),
                "AB NOT expected empty subtracted for circle-rectangle rotation/reversal variant, got {ab_subtracted:?}"
            );

            assert!(
                props_set_match_ignore_area_sign(&ba_remaining, &baseline_ba_remaining, 1e-4),
                "BA NOT mismatch for circle-rectangle rotation/reversal variant\nba={ba_remaining:?}\nbaseline={baseline_ba_remaining:?}"
            );
            assert!(
                ba_subtracted.is_empty(),
                "BA NOT expected empty subtracted for circle-rectangle rotation/reversal variant, got {ba_subtracted:?}"
            );
        }
    }

    unsafe {
        cavc_pline_f(subject_base);
        cavc_pline_f(subject_rotated);
        cavc_pline_f(subject_reversed);
        cavc_pline_f(subject_rotated_reversed);
        cavc_pline_f(clip_base);
        cavc_pline_f(clip_rotated);
        cavc_pline_f(clip_reversed);
        cavc_pline_f(clip_rotated_reversed);
    }
}

#[test]
fn pline_boolean_circle_rectangle_not_complementary_role_flip_matrix_parity() {
    let expected_remaining_ab = vec![
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
    ];

    let subject_base = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);
    let subject_reversed = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);

    let clip_base = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );
    let clip_reversed = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );

    unsafe {
        assert_eq!(cavc_pline_invert_direction(subject_reversed), 0);
        assert_eq!(cavc_pline_invert_direction(clip_reversed), 0);
    }

    let subject_variants = [subject_base, subject_reversed];
    let clip_variants = [clip_base, clip_reversed];

    let (baseline_ba_remaining, baseline_ba_subtracted) =
        run_boolean_props(clip_base, subject_base, 2);
    assert!(
        baseline_ba_subtracted.is_empty(),
        "circle-rectangle BA baseline expected empty subtracted, got {baseline_ba_subtracted:?}"
    );

    for a in subject_variants {
        for b in clip_variants {
            let (ab_remaining, ab_subtracted) = run_boolean_props(a, b, 2);
            let (ba_remaining, ba_subtracted) = run_boolean_props(b, a, 2);

            assert!(
                props_set_match_ignore_area_sign(&ab_remaining, &expected_remaining_ab, 1e-4),
                "AB NOT mismatch for circle-rectangle role-flip variant\nab={ab_remaining:?}\nexpected={expected_remaining_ab:?}"
            );
            assert!(
                ab_subtracted.is_empty(),
                "AB NOT expected empty subtracted for circle-rectangle role-flip variant, got {ab_subtracted:?}"
            );

            assert!(
                props_set_match_ignore_area_sign(&ba_remaining, &baseline_ba_remaining, 1e-4),
                "BA NOT mismatch for circle-rectangle role-flip variant\nba={ba_remaining:?}\nbaseline={baseline_ba_remaining:?}"
            );
            assert!(
                ba_subtracted.is_empty(),
                "BA NOT expected empty subtracted for circle-rectangle role-flip variant, got {ba_subtracted:?}"
            );
        }
    }

    unsafe {
        cavc_pline_f(subject_base);
        cavc_pline_f(subject_reversed);
        cavc_pline_f(clip_base);
        cavc_pline_f(clip_reversed);
    }
}

#[test]
fn pline_boolean_circle_rectangle_commutative_role_flip_matrix_parity() {
    let expected_union = vec![PlineProps::new(
        10,
        109.15381629282,
        52.324068506275,
        0.0,
        -10.0,
        10.0,
        10.0,
    )];
    let expected_intersect = vec![PlineProps::new(
        4,
        29.386000046924,
        25.091858029623,
        3.0,
        -4.0,
        6.0,
        6.0,
    )];
    let expected_xor = vec![
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
    ];

    let subject_base = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);
    let subject_reversed = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);

    let clip_base = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );
    let clip_reversed = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );

    unsafe {
        assert_eq!(cavc_pline_invert_direction(subject_reversed), 0);
        assert_eq!(cavc_pline_invert_direction(clip_reversed), 0);
    }

    let subject_variants = [subject_base, subject_reversed];
    let clip_variants = [clip_base, clip_reversed];

    let cases: [(&str, u32, Vec<PlineProps>); 3] = [
        ("circle_rectangle_union", 0, expected_union),
        ("circle_rectangle_intersect", 1, expected_intersect),
        ("circle_rectangle_xor", 3, expected_xor),
    ];

    for (case_name, operation, expected_remaining) in cases {
        for &a in &subject_variants {
            for &b in &clip_variants {
                let (ab_remaining, ab_subtracted) = run_boolean_props(a, b, operation);
                let (ba_remaining, ba_subtracted) = run_boolean_props(b, a, operation);

                assert!(
                    props_set_match_ignore_area_sign(&ab_remaining, &expected_remaining, 1e-4),
                    "AB mismatch for case={case_name} role-flip variant\nab={ab_remaining:?}\nexpected={expected_remaining:?}"
                );
                assert!(
                    props_set_match_ignore_area_sign(&ba_remaining, &expected_remaining, 1e-4),
                    "BA mismatch for case={case_name} role-flip variant\nba={ba_remaining:?}\nexpected={expected_remaining:?}"
                );
                assert!(
                    props_set_match_ignore_area_sign(&ab_remaining, &ba_remaining, 1e-4),
                    "AB/BA commutative mismatch for case={case_name} role-flip variant\nab={ab_remaining:?}\nba={ba_remaining:?}"
                );
                assert!(
                    ab_subtracted.is_empty(),
                    "AB expected empty subtracted for case={case_name}, got {ab_subtracted:?}"
                );
                assert!(
                    ba_subtracted.is_empty(),
                    "BA expected empty subtracted for case={case_name}, got {ba_subtracted:?}"
                );
            }
        }
    }

    unsafe {
        cavc_pline_f(subject_base);
        cavc_pline_f(subject_reversed);
        cavc_pline_f(clip_base);
        cavc_pline_f(clip_reversed);
    }
}

#[test]
fn pline_boolean_circle_rectangle_commutative_start_index_rotation_matrix_parity() {
    let expected_union = vec![PlineProps::new(
        10,
        109.15381629282,
        52.324068506275,
        0.0,
        -10.0,
        10.0,
        10.0,
    )];
    let expected_intersect = vec![PlineProps::new(
        4,
        29.386000046924,
        25.091858029623,
        3.0,
        -4.0,
        6.0,
        6.0,
    )];
    let expected_xor = vec![
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
    ];

    let subject_base = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);
    let subject_rotated = create_pline(&[(10.0, 1.0, 1.0), (0.0, 1.0, 1.0)], true);
    let subject_reversed = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);
    let subject_rotated_reversed = create_pline(&[(10.0, 1.0, 1.0), (0.0, 1.0, 1.0)], true);

    let clip_base = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );
    let clip_rotated = create_pline(
        &[
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
            (3.0, -10.0, 0.0),
        ],
        true,
    );
    let clip_reversed = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );
    let clip_rotated_reversed = create_pline(
        &[
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
            (3.0, -10.0, 0.0),
        ],
        true,
    );

    unsafe {
        assert_eq!(cavc_pline_invert_direction(subject_reversed), 0);
        assert_eq!(cavc_pline_invert_direction(subject_rotated_reversed), 0);
        assert_eq!(cavc_pline_invert_direction(clip_reversed), 0);
        assert_eq!(cavc_pline_invert_direction(clip_rotated_reversed), 0);
    }

    let subject_variants = [
        subject_base,
        subject_rotated,
        subject_reversed,
        subject_rotated_reversed,
    ];
    let clip_variants = [
        clip_base,
        clip_rotated,
        clip_reversed,
        clip_rotated_reversed,
    ];

    let cases: [(&str, u32, Vec<PlineProps>); 3] = [
        ("circle_rectangle_union", 0, expected_union),
        ("circle_rectangle_intersect", 1, expected_intersect),
        ("circle_rectangle_xor", 3, expected_xor),
    ];

    for (case_name, operation, expected_remaining) in cases {
        for &a in &subject_variants {
            for &b in &clip_variants {
                let (ab_remaining, ab_subtracted) = run_boolean_props(a, b, operation);
                let (ba_remaining, ba_subtracted) = run_boolean_props(b, a, operation);

                assert!(
                    props_set_match_ignore_area_sign(&ab_remaining, &expected_remaining, 1e-4),
                    "AB mismatch for case={case_name} rotation/reversal variant\nab={ab_remaining:?}\nexpected={expected_remaining:?}"
                );
                assert!(
                    props_set_match_ignore_area_sign(&ba_remaining, &expected_remaining, 1e-4),
                    "BA mismatch for case={case_name} rotation/reversal variant\nba={ba_remaining:?}\nexpected={expected_remaining:?}"
                );
                assert!(
                    props_set_match_ignore_area_sign(&ab_remaining, &ba_remaining, 1e-4),
                    "AB/BA commutative mismatch for case={case_name} rotation/reversal variant\nab={ab_remaining:?}\nba={ba_remaining:?}"
                );
                assert!(
                    ab_subtracted.is_empty(),
                    "AB expected empty subtracted for case={case_name}, got {ab_subtracted:?}"
                );
                assert!(
                    ba_subtracted.is_empty(),
                    "BA expected empty subtracted for case={case_name}, got {ba_subtracted:?}"
                );
            }
        }
    }

    unsafe {
        cavc_pline_f(subject_base);
        cavc_pline_f(subject_rotated);
        cavc_pline_f(subject_reversed);
        cavc_pline_f(subject_rotated_reversed);
        cavc_pline_f(clip_base);
        cavc_pline_f(clip_rotated);
        cavc_pline_f(clip_reversed);
        cavc_pline_f(clip_rotated_reversed);
    }
}

#[test]
fn pline_boolean_coincident_commutative_start_index_rotation_matrix_parity() {
    let cases = cpp_coincident_boolean_matrix_cases()
        .into_iter()
        .filter(|case| matches!(case.operation, 0 | 1 | 3))
        .collect::<Vec<_>>();

    for case in cases {
        let subject_base = case.subject;
        let clip_base = case.clip;

        let subject_shift = if subject_base.len() > 1 { 1 } else { 0 };
        let clip_shift = if clip_base.len() > 2 {
            2
        } else if clip_base.len() > 1 {
            1
        } else {
            0
        };

        let subject_variants = [
            subject_base.clone(),
            rotate_closed_input(&subject_base, subject_shift),
        ];
        let clip_variants = [
            clip_base.clone(),
            rotate_closed_input(&clip_base, clip_shift),
        ];

        let subject_baseline = create_pline(&subject_base, true);
        let clip_baseline = create_pline(&clip_base, true);
        let (baseline_ab_remaining, baseline_ab_subtracted) =
            run_boolean_props(subject_baseline, clip_baseline, case.operation);
        let (baseline_ba_remaining, baseline_ba_subtracted) =
            run_boolean_props(clip_baseline, subject_baseline, case.operation);

        assert!(
            props_set_match_ignore_area_sign(&baseline_ab_remaining, &baseline_ba_remaining, 1e-4),
            "baseline AB/BA remaining mismatch for case={}\nab={baseline_ab_remaining:?}\nba={baseline_ba_remaining:?}",
            case.name
        );
        assert!(
            props_set_match_ignore_area_sign(
                &baseline_ab_subtracted,
                &baseline_ba_subtracted,
                1e-4
            ),
            "baseline AB/BA subtracted mismatch for case={}\nab={baseline_ab_subtracted:?}\nba={baseline_ba_subtracted:?}",
            case.name
        );

        unsafe {
            cavc_pline_f(subject_baseline);
            cavc_pline_f(clip_baseline);
        }

        for subject_input in &subject_variants {
            for clip_input in &clip_variants {
                for &subject_reversed in &[false, true] {
                    for &clip_reversed in &[false, true] {
                        let subject_pline = create_pline(subject_input, true);
                        let clip_pline = create_pline(clip_input, true);

                        unsafe {
                            if subject_reversed {
                                assert_eq!(cavc_pline_invert_direction(subject_pline), 0);
                            }
                            if clip_reversed {
                                assert_eq!(cavc_pline_invert_direction(clip_pline), 0);
                            }
                        }

                        let (ab_remaining, ab_subtracted) =
                            run_boolean_props(subject_pline, clip_pline, case.operation);
                        let (ba_remaining, ba_subtracted) =
                            run_boolean_props(clip_pline, subject_pline, case.operation);

                        assert!(
                            props_set_match_ignore_area_sign(
                                &ab_remaining,
                                &baseline_ab_remaining,
                                1e-4
                            ),
                            "AB remaining mismatch for case={} rotation/reversal variant (subject_reversed={}, clip_reversed={})\nab={ab_remaining:?}\nbaseline={baseline_ab_remaining:?}",
                            case.name,
                            subject_reversed,
                            clip_reversed
                        );
                        assert!(
                            props_set_match_ignore_area_sign(
                                &ab_subtracted,
                                &baseline_ab_subtracted,
                                1e-4
                            ),
                            "AB subtracted mismatch for case={} rotation/reversal variant (subject_reversed={}, clip_reversed={})\nab={ab_subtracted:?}\nbaseline={baseline_ab_subtracted:?}",
                            case.name,
                            subject_reversed,
                            clip_reversed
                        );
                        assert!(
                            props_set_match_ignore_area_sign(
                                &ba_remaining,
                                &baseline_ba_remaining,
                                1e-4
                            ),
                            "BA remaining mismatch for case={} rotation/reversal variant (subject_reversed={}, clip_reversed={})\nba={ba_remaining:?}\nbaseline={baseline_ba_remaining:?}",
                            case.name,
                            subject_reversed,
                            clip_reversed
                        );
                        assert!(
                            props_set_match_ignore_area_sign(
                                &ba_subtracted,
                                &baseline_ba_subtracted,
                                1e-4
                            ),
                            "BA subtracted mismatch for case={} rotation/reversal variant (subject_reversed={}, clip_reversed={})\nba={ba_subtracted:?}\nbaseline={baseline_ba_subtracted:?}",
                            case.name,
                            subject_reversed,
                            clip_reversed
                        );
                        assert!(
                            props_set_match_ignore_area_sign(&ab_remaining, &ba_remaining, 1e-4),
                            "AB/BA remaining commutative mismatch for case={} rotation/reversal variant (subject_reversed={}, clip_reversed={})\nab={ab_remaining:?}\nba={ba_remaining:?}",
                            case.name,
                            subject_reversed,
                            clip_reversed
                        );
                        assert!(
                            props_set_match_ignore_area_sign(&ab_subtracted, &ba_subtracted, 1e-4),
                            "AB/BA subtracted commutative mismatch for case={} rotation/reversal variant (subject_reversed={}, clip_reversed={})\nab={ab_subtracted:?}\nba={ba_subtracted:?}",
                            case.name,
                            subject_reversed,
                            clip_reversed
                        );

                        unsafe {
                            cavc_pline_f(subject_pline);
                            cavc_pline_f(clip_pline);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn pline_boolean_coincident_commutative_role_flip_matrix_parity() {
    let cases = cpp_coincident_boolean_matrix_cases()
        .into_iter()
        .filter(|case| matches!(case.operation, 0 | 1 | 3))
        .collect::<Vec<_>>();

    for case in cases {
        let subject_base = case.subject;
        let clip_base = case.clip;

        let subject_baseline = create_pline(&subject_base, true);
        let clip_baseline = create_pline(&clip_base, true);
        let (baseline_ab_remaining, baseline_ab_subtracted) =
            run_boolean_props(subject_baseline, clip_baseline, case.operation);
        let (baseline_ba_remaining, baseline_ba_subtracted) =
            run_boolean_props(clip_baseline, subject_baseline, case.operation);

        assert!(
            props_set_match_ignore_area_sign(&baseline_ab_remaining, &baseline_ba_remaining, 1e-4),
            "baseline AB/BA remaining mismatch for case={}\nab={baseline_ab_remaining:?}\nba={baseline_ba_remaining:?}",
            case.name
        );
        assert!(
            props_set_match_ignore_area_sign(
                &baseline_ab_subtracted,
                &baseline_ba_subtracted,
                1e-4
            ),
            "baseline AB/BA subtracted mismatch for case={}\nab={baseline_ab_subtracted:?}\nba={baseline_ba_subtracted:?}",
            case.name
        );

        unsafe {
            cavc_pline_f(subject_baseline);
            cavc_pline_f(clip_baseline);
        }

        for &subject_reversed in &[false, true] {
            for &clip_reversed in &[false, true] {
                let subject_pline = create_pline(&subject_base, true);
                let clip_pline = create_pline(&clip_base, true);

                unsafe {
                    if subject_reversed {
                        assert_eq!(cavc_pline_invert_direction(subject_pline), 0);
                    }
                    if clip_reversed {
                        assert_eq!(cavc_pline_invert_direction(clip_pline), 0);
                    }
                }

                let (ab_remaining, ab_subtracted) =
                    run_boolean_props(subject_pline, clip_pline, case.operation);
                let (ba_remaining, ba_subtracted) =
                    run_boolean_props(clip_pline, subject_pline, case.operation);

                assert!(
                    props_set_match_ignore_area_sign(&ab_remaining, &baseline_ab_remaining, 1e-4),
                    "AB remaining mismatch for case={} role-flip variant (subject_reversed={}, clip_reversed={})\nab={ab_remaining:?}\nbaseline={baseline_ab_remaining:?}",
                    case.name,
                    subject_reversed,
                    clip_reversed
                );
                assert!(
                    props_set_match_ignore_area_sign(&ba_remaining, &baseline_ba_remaining, 1e-4),
                    "BA remaining mismatch for case={} role-flip variant (subject_reversed={}, clip_reversed={})\nba={ba_remaining:?}\nbaseline={baseline_ba_remaining:?}",
                    case.name,
                    subject_reversed,
                    clip_reversed
                );
                assert!(
                    props_set_match_ignore_area_sign(&ab_remaining, &ba_remaining, 1e-4),
                    "AB/BA remaining commutative mismatch for case={} role-flip variant (subject_reversed={}, clip_reversed={})\nab={ab_remaining:?}\nba={ba_remaining:?}",
                    case.name,
                    subject_reversed,
                    clip_reversed
                );
                assert!(
                    props_set_match_ignore_area_sign(&ab_subtracted, &baseline_ab_subtracted, 1e-4),
                    "AB subtracted mismatch for case={} role-flip variant (subject_reversed={}, clip_reversed={})\nab={ab_subtracted:?}\nbaseline={baseline_ab_subtracted:?}",
                    case.name,
                    subject_reversed,
                    clip_reversed
                );
                assert!(
                    props_set_match_ignore_area_sign(&ba_subtracted, &baseline_ba_subtracted, 1e-4),
                    "BA subtracted mismatch for case={} role-flip variant (subject_reversed={}, clip_reversed={})\nba={ba_subtracted:?}\nbaseline={baseline_ba_subtracted:?}",
                    case.name,
                    subject_reversed,
                    clip_reversed
                );
                assert!(
                    props_set_match_ignore_area_sign(&ab_subtracted, &ba_subtracted, 1e-4),
                    "AB/BA subtracted commutative mismatch for case={} role-flip variant (subject_reversed={}, clip_reversed={})\nab={ab_subtracted:?}\nba={ba_subtracted:?}",
                    case.name,
                    subject_reversed,
                    clip_reversed
                );

                unsafe {
                    cavc_pline_f(subject_pline);
                    cavc_pline_f(clip_pline);
                }
            }
        }
    }
}

#[test]
fn pline_boolean_coincident_not_complementary_role_flip_matrix_parity() {
    let cases = cpp_coincident_boolean_matrix_cases()
        .into_iter()
        .filter(|case| case.operation == 2)
        .collect::<Vec<_>>();

    for case in cases {
        let subject_base = case.subject;
        let clip_base = case.clip;

        let subject_baseline = create_pline(&subject_base, true);
        let clip_baseline = create_pline(&clip_base, true);
        let (baseline_ab_remaining, baseline_ab_subtracted) =
            run_boolean_props(subject_baseline, clip_baseline, case.operation);
        let (baseline_ba_remaining, baseline_ba_subtracted) =
            run_boolean_props(clip_baseline, subject_baseline, case.operation);

        assert!(
            baseline_ab_subtracted.is_empty(),
            "baseline AB NOT expected empty subtracted for case={}, got {baseline_ab_subtracted:?}",
            case.name
        );
        assert!(
            baseline_ba_subtracted.is_empty(),
            "baseline BA NOT expected empty subtracted for case={}, got {baseline_ba_subtracted:?}",
            case.name
        );

        unsafe {
            cavc_pline_f(subject_baseline);
            cavc_pline_f(clip_baseline);
        }

        for &subject_reversed in &[false, true] {
            for &clip_reversed in &[false, true] {
                let subject_pline = create_pline(&subject_base, true);
                let clip_pline = create_pline(&clip_base, true);

                unsafe {
                    if subject_reversed {
                        assert_eq!(cavc_pline_invert_direction(subject_pline), 0);
                    }
                    if clip_reversed {
                        assert_eq!(cavc_pline_invert_direction(clip_pline), 0);
                    }
                }

                let (ab_remaining, ab_subtracted) =
                    run_boolean_props(subject_pline, clip_pline, case.operation);
                let (ba_remaining, ba_subtracted) =
                    run_boolean_props(clip_pline, subject_pline, case.operation);

                assert!(
                    props_set_match_ignore_area_sign(&ab_remaining, &baseline_ab_remaining, 1e-4),
                    "AB NOT remaining mismatch for case={} role-flip variant (subject_reversed={}, clip_reversed={})\nab={ab_remaining:?}\nbaseline={baseline_ab_remaining:?}",
                    case.name,
                    subject_reversed,
                    clip_reversed
                );
                assert!(
                    props_set_match_ignore_area_sign(&ba_remaining, &baseline_ba_remaining, 1e-4),
                    "BA NOT remaining mismatch for case={} role-flip variant (subject_reversed={}, clip_reversed={})\nba={ba_remaining:?}\nbaseline={baseline_ba_remaining:?}",
                    case.name,
                    subject_reversed,
                    clip_reversed
                );
                assert!(
                    ab_subtracted.is_empty(),
                    "AB NOT expected empty subtracted for case={} role-flip variant (subject_reversed={}, clip_reversed={}), got {ab_subtracted:?}",
                    case.name,
                    subject_reversed,
                    clip_reversed
                );
                assert!(
                    ba_subtracted.is_empty(),
                    "BA NOT expected empty subtracted for case={} role-flip variant (subject_reversed={}, clip_reversed={}), got {ba_subtracted:?}",
                    case.name,
                    subject_reversed,
                    clip_reversed
                );

                unsafe {
                    cavc_pline_f(subject_pline);
                    cavc_pline_f(clip_pline);
                }
            }
        }
    }
}

#[test]
fn pline_boolean_coincident_not_complementary_start_index_rotation_matrix_parity() {
    let cases = cpp_coincident_boolean_matrix_cases()
        .into_iter()
        .filter(|case| case.operation == 2)
        .collect::<Vec<_>>();

    for case in cases {
        let subject_base = case.subject;
        let clip_base = case.clip;

        let subject_shift = if subject_base.len() > 1 { 1 } else { 0 };
        let clip_shift = if clip_base.len() > 2 {
            2
        } else if clip_base.len() > 1 {
            1
        } else {
            0
        };

        let subject_variants = [
            subject_base.clone(),
            rotate_closed_input(&subject_base, subject_shift),
        ];
        let clip_variants = [
            clip_base.clone(),
            rotate_closed_input(&clip_base, clip_shift),
        ];

        let subject_baseline = create_pline(&subject_base, true);
        let clip_baseline = create_pline(&clip_base, true);
        let (baseline_ab_remaining, baseline_ab_subtracted) =
            run_boolean_props(subject_baseline, clip_baseline, case.operation);
        let (baseline_ba_remaining, baseline_ba_subtracted) =
            run_boolean_props(clip_baseline, subject_baseline, case.operation);

        assert!(
            baseline_ab_subtracted.is_empty(),
            "baseline AB NOT expected empty subtracted for case={}, got {baseline_ab_subtracted:?}",
            case.name
        );
        assert!(
            baseline_ba_subtracted.is_empty(),
            "baseline BA NOT expected empty subtracted for case={}, got {baseline_ba_subtracted:?}",
            case.name
        );

        unsafe {
            cavc_pline_f(subject_baseline);
            cavc_pline_f(clip_baseline);
        }

        for subject_input in &subject_variants {
            for clip_input in &clip_variants {
                for &subject_reversed in &[false, true] {
                    for &clip_reversed in &[false, true] {
                        let subject_pline = create_pline(subject_input, true);
                        let clip_pline = create_pline(clip_input, true);

                        unsafe {
                            if subject_reversed {
                                assert_eq!(cavc_pline_invert_direction(subject_pline), 0);
                            }
                            if clip_reversed {
                                assert_eq!(cavc_pline_invert_direction(clip_pline), 0);
                            }
                        }

                        let (ab_remaining, ab_subtracted) =
                            run_boolean_props(subject_pline, clip_pline, case.operation);
                        let (ba_remaining, ba_subtracted) =
                            run_boolean_props(clip_pline, subject_pline, case.operation);

                        assert!(
                            props_set_match_ignore_area_sign(
                                &ab_remaining,
                                &baseline_ab_remaining,
                                1e-4
                            ),
                            "AB NOT remaining mismatch for case={} rotation/reversal variant (subject_reversed={}, clip_reversed={})\nab={ab_remaining:?}\nbaseline={baseline_ab_remaining:?}",
                            case.name,
                            subject_reversed,
                            clip_reversed
                        );
                        assert!(
                            props_set_match_ignore_area_sign(
                                &ba_remaining,
                                &baseline_ba_remaining,
                                1e-4
                            ),
                            "BA NOT remaining mismatch for case={} rotation/reversal variant (subject_reversed={}, clip_reversed={})\nba={ba_remaining:?}\nbaseline={baseline_ba_remaining:?}",
                            case.name,
                            subject_reversed,
                            clip_reversed
                        );
                        assert!(
                            ab_subtracted.is_empty(),
                            "AB NOT expected empty subtracted for case={} rotation/reversal variant (subject_reversed={}, clip_reversed={}), got {ab_subtracted:?}",
                            case.name,
                            subject_reversed,
                            clip_reversed
                        );
                        assert!(
                            ba_subtracted.is_empty(),
                            "BA NOT expected empty subtracted for case={} rotation/reversal variant (subject_reversed={}, clip_reversed={}), got {ba_subtracted:?}",
                            case.name,
                            subject_reversed,
                            clip_reversed
                        );

                        unsafe {
                            cavc_pline_f(subject_pline);
                            cavc_pline_f(clip_pline);
                        }
                    }
                }
            }
        }
    }
}

#[test]
fn pline_boolean_coincident_case1_cpp_matrix_parity() {
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

    let cases = [
        BooleanCaseWithInputs {
            name: "coincident_case1_union",
            subject: pline_a,
            clip: pline_b,
            operation: 0,
            expected_remaining: vec![PlineProps::new(
                12,
                -0.032967809756574,
                1.6071238962168,
                -0.255,
                -0.005,
                0.255,
                0.295,
            )],
            expected_subtracted: vec![],
        },
        BooleanCaseWithInputs {
            name: "coincident_case1_excludeAFromB",
            subject: pline_a,
            clip: pline_b,
            operation: 2,
            expected_remaining: vec![PlineProps::new(
                4,
                -0.0023892699081699,
                0.49570796326795,
                -0.105,
                -0.005,
                -0.095,
                0.235,
            )],
            expected_subtracted: vec![],
        },
        BooleanCaseWithInputs {
            name: "coincident_case1_excludeBFromA",
            subject: pline_b,
            clip: pline_a,
            operation: 2,
            expected_remaining: vec![PlineProps::new(
                10,
                -0.030578539848405,
                1.1314159329489,
                -0.255,
                0.235,
                0.255,
                0.295,
            )],
            expected_subtracted: vec![],
        },
        BooleanCaseWithInputs {
            name: "coincident_case1_intersect",
            subject: pline_a,
            clip: pline_b,
            operation: 1,
            expected_remaining: vec![],
            expected_subtracted: vec![],
        },
        BooleanCaseWithInputs {
            name: "coincident_case1_xor",
            subject: pline_a,
            clip: pline_b,
            operation: 3,
            expected_remaining: vec![
                PlineProps::new(
                    4,
                    -0.0023892699081699,
                    0.49570796326795,
                    -0.105,
                    -0.005,
                    -0.095,
                    0.235,
                ),
                PlineProps::new(
                    10,
                    0.030578539848405,
                    1.1314159329489,
                    -0.255,
                    0.235,
                    0.255,
                    0.295,
                ),
            ],
            expected_subtracted: vec![],
        },
    ];

    let actual: Vec<(&str, u32)> = cases
        .iter()
        .map(|case| (case.name, case.operation))
        .collect();
    assert_boolean_case_source_mapping(
        &actual,
        &CPP_COINCIDENT_CASE1_SOURCE_MATRIX,
        "coincident_case1 default matrix",
    );

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

    let actual: Vec<(&str, u32)> = cases
        .iter()
        .map(|case| (case.name, case.operation))
        .collect();
    assert_boolean_case_source_mapping(
        &actual,
        &CPP_COINCIDENT_CASE2_SOURCE_MATRIX,
        "coincident_case2 default matrix",
    );

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
    let ops = CPP_CIRCLE_RECT_SOURCE_OPS; // Or, Not, And, Xor

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
    let cases = cpp_coincident_boolean_matrix_cases();

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
fn pline_boolean_combine_with_self_invariants_vertex_exact_cpp_parity() {
    let source_vertices = vec![
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
    ];
    let expected_forward = tuple_vertices_to_cavc(&source_vertices);
    let pline = create_pline(&source_vertices, true);

    let mut rev_pline = ptr::null();
    unsafe {
        assert_eq!(cavc_pline_clone(pline, &mut rev_pline), 0);
        assert_eq!(cavc_pline_invert_direction(rev_pline as *mut _), 0);
    }
    let expected_reverse = read_vertices(rev_pline);

    // Union with self is self (forward + reversed).
    let (remaining, subtracted) = run_boolean_vertexes(pline, pline, 0);
    assert_eq!(remaining.len(), 1);
    assert!(subtracted.is_empty());
    compare_vertexes(&remaining[0], &expected_forward);

    let (remaining, subtracted) = run_boolean_vertexes(rev_pline, rev_pline, 0);
    assert_eq!(remaining.len(), 1);
    assert!(subtracted.is_empty());
    compare_vertexes(&remaining[0], &expected_reverse);

    // Exclude with self is empty (forward/reverse combinations).
    let (remaining, subtracted) = run_boolean_vertexes(pline, pline, 2);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_vertexes(rev_pline, rev_pline, 2);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_vertexes(rev_pline, pline, 2);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_vertexes(pline, rev_pline, 2);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    // Intersect with self is self (forward + reversed).
    let (remaining, subtracted) = run_boolean_vertexes(pline, pline, 1);
    assert_eq!(remaining.len(), 1);
    assert!(subtracted.is_empty());
    compare_vertexes(&remaining[0], &expected_forward);

    let (remaining, subtracted) = run_boolean_vertexes(rev_pline, rev_pline, 1);
    assert_eq!(remaining.len(), 1);
    assert!(subtracted.is_empty());
    compare_vertexes(&remaining[0], &expected_reverse);

    // XOR with self is empty (forward/reverse combinations).
    let (remaining, subtracted) = run_boolean_vertexes(pline, pline, 3);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_vertexes(rev_pline, rev_pline, 3);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_vertexes(rev_pline, pline, 3);
    assert!(remaining.is_empty());
    assert!(subtracted.is_empty());

    let (remaining, subtracted) = run_boolean_vertexes(pline, rev_pline, 3);
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

        for operation in CPP_CIRCLE_RECT_SOURCE_OPS {
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
fn pline_boolean_options_path_circle_rectangle_role_flip_matrix_cpp_parity() {
    let subject_base = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);
    let subject_reversed = create_pline(&[(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)], true);
    let clip_base = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );
    let clip_reversed = create_pline(
        &[
            (3.0, -10.0, 0.0),
            (6.0, -10.0, 0.0),
            (6.0, 10.0, 0.0),
            (3.0, 10.0, 0.0),
        ],
        true,
    );

    unsafe {
        assert_eq!(cavc_pline_invert_direction(subject_reversed), 0);
        assert_eq!(cavc_pline_invert_direction(clip_reversed), 0);
    }

    let subject_variants = [subject_base, subject_reversed];
    let clip_variants = [clip_base, clip_reversed];

    for &a in &subject_variants {
        for &b in &clip_variants {
            for operation in CPP_CIRCLE_RECT_SOURCE_OPS {
                let (default_ab_remaining, default_ab_subtracted) =
                    run_boolean_props(a, b, operation);
                let (default_ba_remaining, default_ba_subtracted) =
                    run_boolean_props(b, a, operation);

                unsafe {
                    let mut options_ab = cavc_pline_boolean_o {
                        pline1_aabb_index: std::ptr::null(),
                        pos_equal_eps: f64::NAN,
                        collapsed_area_eps: f64::NAN,
                    };
                    assert_eq!(cavc_pline_boolean_o_init(&mut options_ab), 0);
                    let mut aabb_ab = ptr::null();
                    assert_eq!(cavc_pline_create_approx_aabbindex(a, &mut aabb_ab), 0);
                    options_ab.pline1_aabb_index = aabb_ab;

                    let (opt_ab_remaining, opt_ab_subtracted) =
                        run_boolean_props_with_options(a, b, operation, &options_ab);

                    assert!(
                        props_set_match_ignore_area_sign(
                            &opt_ab_remaining,
                            &default_ab_remaining,
                            1e-4
                        ) && props_set_match_ignore_area_sign(
                            &default_ab_remaining,
                            &opt_ab_remaining,
                            1e-4
                        ),
                        "options AB remaining mismatch for op={operation} role-flip variant\ndefault={default_ab_remaining:?}\noptions={opt_ab_remaining:?}"
                    );
                    assert!(
                        props_set_match_ignore_area_sign(
                            &opt_ab_subtracted,
                            &default_ab_subtracted,
                            1e-4
                        ) && props_set_match_ignore_area_sign(
                            &default_ab_subtracted,
                            &opt_ab_subtracted,
                            1e-4
                        ),
                        "options AB subtracted mismatch for op={operation} role-flip variant\ndefault={default_ab_subtracted:?}\noptions={opt_ab_subtracted:?}"
                    );

                    cavc_aabbindex_f(aabb_ab as *mut _);

                    let mut options_ba = cavc_pline_boolean_o {
                        pline1_aabb_index: std::ptr::null(),
                        pos_equal_eps: f64::NAN,
                        collapsed_area_eps: f64::NAN,
                    };
                    assert_eq!(cavc_pline_boolean_o_init(&mut options_ba), 0);
                    let mut aabb_ba = ptr::null();
                    assert_eq!(cavc_pline_create_approx_aabbindex(b, &mut aabb_ba), 0);
                    options_ba.pline1_aabb_index = aabb_ba;

                    let (opt_ba_remaining, opt_ba_subtracted) =
                        run_boolean_props_with_options(b, a, operation, &options_ba);

                    assert!(
                        props_set_match_ignore_area_sign(
                            &opt_ba_remaining,
                            &default_ba_remaining,
                            1e-4
                        ) && props_set_match_ignore_area_sign(
                            &default_ba_remaining,
                            &opt_ba_remaining,
                            1e-4
                        ),
                        "options BA remaining mismatch for op={operation} role-flip variant\ndefault={default_ba_remaining:?}\noptions={opt_ba_remaining:?}"
                    );
                    assert!(
                        props_set_match_ignore_area_sign(
                            &opt_ba_subtracted,
                            &default_ba_subtracted,
                            1e-4
                        ) && props_set_match_ignore_area_sign(
                            &default_ba_subtracted,
                            &opt_ba_subtracted,
                            1e-4
                        ),
                        "options BA subtracted mismatch for op={operation} role-flip variant\ndefault={default_ba_subtracted:?}\noptions={opt_ba_subtracted:?}"
                    );

                    cavc_aabbindex_f(aabb_ba as *mut _);
                }
            }
        }
    }

    unsafe {
        cavc_pline_f(subject_base);
        cavc_pline_f(subject_reversed);
        cavc_pline_f(clip_base);
        cavc_pline_f(clip_reversed);
    }
}

#[test]
fn pline_boolean_options_path_circle_rectangle_start_index_rotation_matrix_cpp_parity() {
    let subject_base_input = vec![(0.0, 1.0, 1.0), (10.0, 1.0, 1.0)];
    let subject_rotated_input = vec![(10.0, 1.0, 1.0), (0.0, 1.0, 1.0)];
    let clip_base_input = vec![
        (3.0, -10.0, 0.0),
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0),
    ];
    let clip_rotated_input = vec![
        (6.0, -10.0, 0.0),
        (6.0, 10.0, 0.0),
        (3.0, 10.0, 0.0),
        (3.0, -10.0, 0.0),
    ];

    let subject_variants = [subject_base_input, subject_rotated_input];
    let clip_variants = [clip_base_input, clip_rotated_input];

    for subject_input in &subject_variants {
        for clip_input in &clip_variants {
            for &subject_reversed in &[false, true] {
                for &clip_reversed in &[false, true] {
                    let subject = create_pline(subject_input, true);
                    let clip = create_pline(clip_input, true);

                    unsafe {
                        if subject_reversed {
                            assert_eq!(cavc_pline_invert_direction(subject), 0);
                        }
                        if clip_reversed {
                            assert_eq!(cavc_pline_invert_direction(clip), 0);
                        }
                    }

                    for operation in CPP_CIRCLE_RECT_SOURCE_OPS {
                        let (default_ab_remaining, default_ab_subtracted) =
                            run_boolean_props(subject, clip, operation);
                        let (default_ba_remaining, default_ba_subtracted) =
                            run_boolean_props(clip, subject, operation);

                        unsafe {
                            let mut options_ab = cavc_pline_boolean_o {
                                pline1_aabb_index: std::ptr::null(),
                                pos_equal_eps: f64::NAN,
                                collapsed_area_eps: f64::NAN,
                            };
                            assert_eq!(cavc_pline_boolean_o_init(&mut options_ab), 0);
                            let mut aabb_ab = ptr::null();
                            assert_eq!(
                                cavc_pline_create_approx_aabbindex(subject, &mut aabb_ab),
                                0
                            );
                            options_ab.pline1_aabb_index = aabb_ab;

                            let (opt_ab_remaining, opt_ab_subtracted) =
                                run_boolean_props_with_options(
                                    subject,
                                    clip,
                                    operation,
                                    &options_ab,
                                );

                            assert!(
                                props_set_match_ignore_area_sign(
                                    &opt_ab_remaining,
                                    &default_ab_remaining,
                                    1e-4
                                ) && props_set_match_ignore_area_sign(
                                    &default_ab_remaining,
                                    &opt_ab_remaining,
                                    1e-4
                                ),
                                "options AB remaining mismatch for start-index-rotation matrix op={operation} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ab_remaining:?}\noptions={opt_ab_remaining:?}"
                            );
                            assert!(
                                props_set_match_ignore_area_sign(
                                    &opt_ab_subtracted,
                                    &default_ab_subtracted,
                                    1e-4
                                ) && props_set_match_ignore_area_sign(
                                    &default_ab_subtracted,
                                    &opt_ab_subtracted,
                                    1e-4
                                ),
                                "options AB subtracted mismatch for start-index-rotation matrix op={operation} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ab_subtracted:?}\noptions={opt_ab_subtracted:?}"
                            );

                            cavc_aabbindex_f(aabb_ab as *mut _);

                            let mut options_ba = cavc_pline_boolean_o {
                                pline1_aabb_index: std::ptr::null(),
                                pos_equal_eps: f64::NAN,
                                collapsed_area_eps: f64::NAN,
                            };
                            assert_eq!(cavc_pline_boolean_o_init(&mut options_ba), 0);
                            let mut aabb_ba = ptr::null();
                            assert_eq!(cavc_pline_create_approx_aabbindex(clip, &mut aabb_ba), 0);
                            options_ba.pline1_aabb_index = aabb_ba;

                            let (opt_ba_remaining, opt_ba_subtracted) =
                                run_boolean_props_with_options(
                                    clip,
                                    subject,
                                    operation,
                                    &options_ba,
                                );

                            assert!(
                                props_set_match_ignore_area_sign(
                                    &opt_ba_remaining,
                                    &default_ba_remaining,
                                    1e-4
                                ) && props_set_match_ignore_area_sign(
                                    &default_ba_remaining,
                                    &opt_ba_remaining,
                                    1e-4
                                ),
                                "options BA remaining mismatch for start-index-rotation matrix op={operation} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ba_remaining:?}\noptions={opt_ba_remaining:?}"
                            );
                            assert!(
                                props_set_match_ignore_area_sign(
                                    &opt_ba_subtracted,
                                    &default_ba_subtracted,
                                    1e-4
                                ) && props_set_match_ignore_area_sign(
                                    &default_ba_subtracted,
                                    &opt_ba_subtracted,
                                    1e-4
                                ),
                                "options BA subtracted mismatch for start-index-rotation matrix op={operation} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ba_subtracted:?}\noptions={opt_ba_subtracted:?}"
                            );

                            cavc_aabbindex_f(aabb_ba as *mut _);
                        }
                    }

                    unsafe {
                        cavc_pline_f(subject);
                        cavc_pline_f(clip);
                    }
                }
            }
        }
    }
}

#[test]
fn pline_boolean_options_coincident_commutative_role_flip_matrix_cpp_parity() {
    let coincident_cases = [
        ("coincident_case1", cpp_coincident_case1_inputs()),
        ("coincident_case2", cpp_coincident_case2_inputs()),
    ];

    for (case_name, (subject_input, clip_input)) in coincident_cases {
        for operation in [0_u32, 1_u32, 3_u32] {
            for &subject_reversed in &[false, true] {
                for &clip_reversed in &[false, true] {
                    let subject = create_pline(&subject_input, true);
                    let clip = create_pline(&clip_input, true);

                    unsafe {
                        if subject_reversed {
                            assert_eq!(cavc_pline_invert_direction(subject), 0);
                        }
                        if clip_reversed {
                            assert_eq!(cavc_pline_invert_direction(clip), 0);
                        }
                    }

                    let (default_ab_remaining, default_ab_subtracted) =
                        run_boolean_props(subject, clip, operation);
                    let (default_ba_remaining, default_ba_subtracted) =
                        run_boolean_props(clip, subject, operation);

                    unsafe {
                        let mut options_ab = cavc_pline_boolean_o {
                            pline1_aabb_index: std::ptr::null(),
                            pos_equal_eps: f64::NAN,
                            collapsed_area_eps: f64::NAN,
                        };
                        assert_eq!(cavc_pline_boolean_o_init(&mut options_ab), 0);
                        let mut aabb_ab = ptr::null();
                        assert_eq!(cavc_pline_create_approx_aabbindex(subject, &mut aabb_ab), 0);
                        options_ab.pline1_aabb_index = aabb_ab;

                        let (opt_ab_remaining, opt_ab_subtracted) =
                            run_boolean_props_with_options(subject, clip, operation, &options_ab);

                        assert!(
                            props_set_match_ignore_area_sign(
                                &opt_ab_remaining,
                                &default_ab_remaining,
                                1e-4
                            ) && props_set_match_ignore_area_sign(
                                &default_ab_remaining,
                                &opt_ab_remaining,
                                1e-4
                            ),
                            "coincident options AB remaining mismatch for {case_name} op={operation} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ab_remaining:?}\noptions={opt_ab_remaining:?}"
                        );
                        assert!(
                            props_set_match_ignore_area_sign(
                                &opt_ab_subtracted,
                                &default_ab_subtracted,
                                1e-4
                            ) && props_set_match_ignore_area_sign(
                                &default_ab_subtracted,
                                &opt_ab_subtracted,
                                1e-4
                            ),
                            "coincident options AB subtracted mismatch for {case_name} op={operation} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ab_subtracted:?}\noptions={opt_ab_subtracted:?}"
                        );

                        cavc_aabbindex_f(aabb_ab as *mut _);

                        let mut options_ba = cavc_pline_boolean_o {
                            pline1_aabb_index: std::ptr::null(),
                            pos_equal_eps: f64::NAN,
                            collapsed_area_eps: f64::NAN,
                        };
                        assert_eq!(cavc_pline_boolean_o_init(&mut options_ba), 0);
                        let mut aabb_ba = ptr::null();
                        assert_eq!(cavc_pline_create_approx_aabbindex(clip, &mut aabb_ba), 0);
                        options_ba.pline1_aabb_index = aabb_ba;

                        let (opt_ba_remaining, opt_ba_subtracted) =
                            run_boolean_props_with_options(clip, subject, operation, &options_ba);

                        assert!(
                            props_set_match_ignore_area_sign(
                                &opt_ba_remaining,
                                &default_ba_remaining,
                                1e-4
                            ) && props_set_match_ignore_area_sign(
                                &default_ba_remaining,
                                &opt_ba_remaining,
                                1e-4
                            ),
                            "coincident options BA remaining mismatch for {case_name} op={operation} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ba_remaining:?}\noptions={opt_ba_remaining:?}"
                        );
                        assert!(
                            props_set_match_ignore_area_sign(
                                &opt_ba_subtracted,
                                &default_ba_subtracted,
                                1e-4
                            ) && props_set_match_ignore_area_sign(
                                &default_ba_subtracted,
                                &opt_ba_subtracted,
                                1e-4
                            ),
                            "coincident options BA subtracted mismatch for {case_name} op={operation} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ba_subtracted:?}\noptions={opt_ba_subtracted:?}"
                        );
                        assert!(
                            props_set_match_ignore_area_sign(
                                &opt_ab_remaining,
                                &opt_ba_remaining,
                                1e-4
                            ),
                            "coincident options AB/BA remaining mismatch for {case_name} op={operation} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\nab={opt_ab_remaining:?}\nba={opt_ba_remaining:?}"
                        );
                        assert!(
                            props_set_match_ignore_area_sign(
                                &opt_ab_subtracted,
                                &opt_ba_subtracted,
                                1e-4
                            ),
                            "coincident options AB/BA subtracted mismatch for {case_name} op={operation} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\nab={opt_ab_subtracted:?}\nba={opt_ba_subtracted:?}"
                        );

                        cavc_aabbindex_f(aabb_ba as *mut _);
                        cavc_pline_f(subject);
                        cavc_pline_f(clip);
                    }
                }
            }
        }
    }
}

#[test]
fn pline_boolean_options_coincident_not_complementary_role_flip_matrix_cpp_parity() {
    let coincident_cases = [
        ("coincident_case1", cpp_coincident_case1_inputs()),
        ("coincident_case2", cpp_coincident_case2_inputs()),
    ];

    for (case_name, (subject_input, clip_input)) in coincident_cases {
        for &subject_reversed in &[false, true] {
            for &clip_reversed in &[false, true] {
                let subject = create_pline(&subject_input, true);
                let clip = create_pline(&clip_input, true);

                unsafe {
                    if subject_reversed {
                        assert_eq!(cavc_pline_invert_direction(subject), 0);
                    }
                    if clip_reversed {
                        assert_eq!(cavc_pline_invert_direction(clip), 0);
                    }
                }

                let (default_ab_remaining, default_ab_subtracted) =
                    run_boolean_props(subject, clip, 2);
                let (default_ba_remaining, default_ba_subtracted) =
                    run_boolean_props(clip, subject, 2);

                unsafe {
                    let mut options_ab = cavc_pline_boolean_o {
                        pline1_aabb_index: std::ptr::null(),
                        pos_equal_eps: f64::NAN,
                        collapsed_area_eps: f64::NAN,
                    };
                    assert_eq!(cavc_pline_boolean_o_init(&mut options_ab), 0);
                    let mut aabb_ab = ptr::null();
                    assert_eq!(cavc_pline_create_approx_aabbindex(subject, &mut aabb_ab), 0);
                    options_ab.pline1_aabb_index = aabb_ab;

                    let (opt_ab_remaining, opt_ab_subtracted) =
                        run_boolean_props_with_options(subject, clip, 2, &options_ab);

                    assert!(
                        props_set_match_ignore_area_sign(
                            &opt_ab_remaining,
                            &default_ab_remaining,
                            1e-4
                        ) && props_set_match_ignore_area_sign(
                            &default_ab_remaining,
                            &opt_ab_remaining,
                            1e-4
                        ),
                        "coincident options AB NOT remaining mismatch for {case_name} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ab_remaining:?}\noptions={opt_ab_remaining:?}"
                    );
                    assert!(
                        props_set_match_ignore_area_sign(
                            &opt_ab_subtracted,
                            &default_ab_subtracted,
                            1e-4
                        ) && props_set_match_ignore_area_sign(
                            &default_ab_subtracted,
                            &opt_ab_subtracted,
                            1e-4
                        ),
                        "coincident options AB NOT subtracted mismatch for {case_name} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ab_subtracted:?}\noptions={opt_ab_subtracted:?}"
                    );

                    cavc_aabbindex_f(aabb_ab as *mut _);

                    let mut options_ba = cavc_pline_boolean_o {
                        pline1_aabb_index: std::ptr::null(),
                        pos_equal_eps: f64::NAN,
                        collapsed_area_eps: f64::NAN,
                    };
                    assert_eq!(cavc_pline_boolean_o_init(&mut options_ba), 0);
                    let mut aabb_ba = ptr::null();
                    assert_eq!(cavc_pline_create_approx_aabbindex(clip, &mut aabb_ba), 0);
                    options_ba.pline1_aabb_index = aabb_ba;

                    let (opt_ba_remaining, opt_ba_subtracted) =
                        run_boolean_props_with_options(clip, subject, 2, &options_ba);

                    assert!(
                        props_set_match_ignore_area_sign(
                            &opt_ba_remaining,
                            &default_ba_remaining,
                            1e-4
                        ) && props_set_match_ignore_area_sign(
                            &default_ba_remaining,
                            &opt_ba_remaining,
                            1e-4
                        ),
                        "coincident options BA NOT remaining mismatch for {case_name} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ba_remaining:?}\noptions={opt_ba_remaining:?}"
                    );
                    assert!(
                        props_set_match_ignore_area_sign(
                            &opt_ba_subtracted,
                            &default_ba_subtracted,
                            1e-4
                        ) && props_set_match_ignore_area_sign(
                            &default_ba_subtracted,
                            &opt_ba_subtracted,
                            1e-4
                        ),
                        "coincident options BA NOT subtracted mismatch for {case_name} subject_reversed={subject_reversed} clip_reversed={clip_reversed}\ndefault={default_ba_subtracted:?}\noptions={opt_ba_subtracted:?}"
                    );

                    cavc_aabbindex_f(aabb_ba as *mut _);
                    cavc_pline_f(subject);
                    cavc_pline_f(clip);
                }
            }
        }
    }
}

#[test]
fn pline_boolean_options_path_circle_rectangle_vertex_output_cpp_parity() {
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

        for operation in CPP_CIRCLE_RECT_SOURCE_OPS {
            let (default_remaining, default_subtracted) =
                run_boolean_vertexes(pline_a, pline_b, operation);
            let (opt_remaining, opt_subtracted) =
                run_boolean_vertexes_with_options(pline_a, pline_b, operation, &options);

            assert!(
                vertex_lists_match_unordered(&opt_remaining, &default_remaining, true),
                "boolean options remaining vertex mismatch for op={operation}\ndefault={default_remaining:?}\noptions={opt_remaining:?}"
            );
            assert!(
                vertex_lists_match_unordered(&default_remaining, &opt_remaining, true),
                "boolean options remaining reverse vertex mismatch for op={operation}\ndefault={default_remaining:?}\noptions={opt_remaining:?}"
            );
            assert!(
                vertex_lists_match_unordered(&opt_subtracted, &default_subtracted, true),
                "boolean options subtracted vertex mismatch for op={operation}\ndefault={default_subtracted:?}\noptions={opt_subtracted:?}"
            );
            assert!(
                vertex_lists_match_unordered(&default_subtracted, &opt_subtracted, true),
                "boolean options subtracted reverse vertex mismatch for op={operation}\ndefault={default_subtracted:?}\noptions={opt_subtracted:?}"
            );
        }

        cavc_aabbindex_f(aabb1 as *mut _);
        cavc_pline_f(pline_a);
        cavc_pline_f(pline_b);
    }
}

#[test]
fn pline_boolean_options_path_circle_rectangle_pos_equal_eps_matrix_cpp_parity() {
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

    let mut default_options = cavc_pline_boolean_o {
        pline1_aabb_index: std::ptr::null(),
        pos_equal_eps: f64::NAN,
        collapsed_area_eps: f64::NAN,
    };

    unsafe {
        assert_eq!(cavc_pline_boolean_o_init(&mut default_options), 0);
        let base_pos_equal_eps = default_options.pos_equal_eps;

        let mut aabb1 = ptr::null();
        assert_eq!(cavc_pline_create_approx_aabbindex(pline_a, &mut aabb1), 0);

        for scale in [0.5_f64, 1.0_f64, 2.0_f64] {
            let options = cavc_pline_boolean_o {
                pline1_aabb_index: aabb1,
                pos_equal_eps: base_pos_equal_eps * scale,
                collapsed_area_eps: f64::NAN,
            };

            for operation in CPP_CIRCLE_RECT_SOURCE_OPS {
                let (default_remaining_props, default_subtracted_props) =
                    run_boolean_props(pline_a, pline_b, operation);
                let (opt_remaining_props, opt_subtracted_props) =
                    run_boolean_props_with_options(pline_a, pline_b, operation, &options);

                assert!(
                    props_set_match_ignore_area_sign(
                        &opt_remaining_props,
                        &default_remaining_props,
                        1e-4
                    ) && props_set_match_ignore_area_sign(
                        &default_remaining_props,
                        &opt_remaining_props,
                        1e-4
                    ),
                    "boolean pos_equal_eps matrix remaining props mismatch for scale={scale} op={operation}\ndefault={default_remaining_props:?}\noptions={opt_remaining_props:?}"
                );
                assert!(
                    props_set_match_ignore_area_sign(
                        &opt_subtracted_props,
                        &default_subtracted_props,
                        1e-4
                    ) && props_set_match_ignore_area_sign(
                        &default_subtracted_props,
                        &opt_subtracted_props,
                        1e-4
                    ),
                    "boolean pos_equal_eps matrix subtracted props mismatch for scale={scale} op={operation}\ndefault={default_subtracted_props:?}\noptions={opt_subtracted_props:?}"
                );

                let (default_remaining_v, default_subtracted_v) =
                    run_boolean_vertexes(pline_a, pline_b, operation);
                let (opt_remaining_v, opt_subtracted_v) =
                    run_boolean_vertexes_with_options(pline_a, pline_b, operation, &options);

                assert!(
                    vertex_lists_match_unordered(&opt_remaining_v, &default_remaining_v, true)
                        && vertex_lists_match_unordered(
                            &default_remaining_v,
                            &opt_remaining_v,
                            true
                        ),
                    "boolean pos_equal_eps matrix remaining vertex mismatch for scale={scale} op={operation}\ndefault={default_remaining_v:?}\noptions={opt_remaining_v:?}"
                );
                assert!(
                    vertex_lists_match_unordered(&opt_subtracted_v, &default_subtracted_v, true)
                        && vertex_lists_match_unordered(
                            &default_subtracted_v,
                            &opt_subtracted_v,
                            true
                        ),
                    "boolean pos_equal_eps matrix subtracted vertex mismatch for scale={scale} op={operation}\ndefault={default_subtracted_v:?}\noptions={opt_subtracted_v:?}"
                );
            }
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
    let cases = cpp_coincident_boolean_matrix_cases();

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
    let cases = cpp_coincident_boolean_matrix_cases();

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
fn pline_boolean_options_coincident_matrices_vertex_output_cpp_parity() {
    let cases = cpp_coincident_boolean_matrix_cases();

    for case in cases {
        let pline_a = create_pline(&case.subject, true);
        let pline_b = create_pline(&case.clip, true);
        let (default_remaining, default_subtracted) =
            run_boolean_vertexes(pline_a, pline_b, case.operation);

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
                run_boolean_vertexes_with_options(pline_a, pline_b, case.operation, &options);
            assert!(
                vertex_lists_match_unordered(&opt_remaining, &default_remaining, true),
                "coincident options output remaining vertex mismatch for {}\ndefault={default_remaining:?}\noptions={opt_remaining:?}",
                case.name
            );
            assert!(
                vertex_lists_match_unordered(&default_remaining, &opt_remaining, true),
                "coincident options output remaining reverse vertex mismatch for {}\ndefault={default_remaining:?}\noptions={opt_remaining:?}",
                case.name
            );
            assert!(
                vertex_lists_match_unordered(&opt_subtracted, &default_subtracted, true),
                "coincident options output subtracted vertex mismatch for {}\ndefault={default_subtracted:?}\noptions={opt_subtracted:?}",
                case.name
            );
            assert!(
                vertex_lists_match_unordered(&default_subtracted, &opt_subtracted, true),
                "coincident options output subtracted reverse vertex mismatch for {}\ndefault={default_subtracted:?}\noptions={opt_subtracted:?}",
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

        let mut options = init_parallel_offset_options();

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
fn pline_parallel_offset_options_path_vertex_output_cpp_matrix_parity() {
    for case in cpp_offset_simple_cases()
        .into_iter()
        .chain(cpp_offset_specific_cases())
    {
        let pline = create_pline(&case.input, case.is_closed);
        let default_vertexes = run_parallel_offset_vertexes(pline, case.delta);

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

            let option_vertexes =
                run_parallel_offset_vertexes_with_options(pline, case.delta, &options);
            assert!(
                vertex_lists_match_unordered(&option_vertexes, &default_vertexes, case.is_closed),
                "parallel offset options vertex mismatch for {}\ndefault={default_vertexes:?}\noptions={option_vertexes:?}",
                case.name
            );
            assert!(
                vertex_lists_match_unordered(&default_vertexes, &option_vertexes, case.is_closed),
                "parallel offset options reverse vertex mismatch for {}\ndefault={default_vertexes:?}\noptions={option_vertexes:?}",
                case.name
            );

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_parallel_offset_options_path_tolerance_matrix_cpp_parity() {
    for case in cpp_offset_simple_cases()
        .into_iter()
        .chain(cpp_offset_specific_cases())
    {
        let pline = create_pline(&case.input, case.is_closed);
        let default_props = run_parallel_offset_props(pline, case.delta);
        let default_vertexes = run_parallel_offset_vertexes(pline, case.delta);

        let mut default_options = init_parallel_offset_options();

        unsafe {
            assert_eq!(cavc_pline_parallel_offset_o_init(&mut default_options), 0);
            let base_pos_equal_eps = default_options.pos_equal_eps;
            let base_slice_join_eps = default_options.slice_join_eps;
            let base_offset_dist_eps = default_options.offset_dist_eps;

            let mut aabb_index = ptr::null();
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
                0
            );

            for scale in CPP_TOLERANCE_SCALE_MATRIX {
                let options = cavc_pline_parallel_offset_o {
                    aabb_index,
                    pos_equal_eps: base_pos_equal_eps * scale,
                    slice_join_eps: base_slice_join_eps * scale,
                    offset_dist_eps: base_offset_dist_eps * scale,
                    handle_self_intersects: default_options.handle_self_intersects,
                };

                let option_props =
                    run_parallel_offset_props_with_options(pline, case.delta, &options);
                assert!(
                    props_set_match_ignore_area_sign(&option_props, &default_props, 1e-4)
                        && props_set_match_ignore_area_sign(&default_props, &option_props, 1e-4),
                    "parallel offset tolerance matrix props mismatch for {} scale={scale}\ndefault={default_props:?}\noptions={option_props:?}",
                    case.name
                );

                let option_vertexes =
                    run_parallel_offset_vertexes_with_options(pline, case.delta, &options);
                assert!(
                    vertex_lists_match_unordered(
                        &option_vertexes,
                        &default_vertexes,
                        case.is_closed
                    ) && vertex_lists_match_unordered(
                        &default_vertexes,
                        &option_vertexes,
                        case.is_closed
                    ),
                    "parallel offset tolerance matrix vertex mismatch for {} scale={scale}\ndefault={default_vertexes:?}\noptions={option_vertexes:?}",
                    case.name
                );
            }

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_parallel_offset_options_path_self_intersects_mode_matrix_cpp_parity() {
    for case in cpp_offset_simple_cases() {
        let pline = create_pline(&case.input, case.is_closed);
        let default_props = run_parallel_offset_props(pline, case.delta);
        let default_vertexes = run_parallel_offset_vertexes(pline, case.delta);

        let mut default_options = init_parallel_offset_options();

        unsafe {
            assert_eq!(cavc_pline_parallel_offset_o_init(&mut default_options), 0);

            let mut aabb_index = ptr::null();
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
                0
            );

            for mode in CPP_SELF_INTERSECTS_INCLUDE_MODES {
                let options = cavc_pline_parallel_offset_o {
                    aabb_index,
                    pos_equal_eps: default_options.pos_equal_eps,
                    slice_join_eps: default_options.slice_join_eps,
                    offset_dist_eps: default_options.offset_dist_eps,
                    handle_self_intersects: mode as u8,
                };

                let option_props =
                    run_parallel_offset_props_with_options(pline, case.delta, &options);
                assert!(
                    props_set_match_ignore_area_sign(&option_props, &default_props, 1e-4)
                        && props_set_match_ignore_area_sign(&default_props, &option_props, 1e-4),
                    "parallel offset self-intersects mode props mismatch for {} mode={mode}\ndefault={default_props:?}\noptions={option_props:?}",
                    case.name
                );

                let option_vertexes =
                    run_parallel_offset_vertexes_with_options(pline, case.delta, &options);
                assert!(
                    vertex_lists_match_unordered(
                        &option_vertexes,
                        &default_vertexes,
                        case.is_closed
                    ) && vertex_lists_match_unordered(
                        &default_vertexes,
                        &option_vertexes,
                        case.is_closed
                    ),
                    "parallel offset self-intersects mode vertex mismatch for {} mode={mode}\ndefault={default_vertexes:?}\noptions={option_vertexes:?}",
                    case.name
                );
            }

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_parallel_offset_options_path_self_intersects_mode_stress_matrix_cpp_parity() {
    for case in cpp_offset_simple_cases()
        .into_iter()
        .chain(cpp_offset_specific_cases())
    {
        let pline = create_pline(&case.input, case.is_closed);
        let default_props = run_parallel_offset_props(pline, case.delta);
        let default_vertexes = run_parallel_offset_vertexes(pline, case.delta);

        let mut default_options = init_parallel_offset_options();

        unsafe {
            assert_eq!(cavc_pline_parallel_offset_o_init(&mut default_options), 0);

            let mut aabb_index = ptr::null();
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
                0
            );

            for mode in CPP_SELF_INTERSECTS_INCLUDE_MODES {
                for scale in CPP_TOLERANCE_SCALE_MATRIX {
                    let options = cavc_pline_parallel_offset_o {
                        aabb_index,
                        pos_equal_eps: default_options.pos_equal_eps * scale,
                        slice_join_eps: default_options.slice_join_eps * scale,
                        offset_dist_eps: default_options.offset_dist_eps * scale,
                        handle_self_intersects: mode as u8,
                    };

                    let option_props =
                        run_parallel_offset_props_with_options(pline, case.delta, &options);
                    assert!(
                        props_set_match_ignore_area_sign(&option_props, &default_props, 1e-4)
                            && props_set_match_ignore_area_sign(
                                &default_props,
                                &option_props,
                                1e-4
                            ),
                        "parallel offset self-intersects stress props mismatch for {} mode={} scale={scale}\ndefault={default_props:?}\noptions={option_props:?}",
                        case.name,
                        mode
                    );

                    let option_vertexes =
                        run_parallel_offset_vertexes_with_options(pline, case.delta, &options);
                    assert!(
                        vertex_lists_match_unordered(
                            &option_vertexes,
                            &default_vertexes,
                            case.is_closed
                        ) && vertex_lists_match_unordered(
                            &default_vertexes,
                            &option_vertexes,
                            case.is_closed
                        ),
                        "parallel offset self-intersects stress vertex mismatch for {} mode={} scale={scale}\ndefault={default_vertexes:?}\noptions={option_vertexes:?}",
                        case.name,
                        mode
                    );
                }
            }

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_parallel_offset_options_path_self_intersects_mode_does_not_modify_input_cpp_parity() {
    for case in cpp_offset_simple_cases()
        .into_iter()
        .chain(cpp_offset_specific_cases())
    {
        for mode in CPP_SELF_INTERSECTS_INCLUDE_MODES {
            let pline = create_pline(&case.input, case.is_closed);
            let before = read_vertices(pline);
            let mut options = init_parallel_offset_options();

            unsafe {
                assert_eq!(cavc_pline_parallel_offset_o_init(&mut options), 0);

                let mut aabb_index = ptr::null();
                assert_eq!(
                    cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
                    0
                );

                options.aabb_index = aabb_index;
                options.handle_self_intersects = mode as u8;

                let _ = run_parallel_offset_props_with_options(pline, case.delta, &options);
                let after = read_vertices(pline);
                compare_vertexes(&after, &before);

                cavc_aabbindex_f(aabb_index as *mut _);
                cavc_pline_f(pline);
            }
        }
    }
}

#[test]
fn pline_parallel_offset_options_path_reversed_self_intersects_stress_matrix_cpp_parity() {
    for case in cpp_offset_simple_cases()
        .into_iter()
        .chain(cpp_offset_specific_cases())
    {
        let pline = create_pline(&case.input, case.is_closed);
        unsafe {
            assert_eq!(cavc_pline_invert_direction(pline), 0);
        }
        let delta = -case.delta;
        let default_props = run_parallel_offset_props(pline, delta);
        let default_vertexes = run_parallel_offset_vertexes(pline, delta);

        let mut default_options = init_parallel_offset_options();

        unsafe {
            assert_eq!(cavc_pline_parallel_offset_o_init(&mut default_options), 0);

            let mut aabb_index = ptr::null();
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
                0
            );

            for mode in CPP_SELF_INTERSECTS_INCLUDE_MODES {
                for scale in CPP_TOLERANCE_SCALE_MATRIX {
                    let options = cavc_pline_parallel_offset_o {
                        aabb_index,
                        pos_equal_eps: default_options.pos_equal_eps * scale,
                        slice_join_eps: default_options.slice_join_eps * scale,
                        offset_dist_eps: default_options.offset_dist_eps * scale,
                        handle_self_intersects: mode as u8,
                    };

                    let option_props =
                        run_parallel_offset_props_with_options(pline, delta, &options);
                    assert!(
                        props_set_match_ignore_area_sign(&option_props, &default_props, 1e-4)
                            && props_set_match_ignore_area_sign(
                                &default_props,
                                &option_props,
                                1e-4
                            ),
                        "parallel offset reversed self-intersects stress props mismatch for {} mode={} scale={scale}\ndefault={default_props:?}\noptions={option_props:?}",
                        case.name,
                        mode
                    );

                    let option_vertexes =
                        run_parallel_offset_vertexes_with_options(pline, delta, &options);
                    assert!(
                        vertex_lists_match_unordered(
                            &option_vertexes,
                            &default_vertexes,
                            case.is_closed
                        ) && vertex_lists_match_unordered(
                            &default_vertexes,
                            &option_vertexes,
                            case.is_closed
                        ),
                        "parallel offset reversed self-intersects stress vertex mismatch for {} mode={} scale={scale}\ndefault={default_vertexes:?}\noptions={option_vertexes:?}",
                        case.name,
                        mode
                    );
                }
            }

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_parallel_offset_options_path_self_intersects_stress_output_and_no_modify_cpp_parity() {
    for case in cpp_offset_simple_cases()
        .into_iter()
        .chain(cpp_offset_specific_cases())
    {
        let pline = create_pline(&case.input, case.is_closed);
        let before = read_vertices(pline);
        let default_props = run_parallel_offset_props(pline, case.delta);
        let default_vertexes = run_parallel_offset_vertexes(pline, case.delta);

        let mut default_options = init_parallel_offset_options();

        unsafe {
            assert_eq!(cavc_pline_parallel_offset_o_init(&mut default_options), 0);
            let mut aabb_index = ptr::null();
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
                0
            );

            for mode in CPP_SELF_INTERSECTS_INCLUDE_MODES {
                for scale in CPP_TOLERANCE_SCALE_MATRIX {
                    let options = cavc_pline_parallel_offset_o {
                        aabb_index,
                        pos_equal_eps: default_options.pos_equal_eps * scale,
                        slice_join_eps: default_options.slice_join_eps * scale,
                        offset_dist_eps: default_options.offset_dist_eps * scale,
                        handle_self_intersects: mode as u8,
                    };

                    let option_props =
                        run_parallel_offset_props_with_options(pline, case.delta, &options);
                    assert!(
                        props_set_match_ignore_area_sign(&option_props, &default_props, 1e-4)
                            && props_set_match_ignore_area_sign(
                                &default_props,
                                &option_props,
                                1e-4
                            ),
                        "parallel offset output/no-modify stress props mismatch for {} mode={} scale={scale}\ndefault={default_props:?}\noptions={option_props:?}",
                        case.name,
                        mode
                    );

                    let option_vertexes =
                        run_parallel_offset_vertexes_with_options(pline, case.delta, &options);
                    assert!(
                        vertex_lists_match_unordered(
                            &option_vertexes,
                            &default_vertexes,
                            case.is_closed
                        ) && vertex_lists_match_unordered(
                            &default_vertexes,
                            &option_vertexes,
                            case.is_closed
                        ),
                        "parallel offset output/no-modify stress vertex mismatch for {} mode={} scale={scale}\ndefault={default_vertexes:?}\noptions={option_vertexes:?}",
                        case.name,
                        mode
                    );

                    let after = read_vertices(pline);
                    compare_vertexes(&after, &before);
                }
            }

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_parallel_offset_options_path_reversed_self_intersects_stress_does_not_modify_input_cpp_parity()
 {
    for case in cpp_offset_simple_cases()
        .into_iter()
        .chain(cpp_offset_specific_cases())
    {
        let pline = create_pline(&case.input, case.is_closed);
        unsafe {
            assert_eq!(cavc_pline_invert_direction(pline), 0);
        }
        let delta = -case.delta;
        let before = read_vertices(pline);

        let mut default_options = init_parallel_offset_options();

        unsafe {
            assert_eq!(cavc_pline_parallel_offset_o_init(&mut default_options), 0);
            let mut aabb_index = ptr::null();
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
                0
            );

            for mode in CPP_SELF_INTERSECTS_INCLUDE_MODES {
                for scale in CPP_TOLERANCE_SCALE_MATRIX {
                    let options = cavc_pline_parallel_offset_o {
                        aabb_index,
                        pos_equal_eps: default_options.pos_equal_eps * scale,
                        slice_join_eps: default_options.slice_join_eps * scale,
                        offset_dist_eps: default_options.offset_dist_eps * scale,
                        handle_self_intersects: mode as u8,
                    };

                    let _ = run_parallel_offset_props_with_options(pline, delta, &options);
                    let after = read_vertices(pline);
                    compare_vertexes(&after, &before);
                }
            }

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_parallel_offset_options_path_reversed_self_intersects_stress_output_and_no_modify_cpp_parity()
 {
    for case in cpp_offset_simple_cases()
        .into_iter()
        .chain(cpp_offset_specific_cases())
    {
        let pline = create_pline(&case.input, case.is_closed);
        unsafe {
            assert_eq!(cavc_pline_invert_direction(pline), 0);
        }
        let delta = -case.delta;
        let before = read_vertices(pline);
        let default_props = run_parallel_offset_props(pline, delta);
        let default_vertexes = run_parallel_offset_vertexes(pline, delta);

        let mut default_options = init_parallel_offset_options();

        unsafe {
            assert_eq!(cavc_pline_parallel_offset_o_init(&mut default_options), 0);
            let mut aabb_index = ptr::null();
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
                0
            );

            for mode in CPP_SELF_INTERSECTS_INCLUDE_MODES {
                for scale in CPP_TOLERANCE_SCALE_MATRIX {
                    let options = cavc_pline_parallel_offset_o {
                        aabb_index,
                        pos_equal_eps: default_options.pos_equal_eps * scale,
                        slice_join_eps: default_options.slice_join_eps * scale,
                        offset_dist_eps: default_options.offset_dist_eps * scale,
                        handle_self_intersects: mode as u8,
                    };

                    let option_props =
                        run_parallel_offset_props_with_options(pline, delta, &options);
                    assert!(
                        props_set_match_ignore_area_sign(&option_props, &default_props, 1e-4)
                            && props_set_match_ignore_area_sign(
                                &default_props,
                                &option_props,
                                1e-4
                            ),
                        "parallel offset reversed output/no-modify stress props mismatch for {} mode={} scale={scale}\ndefault={default_props:?}\noptions={option_props:?}",
                        case.name,
                        mode
                    );

                    let option_vertexes =
                        run_parallel_offset_vertexes_with_options(pline, delta, &options);
                    assert!(
                        vertex_lists_match_unordered(
                            &option_vertexes,
                            &default_vertexes,
                            case.is_closed
                        ) && vertex_lists_match_unordered(
                            &default_vertexes,
                            &option_vertexes,
                            case.is_closed
                        ),
                        "parallel offset reversed output/no-modify stress vertex mismatch for {} mode={} scale={scale}\ndefault={default_vertexes:?}\noptions={option_vertexes:?}",
                        case.name,
                        mode
                    );

                    let after = read_vertices(pline);
                    compare_vertexes(&after, &before);
                }
            }

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline);
        }
    }
}

fn cpp_specific_edge_attribution(case_name: &str) -> &'static str {
    match case_name {
        "offset_arc_just_past_line1" => {
            "old C++ specific case: offset arc just past line (float epsilon thresholding)"
        }
        "intersect_ontop_first_vertex" => {
            "old C++ specific case: first vertex ontop of second-segment intersection"
        }
        "collapsed_rectangle" => "old C++ specific case: collapsed rectangle expects empty result",
        "closed_rectangle_inward" => "old C++ simple case: closed rectangle offset inward",
        "closed_rectangle_coincident" => {
            "old C++ simple edge case: closed rectangle offset inward into coincident line"
        }
        "open_rectangle_inward" => "old C++ simple case: open rectangle offset inward",
        "closed_rectangle_outward" => "old C++ simple case: closed rectangle offset outward",
        "open_rectangle_outward" => "old C++ simple case: open rectangle offset outward",
        "closed_diamond_outward" => "old C++ simple case: closed diamond offset outward",
        "closed_diamond_inward" => "old C++ simple case: closed diamond offset inward",
        "open_diamond_inward" => "old C++ simple case: open diamond offset inward",
        "open_diamond_outward" => "old C++ simple case: open diamond offset outward",
        other => panic!("unexpected specific case without attribution: {other}"),
    }
}

fn run_parallel_offset_options_specific_edge_attribution_matrix(
    case: &OffsetCase,
    reverse_input: bool,
) {
    let attribution = cpp_specific_edge_attribution(case.name);
    let pline = create_pline(&case.input, case.is_closed);
    if reverse_input {
        unsafe {
            assert_eq!(cavc_pline_invert_direction(pline), 0);
        }
    }
    let delta = if reverse_input {
        -case.delta
    } else {
        case.delta
    };
    let prefix = if reverse_input { "reversed " } else { "" };
    let before = read_vertices(pline);
    let default_props = run_parallel_offset_props(pline, delta);
    let default_vertexes = run_parallel_offset_vertexes(pline, delta);

    let mut default_options = init_parallel_offset_options();

    unsafe {
        assert_eq!(cavc_pline_parallel_offset_o_init(&mut default_options), 0);
        let mut aabb_index = ptr::null();
        assert_eq!(
            cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
            0
        );

        for mode in CPP_SELF_INTERSECTS_INCLUDE_MODES {
            for scale in CPP_TOLERANCE_SCALE_MATRIX {
                let options = cavc_pline_parallel_offset_o {
                    aabb_index,
                    pos_equal_eps: default_options.pos_equal_eps * scale,
                    slice_join_eps: default_options.slice_join_eps * scale,
                    offset_dist_eps: default_options.offset_dist_eps * scale,
                    handle_self_intersects: mode as u8,
                };

                let option_props = run_parallel_offset_props_with_options(pline, delta, &options);
                assert!(
                    props_set_match_ignore_area_sign(&option_props, &default_props, 1e-4)
                        && props_set_match_ignore_area_sign(&default_props, &option_props, 1e-4),
                    "parallel offset {prefix}specific-edge props mismatch [{}] case={} mode={} scale={scale}\ndefault={default_props:?}\noptions={option_props:?}",
                    attribution,
                    case.name,
                    mode
                );

                let option_vertexes =
                    run_parallel_offset_vertexes_with_options(pline, delta, &options);
                assert!(
                    vertex_lists_match_unordered(
                        &option_vertexes,
                        &default_vertexes,
                        case.is_closed
                    ) && vertex_lists_match_unordered(
                        &default_vertexes,
                        &option_vertexes,
                        case.is_closed
                    ),
                    "parallel offset {prefix}specific-edge vertex mismatch [{}] case={} mode={} scale={scale}\ndefault={default_vertexes:?}\noptions={option_vertexes:?}",
                    attribution,
                    case.name,
                    mode
                );

                let after = read_vertices(pline);
                compare_vertexes(&after, &before);
            }
        }

        cavc_aabbindex_f(aabb_index as *mut _);
        cavc_pline_f(pline);
    }
}

#[test]
fn pline_parallel_offset_options_path_reversed_specific_edge_attribution_matrix_cpp_parity() {
    for case in cpp_offset_specific_edge_matrix_cases() {
        run_parallel_offset_options_specific_edge_attribution_matrix(&case, true);
    }
}

#[test]
fn pline_parallel_offset_options_path_specific_edge_attribution_matrix_cpp_parity() {
    for case in cpp_offset_specific_edge_matrix_cases() {
        run_parallel_offset_options_specific_edge_attribution_matrix(&case, false);
    }
}

#[test]
fn pline_parallel_offset_options_path_does_not_modify_input_cpp_parity() {
    for case in cpp_offset_simple_cases()
        .into_iter()
        .chain(cpp_offset_specific_cases())
    {
        let pline = create_pline(&case.input, case.is_closed);
        let before = read_vertices(pline);

        let mut options = init_parallel_offset_options();

        unsafe {
            assert_eq!(cavc_pline_parallel_offset_o_init(&mut options), 0);
            let mut aabb_index = ptr::null();
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline, &mut aabb_index),
                0
            );
            options.aabb_index = aabb_index;

            let _ = run_parallel_offset_props_with_options(pline, case.delta, &options);
            let after = read_vertices(pline);
            compare_vertexes(&after, &before);

            cavc_aabbindex_f(aabb_index as *mut _);
            cavc_pline_f(pline);
        }
    }
}

#[test]
fn pline_boolean_options_path_circle_rectangle_does_not_modify_input_cpp_parity() {
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

        for operation in CPP_CIRCLE_RECT_SOURCE_OPS {
            let _ = run_boolean_props_with_options(pline_a, pline_b, operation, &options);
            let after_a = read_vertices(pline_a);
            let after_b = read_vertices(pline_b);
            compare_vertexes(&after_a, &before_a);
            compare_vertexes(&after_b, &before_b);
        }

        cavc_aabbindex_f(aabb1 as *mut _);
        cavc_pline_f(pline_a);
        cavc_pline_f(pline_b);
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
fn pline_scan_for_self_intersect_invalid_options_error_ffi() {
    let hourglass = create_pline(
        &[
            (0.0, 2.0, 0.0),
            (1.0, 1.0, 0.0),
            (0.0, 1.0, 0.0),
            (1.0, 2.0, 0.0),
        ],
        true,
    );

    unsafe {
        let mut is_self_intersecting: u8 = 0;
        let mut options = cavc_pline_self_intersect_o {
            pline_aabb_index: ptr::null(),
            pos_equal_eps: f64::NAN,
            include: u32::MAX,
        };
        assert_eq!(cavc_pline_self_intersect_o_init(&mut options), 0);
        options.include = u32::MAX;

        assert_eq!(
            cavc_pline_scan_for_self_intersect(hourglass, &options, &mut is_self_intersecting),
            2
        );
        assert_eq!(
            cavc_pline_scan_for_self_intersect(ptr::null(), &options, &mut is_self_intersecting),
            1
        );

        cavc_pline_f(hourglass);
    }
}

#[test]
fn boolean_and_self_intersect_failure_path_output_stability_ffi() {
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

    unsafe {
        let plinelist_sentinel =
            std::ptr::NonNull::<cavc_plinelist>::dangling().as_ptr() as *const cavc_plinelist;

        let mut pos_plines = plinelist_sentinel;
        let mut neg_plines = plinelist_sentinel;
        assert_eq!(
            cavc_pline_boolean(
                pline_a,
                pline_b,
                u32::MAX,
                ptr::null(),
                &mut pos_plines,
                &mut neg_plines
            ),
            2
        );
        assert_eq!(pos_plines, plinelist_sentinel);
        assert_eq!(neg_plines, plinelist_sentinel);

        assert_eq!(
            cavc_pline_boolean(
                ptr::null(),
                pline_b,
                0,
                ptr::null(),
                &mut pos_plines,
                &mut neg_plines
            ),
            1
        );
        assert_eq!(pos_plines, plinelist_sentinel);
        assert_eq!(neg_plines, plinelist_sentinel);

        let mut boolean_options = cavc_pline_boolean_o {
            pline1_aabb_index: ptr::dangling::<cavc_aabbindex>(),
            pos_equal_eps: f64::NAN,
            collapsed_area_eps: 0.0,
        };
        assert_eq!(cavc_pline_boolean_o_init(&mut boolean_options), 0);
        assert_eq!(
            cavc_pline_boolean(
                ptr::null(),
                pline_b,
                0,
                &boolean_options,
                &mut pos_plines,
                &mut neg_plines
            ),
            1
        );
        assert_eq!(pos_plines, plinelist_sentinel);
        assert_eq!(neg_plines, plinelist_sentinel);

        let mut options = cavc_pline_self_intersect_o {
            pline_aabb_index: ptr::null(),
            pos_equal_eps: f64::NAN,
            include: u32::MAX,
        };
        assert_eq!(cavc_pline_self_intersect_o_init(&mut options), 0);
        options.include = u32::MAX;

        let mut is_self_intersecting = 17_u8;
        assert_eq!(
            cavc_pline_scan_for_self_intersect(pline_a, &options, &mut is_self_intersecting),
            2
        );
        assert_eq!(is_self_intersecting, 17);
        assert_eq!(
            cavc_pline_scan_for_self_intersect(ptr::null(), &options, &mut is_self_intersecting),
            1
        );
        assert_eq!(is_self_intersecting, 17);
        assert_eq!(
            cavc_pline_scan_for_self_intersect(ptr::null(), ptr::null(), &mut is_self_intersecting),
            1
        );
        assert_eq!(is_self_intersecting, 17);

        cavc_pline_f(pline_a);
        cavc_pline_f(pline_b);
    }
}

#[test]
fn pline_parallel_offset_failure_path_output_stability_ffi() {
    unsafe {
        let plinelist_sentinel =
            std::ptr::NonNull::<cavc_plinelist>::dangling().as_ptr() as *const cavc_plinelist;
        let mut results = plinelist_sentinel;

        assert_eq!(
            cavc_pline_parallel_offset(ptr::null(), 2.5, ptr::null(), &mut results),
            1
        );
        assert_eq!(results, plinelist_sentinel);

        let mut options = init_parallel_offset_options();
        assert_eq!(cavc_pline_parallel_offset_o_init(&mut options), 0);
        assert_eq!(
            cavc_pline_parallel_offset(ptr::null(), -3.25, &options, &mut results),
            1
        );
        assert_eq!(results, plinelist_sentinel);
    }
}

#[test]
fn pline_boolean_invalid_operation_error_ffi() {
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

    unsafe {
        let mut pos_plines = ptr::null();
        let mut neg_plines = ptr::null();
        assert_eq!(
            cavc_pline_boolean(
                pline_a,
                pline_b,
                u32::MAX,
                ptr::null(),
                &mut pos_plines,
                &mut neg_plines
            ),
            2
        );
        assert!(pos_plines.is_null());
        assert!(neg_plines.is_null());

        let mut boolean_options = cavc_pline_boolean_o {
            pline1_aabb_index: ptr::dangling::<cavc_aabbindex>(),
            pos_equal_eps: f64::NAN,
            collapsed_area_eps: 0.0,
        };
        assert_eq!(cavc_pline_boolean_o_init(&mut boolean_options), 0);
        let plinelist_sentinel =
            std::ptr::NonNull::<cavc_plinelist>::dangling().as_ptr() as *const cavc_plinelist;
        pos_plines = plinelist_sentinel;
        neg_plines = plinelist_sentinel;
        assert_eq!(
            cavc_pline_boolean(
                pline_a,
                pline_b,
                u32::MAX,
                &boolean_options,
                &mut pos_plines,
                &mut neg_plines
            ),
            2
        );
        assert_eq!(pos_plines, plinelist_sentinel);
        assert_eq!(neg_plines, plinelist_sentinel);

        assert_eq!(
            cavc_pline_boolean(
                ptr::null(),
                pline_b,
                0,
                ptr::null(),
                &mut pos_plines,
                &mut neg_plines
            ),
            1
        );

        cavc_pline_f(pline_a);
        cavc_pline_f(pline_b);
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

#[test]
fn pline_contains_invalid_input_result_contract_ffi() {
    let rectangle = create_pline(
        &[
            (-10.0, -10.0, 0.0),
            (10.0, -10.0, 0.0),
            (10.0, 10.0, 0.0),
            (-10.0, 10.0, 0.0),
        ],
        true,
    );

    unsafe {
        let mut result = CAVC_CONTAINS_RESULT_INTERSECTED;
        assert_eq!(
            cavc_pline_contains(ptr::null(), rectangle, ptr::null(), &mut result),
            1
        );
        assert_eq!(result, CAVC_CONTAINS_RESULT_INVALID_INPUT);

        assert_eq!(
            cavc_pline_contains(rectangle, ptr::null(), ptr::null(), &mut result),
            1
        );
        assert_eq!(result, CAVC_CONTAINS_RESULT_INVALID_INPUT);

        assert_eq!(
            cavc_pline_contains(ptr::null(), rectangle, ptr::null(), ptr::null_mut()),
            1
        );

        let mut contains_options = cavc_pline_contains_o {
            pline1_aabb_index: ptr::dangling::<cavc_aabbindex>(),
            pos_equal_eps: f64::NAN,
        };
        assert_eq!(cavc_pline_contains_o_init(&mut contains_options), 0);

        result = CAVC_CONTAINS_RESULT_INTERSECTED;
        assert_eq!(
            cavc_pline_contains(ptr::null(), rectangle, &contains_options, &mut result),
            1
        );
        assert_eq!(result, CAVC_CONTAINS_RESULT_INVALID_INPUT);

        result = CAVC_CONTAINS_RESULT_INTERSECTED;
        assert_eq!(
            cavc_pline_contains(rectangle, ptr::null(), &contains_options, &mut result),
            1
        );
        assert_eq!(result, CAVC_CONTAINS_RESULT_INVALID_INPUT);
        assert_eq!(
            cavc_pline_contains(ptr::null(), rectangle, &contains_options, ptr::null_mut()),
            1
        );
        assert_eq!(
            cavc_pline_contains(rectangle, ptr::null(), &contains_options, ptr::null_mut()),
            1
        );

        cavc_pline_f(rectangle);
    }
}

#[test]
fn aabbindex_extents_cpp_parity() {
    // old C++ source: TEST_staticspatialindex.cpp -> StaticSpatialIndexTests.index,
    // StaticSpatialIndexTests.skip_sorting_small_index (extents assertions)
    struct AabbExtentsCase {
        source_case: &'static str,
        input: Vec<(f64, f64, f64)>,
        expected: (f64, f64, f64, f64),
    }

    let cases = vec![
        AabbExtentsCase {
            source_case: "StaticSpatialIndexTests.index",
            input: vec![
                (0.0, 1.0, 0.0),
                (96.0, 1.0, 0.0),
                (96.0, 95.0, 0.0),
                (0.0, 95.0, 0.0),
            ],
            expected: (0.0, 1.0, 96.0, 95.0),
        },
        AabbExtentsCase {
            source_case: "StaticSpatialIndexTests.skip_sorting_small_index",
            input: vec![
                (0.0, 2.0, 0.0),
                (96.0, 2.0, 0.0),
                (96.0, 93.0, 0.0),
                (0.0, 93.0, 0.0),
            ],
            expected: (0.0, 2.0, 96.0, 93.0),
        },
    ];

    let mut covered_source_cases = Vec::new();
    for case in cases {
        let pline = create_pline(&case.input, true);
        let mut approx_index = ptr::null();
        let mut exact_index = ptr::null();

        unsafe {
            assert_eq!(
                cavc_pline_create_approx_aabbindex(pline, &mut approx_index),
                0
            );
            assert_eq!(cavc_pline_create_aabbindex(pline, &mut exact_index), 0);
        }

        let approx_extents = read_aabbindex_extents(approx_index);
        let exact_extents = read_aabbindex_extents(exact_index);

        assert_fuzzy_eq!(approx_extents.0, case.expected.0);
        assert_fuzzy_eq!(approx_extents.1, case.expected.1);
        assert_fuzzy_eq!(approx_extents.2, case.expected.2);
        assert_fuzzy_eq!(approx_extents.3, case.expected.3);

        assert_fuzzy_eq!(exact_extents.0, case.expected.0);
        assert_fuzzy_eq!(exact_extents.1, case.expected.1);
        assert_fuzzy_eq!(exact_extents.2, case.expected.2);
        assert_fuzzy_eq!(exact_extents.3, case.expected.3);

        assert_fuzzy_eq!(approx_extents.0, exact_extents.0);
        assert_fuzzy_eq!(approx_extents.1, exact_extents.1);
        assert_fuzzy_eq!(approx_extents.2, exact_extents.2);
        assert_fuzzy_eq!(approx_extents.3, exact_extents.3);

        unsafe {
            cavc_aabbindex_f(approx_index as *mut _);
            cavc_aabbindex_f(exact_index as *mut _);
            cavc_pline_f(pline);
        }

        covered_source_cases.push(case.source_case);
    }

    let mut min_x = f64::NAN;
    let mut min_y = f64::NAN;
    let mut max_x = f64::NAN;
    let mut max_y = f64::NAN;
    unsafe {
        assert_eq!(
            cavc_aabbindex_get_extents(ptr::null(), &mut min_x, &mut min_y, &mut max_x, &mut max_y),
            1
        );

        let mut null_index = ptr::null();
        assert_eq!(cavc_pline_create_aabbindex(ptr::null(), &mut null_index), 1);
        assert!(null_index.is_null());
    }

    assert_source_case_coverage(
        &covered_source_cases,
        &CPP_AABBINDEX_EXTENTS_SOURCE_CASES,
        "aabbindex extents cpp parity",
    );
}

#[test]
fn aabbindex_extents_empty_index_nan_ffi() {
    let empty_pline = create_pline(&[], true);
    let mut approx_index = ptr::null();
    let mut exact_index = ptr::null();

    unsafe {
        assert_eq!(
            cavc_pline_create_approx_aabbindex(empty_pline, &mut approx_index),
            0
        );
        assert_eq!(
            cavc_pline_create_aabbindex(empty_pline, &mut exact_index),
            0
        );
    }

    let approx_extents = read_aabbindex_extents(approx_index);
    let exact_extents = read_aabbindex_extents(exact_index);

    assert!(approx_extents.0.is_nan());
    assert!(approx_extents.1.is_nan());
    assert!(approx_extents.2.is_nan());
    assert!(approx_extents.3.is_nan());
    assert!(exact_extents.0.is_nan());
    assert!(exact_extents.1.is_nan());
    assert!(exact_extents.2.is_nan());
    assert!(exact_extents.3.is_nan());

    unsafe {
        cavc_aabbindex_f(approx_index as *mut _);
        cavc_aabbindex_f(exact_index as *mut _);
        cavc_pline_f(empty_pline);
    }
}

#[test]
fn aabbindex_failure_path_output_stability_ffi() {
    unsafe {
        let aabb_sentinel =
            std::ptr::NonNull::<cavc_aabbindex>::dangling().as_ptr() as *const cavc_aabbindex;

        let mut approx_index = aabb_sentinel;
        assert_eq!(
            cavc_pline_create_approx_aabbindex(ptr::null(), &mut approx_index),
            1
        );
        assert_eq!(approx_index, aabb_sentinel);

        let mut exact_index = aabb_sentinel;
        assert_eq!(
            cavc_pline_create_aabbindex(ptr::null(), &mut exact_index),
            1
        );
        assert_eq!(exact_index, aabb_sentinel);

        let (mut min_x, mut min_y, mut max_x, mut max_y) = (1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64);
        assert_eq!(
            cavc_aabbindex_get_extents(ptr::null(), &mut min_x, &mut min_y, &mut max_x, &mut max_y),
            1
        );
        assert_fuzzy_eq!(min_x, 1.0);
        assert_fuzzy_eq!(min_y, 2.0);
        assert_fuzzy_eq!(max_x, 3.0);
        assert_fuzzy_eq!(max_y, 4.0);
    }
}

#[test]
fn ffi_options_create_init_lifecycle_parity() {
    unsafe {
        // parallel_offset options: create/free parity with init defaults
        let mut parallel_created_ptr: *mut cavc_pline_parallel_offset_o = ptr::null_mut();
        assert_eq!(
            cavc_pline_parallel_offset_o_create(&mut parallel_created_ptr),
            0
        );
        assert!(!parallel_created_ptr.is_null());
        let parallel_created = *parallel_created_ptr;

        let mut parallel_inited = cavc_pline_parallel_offset_o {
            aabb_index: ptr::dangling::<cavc_aabbindex>(),
            pos_equal_eps: f64::NAN,
            slice_join_eps: f64::NAN,
            offset_dist_eps: f64::NAN,
            handle_self_intersects: 255,
        };
        assert_eq!(cavc_pline_parallel_offset_o_init(&mut parallel_inited), 0);
        assert!(parallel_created.aabb_index.is_null());
        assert!(parallel_inited.aabb_index.is_null());
        assert_fuzzy_eq!(
            parallel_created.pos_equal_eps,
            parallel_inited.pos_equal_eps
        );
        assert_fuzzy_eq!(
            parallel_created.slice_join_eps,
            parallel_inited.slice_join_eps
        );
        assert_fuzzy_eq!(
            parallel_created.offset_dist_eps,
            parallel_inited.offset_dist_eps
        );
        assert_eq!(
            parallel_created.handle_self_intersects,
            parallel_inited.handle_self_intersects
        );
        cavc_pline_parallel_offset_o_f(ptr::null_mut());
        cavc_pline_parallel_offset_o_f(parallel_created_ptr);

        // boolean options: create/free parity with init defaults
        let mut boolean_created_ptr: *mut cavc_pline_boolean_o = ptr::null_mut();
        assert_eq!(cavc_pline_boolean_o_create(&mut boolean_created_ptr), 0);
        assert!(!boolean_created_ptr.is_null());
        let boolean_created = *boolean_created_ptr;

        let mut boolean_inited = cavc_pline_boolean_o {
            pline1_aabb_index: ptr::dangling::<cavc_aabbindex>(),
            pos_equal_eps: f64::NAN,
            collapsed_area_eps: 0.0,
        };
        assert_eq!(cavc_pline_boolean_o_init(&mut boolean_inited), 0);
        assert!(boolean_created.pline1_aabb_index.is_null());
        assert!(boolean_inited.pline1_aabb_index.is_null());
        assert_fuzzy_eq!(boolean_created.pos_equal_eps, boolean_inited.pos_equal_eps);
        assert!(boolean_created.collapsed_area_eps.is_nan());
        assert!(boolean_inited.collapsed_area_eps.is_nan());
        cavc_pline_boolean_o_f(ptr::null_mut());
        cavc_pline_boolean_o_f(boolean_created_ptr);

        // self_intersect init: null error + default write
        let mut self_intersect_inited = cavc_pline_self_intersect_o {
            pline_aabb_index: ptr::dangling::<cavc_aabbindex>(),
            pos_equal_eps: f64::NAN,
            include: u32::MAX,
        };
        assert_eq!(cavc_pline_self_intersect_o_init(ptr::null_mut()), 1);
        assert_eq!(
            cavc_pline_self_intersect_o_init(&mut self_intersect_inited),
            0
        );
        assert!(self_intersect_inited.pline_aabb_index.is_null());
        assert!(self_intersect_inited.pos_equal_eps.is_finite());
        assert!(self_intersect_inited.include <= CAVC_SELF_INTERSECTS_INCLUDE_GLOBAL);

        // contains init: null error + default write
        let mut contains_inited = cavc_pline_contains_o {
            pline1_aabb_index: ptr::dangling::<cavc_aabbindex>(),
            pos_equal_eps: f64::NAN,
        };
        assert_eq!(cavc_pline_contains_o_init(ptr::null_mut()), 1);
        assert_eq!(cavc_pline_contains_o_init(&mut contains_inited), 0);
        assert!(contains_inited.pline1_aabb_index.is_null());
        assert!(contains_inited.pos_equal_eps.is_finite());

        // shape offset init: null error + default write
        let mut shape_offset_inited = cavc_shape_offset_o {
            pos_equal_eps: f64::NAN,
            offset_dist_eps: f64::NAN,
            slice_join_eps: f64::NAN,
        };
        assert_eq!(cavc_shape_offset_o_init(ptr::null_mut()), 1);
        assert_eq!(cavc_shape_offset_o_init(&mut shape_offset_inited), 0);
        assert!(shape_offset_inited.pos_equal_eps.is_finite());
        assert!(shape_offset_inited.offset_dist_eps.is_finite());
        assert!(shape_offset_inited.slice_join_eps.is_finite());
    }
}

#[test]
fn shape_set_cw_pline_userdata_values_ffi() {
    let outer = create_pline(
        &[
            (-200.0, 200.0, 0.0),
            (-200.0, -200.0, 0.0),
            (200.0, -200.0, 0.0),
            (200.0, 200.0, 0.0),
        ],
        true,
    );
    let inner = create_pline(
        &[
            (-100.0, 0.0, 0.0),
            (0.0, 100.0, 0.0),
            (100.0, 0.0, 0.0),
            (0.0, -100.0, 0.0),
        ],
        true,
    );

    unsafe {
        let mut list = ptr::null_mut();
        assert_eq!(cavc_plinelist_create(0, &mut list), 0);
        assert_eq!(cavc_plinelist_push(list, outer), 0);
        assert_eq!(cavc_plinelist_push(list, inner), 0);

        let mut shape = ptr::null_mut();
        assert_eq!(cavc_shape_create(list, &mut shape), 0);
        cavc_plinelist_f(list);

        let payload = [11_u64, 22_u64, 33_u64];
        let mut cw_count = 0_u32;
        assert_eq!(cavc_shape_get_cw_count(shape, &mut cw_count), 0);
        assert_eq!(cw_count, 1);

        assert_eq!(
            cavc_shape_set_cw_pline_userdata_values(ptr::null_mut(), 0, payload.as_ptr(), 3),
            1
        );
        assert_eq!(
            cavc_shape_set_cw_pline_userdata_values(shape, 99, payload.as_ptr(), 3),
            2
        );

        assert_eq!(
            cavc_shape_set_cw_pline_userdata_values(shape, 0, payload.as_ptr(), 3),
            0
        );
        let mut count = 0_u32;
        assert_eq!(
            cavc_shape_get_cw_pline_userdata_count(shape, 0, &mut count),
            0
        );
        assert_eq!(count, 3);

        let mut out = [0_u64; 3];
        assert_eq!(
            cavc_shape_get_cw_pline_userdata_values(shape, 0, out.as_mut_ptr()),
            0
        );
        assert_eq!(out, payload);

        assert_eq!(
            cavc_shape_get_cw_pline_userdata_values(ptr::null(), 0, out.as_mut_ptr()),
            1
        );
        assert_eq!(
            cavc_shape_get_cw_pline_userdata_values(shape, 99, out.as_mut_ptr()),
            2
        );

        // null payload with non-zero count still clears (by API implementation contract)
        assert_eq!(
            cavc_shape_set_cw_pline_userdata_values(shape, 0, ptr::null(), 9),
            0
        );
        assert_eq!(
            cavc_shape_get_cw_pline_userdata_count(shape, 0, &mut count),
            0
        );
        assert_eq!(count, 0);

        cavc_shape_f(shape);
    }
}

#[test]
fn shape_set_ccw_pline_userdata_values_ffi() {
    let outer = create_pline(
        &[
            (-200.0, 200.0, 0.0),
            (-200.0, -200.0, 0.0),
            (200.0, -200.0, 0.0),
            (200.0, 200.0, 0.0),
        ],
        true,
    );
    let inner = create_pline(
        &[
            (-100.0, 0.0, 0.0),
            (0.0, 100.0, 0.0),
            (100.0, 0.0, 0.0),
            (0.0, -100.0, 0.0),
        ],
        true,
    );

    unsafe {
        let mut list = ptr::null_mut();
        assert_eq!(cavc_plinelist_create(0, &mut list), 0);
        assert_eq!(cavc_plinelist_push(list, outer), 0);
        assert_eq!(cavc_plinelist_push(list, inner), 0);

        let mut shape = ptr::null_mut();
        assert_eq!(cavc_shape_create(list, &mut shape), 0);
        cavc_plinelist_f(list);

        let payload = [101_u64, 202_u64];
        let mut ccw_count = 0_u32;
        assert_eq!(cavc_shape_get_ccw_count(shape, &mut ccw_count), 0);
        assert_eq!(ccw_count, 1);

        assert_eq!(
            cavc_shape_set_ccw_pline_userdata_values(ptr::null_mut(), 0, payload.as_ptr(), 2),
            1
        );
        assert_eq!(
            cavc_shape_set_ccw_pline_userdata_values(shape, 99, payload.as_ptr(), 2),
            2
        );

        assert_eq!(
            cavc_shape_set_ccw_pline_userdata_values(shape, 0, payload.as_ptr(), 2),
            0
        );
        let mut count = 0_u32;
        assert_eq!(
            cavc_shape_get_ccw_pline_userdata_count(shape, 0, &mut count),
            0
        );
        assert_eq!(count, 2);

        let mut out = [0_u64; 2];
        assert_eq!(
            cavc_shape_get_ccw_pline_userdata_values(shape, 0, out.as_mut_ptr()),
            0
        );
        assert_eq!(out, payload);

        assert_eq!(
            cavc_shape_get_ccw_pline_userdata_values(ptr::null(), 0, out.as_mut_ptr()),
            1
        );
        assert_eq!(
            cavc_shape_get_ccw_pline_userdata_values(shape, 99, out.as_mut_ptr()),
            2
        );

        assert_eq!(
            cavc_shape_set_ccw_pline_userdata_values(shape, 0, ptr::null(), 5),
            0
        );
        assert_eq!(
            cavc_shape_get_ccw_pline_userdata_count(shape, 0, &mut count),
            0
        );
        assert_eq!(count, 0);

        cavc_shape_f(shape);
    }
}

#[test]
fn shape_userdata_getter_failure_path_output_stability_ffi() {
    let outer = create_pline(
        &[
            (-200.0, 200.0, 0.0),
            (-200.0, -200.0, 0.0),
            (200.0, -200.0, 0.0),
            (200.0, 200.0, 0.0),
        ],
        true,
    );
    let inner = create_pline(
        &[
            (-100.0, 0.0, 0.0),
            (0.0, 100.0, 0.0),
            (100.0, 0.0, 0.0),
            (0.0, -100.0, 0.0),
        ],
        true,
    );

    unsafe {
        let mut list = ptr::null_mut();
        assert_eq!(cavc_plinelist_create(0, &mut list), 0);
        assert_eq!(cavc_plinelist_push(list, outer), 0);
        assert_eq!(cavc_plinelist_push(list, inner), 0);

        let mut shape = ptr::null_mut();
        assert_eq!(cavc_shape_create(list, &mut shape), 0);
        cavc_plinelist_f(list);

        let ccw_payload = [101_u64, 202_u64];
        let cw_payload = [11_u64, 22_u64, 33_u64];
        assert_eq!(
            cavc_shape_set_ccw_pline_userdata_values(shape, 0, ccw_payload.as_ptr(), 2),
            0
        );
        assert_eq!(
            cavc_shape_set_cw_pline_userdata_values(shape, 0, cw_payload.as_ptr(), 3),
            0
        );

        let mut ccw_count = 777_u32;
        assert_eq!(
            cavc_shape_get_ccw_pline_userdata_count(ptr::null(), 0, &mut ccw_count),
            1
        );
        assert_eq!(ccw_count, 777);
        assert_eq!(
            cavc_shape_get_ccw_pline_userdata_count(shape, 99, &mut ccw_count),
            2
        );
        assert_eq!(ccw_count, 777);

        let mut cw_count = 888_u32;
        assert_eq!(
            cavc_shape_get_cw_pline_userdata_count(ptr::null(), 0, &mut cw_count),
            1
        );
        assert_eq!(cw_count, 888);
        assert_eq!(
            cavc_shape_get_cw_pline_userdata_count(shape, 99, &mut cw_count),
            2
        );
        assert_eq!(cw_count, 888);

        let mut ccw_userdata = [501_u64, 502_u64];
        assert_eq!(
            cavc_shape_get_ccw_pline_userdata_values(ptr::null(), 0, ccw_userdata.as_mut_ptr()),
            1
        );
        assert_eq!(ccw_userdata, [501, 502]);
        assert_eq!(
            cavc_shape_get_ccw_pline_userdata_values(shape, 99, ccw_userdata.as_mut_ptr()),
            2
        );
        assert_eq!(ccw_userdata, [501, 502]);

        let mut cw_userdata = [601_u64, 602_u64, 603_u64];
        assert_eq!(
            cavc_shape_get_cw_pline_userdata_values(ptr::null(), 0, cw_userdata.as_mut_ptr()),
            1
        );
        assert_eq!(cw_userdata, [601, 602, 603]);
        assert_eq!(
            cavc_shape_get_cw_pline_userdata_values(shape, 99, cw_userdata.as_mut_ptr()),
            2
        );
        assert_eq!(cw_userdata, [601, 602, 603]);

        cavc_shape_f(shape);
    }
}

#[test]
fn shape_polyline_access_error_contracts_ffi() {
    let outer = create_pline(
        &[
            (-200.0, 200.0, 0.0),
            (-200.0, -200.0, 0.0),
            (200.0, -200.0, 0.0),
            (200.0, 200.0, 0.0),
        ],
        true,
    );
    let inner = create_pline(
        &[
            (-100.0, 0.0, 0.0),
            (0.0, 100.0, 0.0),
            (100.0, 0.0, 0.0),
            (0.0, -100.0, 0.0),
        ],
        true,
    );

    unsafe {
        let mut list = ptr::null_mut();
        assert_eq!(cavc_plinelist_create(0, &mut list), 0);
        assert_eq!(cavc_plinelist_push(list, outer), 0);
        assert_eq!(cavc_plinelist_push(list, inner), 0);

        let mut shape = ptr::null_mut();
        assert_eq!(cavc_shape_create(list, &mut shape), 0);
        cavc_plinelist_f(list);

        let mut count = 123_u32;
        assert_eq!(
            cavc_shape_get_ccw_polyline_count(ptr::null(), 0, &mut count),
            1
        );
        assert_eq!(count, 123);
        assert_eq!(cavc_shape_get_ccw_polyline_count(shape, 99, &mut count), 2);
        assert_eq!(count, 123);

        count = 456;
        assert_eq!(
            cavc_shape_get_cw_polyline_count(ptr::null(), 0, &mut count),
            1
        );
        assert_eq!(count, 456);
        assert_eq!(cavc_shape_get_cw_polyline_count(shape, 99, &mut count), 2);
        assert_eq!(count, 456);

        let mut is_closed = 7_u8;
        assert_eq!(
            cavc_shape_get_ccw_polyline_is_closed(ptr::null(), 0, &mut is_closed),
            1
        );
        assert_eq!(is_closed, 7);
        assert_eq!(
            cavc_shape_get_ccw_polyline_is_closed(shape, 99, &mut is_closed),
            2
        );
        assert_eq!(is_closed, 7);

        is_closed = 9;
        assert_eq!(
            cavc_shape_get_cw_polyline_is_closed(ptr::null(), 0, &mut is_closed),
            1
        );
        assert_eq!(is_closed, 9);
        assert_eq!(
            cavc_shape_get_cw_polyline_is_closed(shape, 99, &mut is_closed),
            2
        );
        assert_eq!(is_closed, 9);

        let mut ccw_vertex_data = [cavc_vertex::new(11.0, 22.0, 33.0); 2];
        assert_eq!(
            cavc_shape_get_ccw_polyline_vertex_data(ptr::null(), 0, ccw_vertex_data.as_mut_ptr()),
            1
        );
        assert_fuzzy_eq!(ccw_vertex_data[0].x, 11.0);
        assert_fuzzy_eq!(ccw_vertex_data[0].y, 22.0);
        assert_fuzzy_eq!(ccw_vertex_data[0].bulge, 33.0);
        assert_eq!(
            cavc_shape_get_ccw_polyline_vertex_data(shape, 99, ccw_vertex_data.as_mut_ptr()),
            2
        );
        assert_fuzzy_eq!(ccw_vertex_data[0].x, 11.0);
        assert_fuzzy_eq!(ccw_vertex_data[0].y, 22.0);
        assert_fuzzy_eq!(ccw_vertex_data[0].bulge, 33.0);

        let mut cw_vertex_data = [cavc_vertex::new(44.0, 55.0, 66.0); 2];
        assert_eq!(
            cavc_shape_get_cw_polyline_vertex_data(ptr::null(), 0, cw_vertex_data.as_mut_ptr()),
            1
        );
        assert_fuzzy_eq!(cw_vertex_data[0].x, 44.0);
        assert_fuzzy_eq!(cw_vertex_data[0].y, 55.0);
        assert_fuzzy_eq!(cw_vertex_data[0].bulge, 66.0);
        assert_eq!(
            cavc_shape_get_cw_polyline_vertex_data(shape, 99, cw_vertex_data.as_mut_ptr()),
            2
        );
        assert_fuzzy_eq!(cw_vertex_data[0].x, 44.0);
        assert_fuzzy_eq!(cw_vertex_data[0].y, 55.0);
        assert_fuzzy_eq!(cw_vertex_data[0].bulge, 66.0);

        cavc_shape_f(shape);
    }
}

#[test]
fn shape_root_invalid_input_contracts_ffi() {
    unsafe {
        let shape_sentinel = std::ptr::NonNull::<cavc_shape>::dangling().as_ptr();

        let mut created_shape = shape_sentinel;
        assert_eq!(cavc_shape_create(ptr::null(), &mut created_shape), 1);
        assert_eq!(created_shape, shape_sentinel);

        let mut offset_shape = shape_sentinel;
        assert_eq!(
            cavc_shape_parallel_offset(ptr::null(), 12.0, ptr::null(), &mut offset_shape),
            1
        );
        assert_eq!(offset_shape, shape_sentinel);

        let mut ccw_count = 101_u32;
        assert_eq!(cavc_shape_get_ccw_count(ptr::null(), &mut ccw_count), 1);
        assert_eq!(ccw_count, 101);

        let mut cw_count = 202_u32;
        assert_eq!(cavc_shape_get_cw_count(ptr::null(), &mut cw_count), 1);
        assert_eq!(cw_count, 202);
    }
}

#[test]
fn shape_parallel_offset_failure_path_output_stability_ffi() {
    unsafe {
        let shape_sentinel = std::ptr::NonNull::<cavc_shape>::dangling().as_ptr();
        let mut offset_shape = shape_sentinel;

        assert_eq!(
            cavc_shape_parallel_offset(ptr::null(), 12.0, ptr::null(), &mut offset_shape),
            1
        );
        assert_eq!(offset_shape, shape_sentinel);

        let mut options = cavc_shape_offset_o {
            pos_equal_eps: f64::NAN,
            offset_dist_eps: f64::NAN,
            slice_join_eps: f64::NAN,
        };
        assert_eq!(cavc_shape_offset_o_init(&mut options), 0);

        assert_eq!(
            cavc_shape_parallel_offset(ptr::null(), -8.0, &options, &mut offset_shape),
            1
        );
        assert_eq!(offset_shape, shape_sentinel);
    }
}

#[test]
fn plinelist_failure_path_output_stability_ffi() {
    unsafe {
        let mut count = 313_u32;
        assert_eq!(cavc_plinelist_get_count(ptr::null(), &mut count), 1);
        assert_eq!(count, 313);

        let pline_sentinel =
            std::ptr::NonNull::<cavc_pline>::dangling().as_ptr() as *const cavc_pline;
        let mut out_pline = pline_sentinel;

        assert_eq!(cavc_plinelist_get_pline(ptr::null(), 0, &mut out_pline), 1);
        assert_eq!(out_pline, pline_sentinel);

        assert_eq!(cavc_plinelist_pop(ptr::null_mut(), &mut out_pline), 1);
        assert_eq!(out_pline, pline_sentinel);

        assert_eq!(cavc_plinelist_take(ptr::null_mut(), 0, &mut out_pline), 1);
        assert_eq!(out_pline, pline_sentinel);

        let mut empty_list = ptr::null_mut();
        assert_eq!(cavc_plinelist_create(0, &mut empty_list), 0);
        assert_eq!(cavc_plinelist_get_pline(empty_list, 0, &mut out_pline), 2);
        assert_eq!(out_pline, pline_sentinel);
        assert_eq!(cavc_plinelist_pop(empty_list as *mut _, &mut out_pline), 2);
        assert_eq!(out_pline, pline_sentinel);
        assert_eq!(
            cavc_plinelist_take(empty_list as *mut _, 0, &mut out_pline),
            2
        );
        assert_eq!(out_pline, pline_sentinel);
        cavc_plinelist_f(empty_list);

        let mut list = ptr::null_mut();
        assert_eq!(cavc_plinelist_create(0, &mut list), 0);
        let pline = create_pline(&[(0.0, 0.0, 0.0), (2.0, 0.0, 0.0)], false);
        assert_eq!(cavc_plinelist_push(list, pline), 0);
        assert_eq!(cavc_plinelist_get_pline(list, 99, &mut out_pline), 2);
        assert_eq!(out_pline, pline_sentinel);
        assert_eq!(cavc_plinelist_take(list as *mut _, 99, &mut out_pline), 2);
        assert_eq!(out_pline, pline_sentinel);
        cavc_plinelist_f(list);
    }
}
