use cavalier_contours::core::math::Vector2;
use cavalier_contours::polyline::{
    BooleanOp, PlineCreation, PlineSource, PlineSourceMut, Polyline, seg_fast_approx_bounding_box,
};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::{
    f64::consts::{PI, TAU},
    hint::black_box,
    time::Duration,
};

const ARCS_TO_LINES_ERROR: f64 = 0.01;
const PATHOLOGICAL_SEGMENT_COUNTS: [usize; 4] = [10, 25, 50, 100];

#[derive(Clone)]
struct BenchmarkProfile {
    id: String,
    offset_count: usize,
    offset_delta: f64,
    pline: Polyline<f64>,
}

fn closed_polyline(vertices: &[(f64, f64, f64)]) -> Polyline<f64> {
    let mut pline = Polyline::<f64>::with_capacity(vertices.len(), true);
    for &(x, y, bulge) in vertices {
        pline.add(x, y, bulge);
    }
    pline
}

fn profile(
    id: impl Into<String>,
    offset_count: usize,
    offset_delta: f64,
    pline: Polyline<f64>,
) -> BenchmarkProfile {
    BenchmarkProfile {
        id: id.into(),
        offset_count,
        offset_delta,
        pline,
    }
}

fn square() -> BenchmarkProfile {
    profile(
        "square",
        30,
        1.0,
        closed_polyline(&[
            (-15.0, -15.0, 0.0),
            (25.0, -15.0, 0.0),
            (25.0, 25.0, 0.0),
            (-15.0, 25.0, 0.0),
        ]),
    )
}

fn diamond() -> BenchmarkProfile {
    profile(
        "diamond",
        30,
        1.0,
        closed_polyline(&[
            (-15.0, 5.0, 0.0),
            (5.0, -15.0, 0.0),
            (25.0, 5.0, 0.0),
            (5.0, 25.0, 0.0),
        ]),
    )
}

fn circle() -> BenchmarkProfile {
    profile(
        "circle",
        30,
        1.0,
        closed_polyline(&[(-35.0, 5.0, 1.0), (45.0, 5.0, 1.0)]),
    )
}

fn rounded_rectangle() -> BenchmarkProfile {
    let center_x = 5.0;
    let center_y = 5.0;
    let total_width = 40.0;
    let total_height = 20.0;
    let corner_radius = 5.0;
    let width = total_width - 2.0 * corner_radius;
    let height = total_height - 2.0 * corner_radius;
    let bulge = (PI / 8.0).tan();

    profile(
        "rounded_rectangle",
        30,
        0.5,
        closed_polyline(&[
            (center_x - width / 2.0, center_y - total_height / 2.0, 0.0),
            (center_x + width / 2.0, center_y - total_height / 2.0, bulge),
            (center_x + total_width / 2.0, center_y - height / 2.0, 0.0),
            (center_x + total_width / 2.0, center_y + height / 2.0, bulge),
            (center_x + width / 2.0, center_y + total_height / 2.0, 0.0),
            (center_x - width / 2.0, center_y + total_height / 2.0, bulge),
            (center_x - total_width / 2.0, center_y + height / 2.0, 0.0),
            (center_x - total_width / 2.0, center_y - height / 2.0, bulge),
        ]),
    )
}

fn profile1() -> BenchmarkProfile {
    profile(
        "profile1",
        40,
        0.1,
        closed_polyline(&[
            (0.0, 0.0, 0.0),
            (2.0, 0.0, 1.0),
            (10.0, 0.0, -0.5),
            (10.0, 10.0, 0.5),
            (14.0, 20.0, -0.5),
            (0.0, 20.0, 0.0),
        ]),
    )
}

fn profile2() -> BenchmarkProfile {
    profile(
        "profile2",
        40,
        0.1,
        closed_polyline(&[
            (0.0, 25.0, 1.0),
            (0.0, 0.0, 0.0),
            (2.0, 0.0, 1.0),
            (10.0, 0.0, -0.5),
            (8.0, 9.0, 0.374794619217547),
            (21.0, 0.0, 0.0),
            (23.0, 0.0, 1.0),
            (32.0, 0.0, -0.5),
            (28.0, 0.0, 0.5),
            (39.0, 21.0, 0.0),
            (28.0, 12.0, 0.0),
        ]),
    )
}

fn pathological_profile1(segment_count: usize) -> BenchmarkProfile {
    let mut pline = Polyline::<f64>::with_capacity(segment_count, true);
    for i in 0..segment_count {
        let angle = i as f64 * TAU / segment_count as f64;
        let bulge = if i % 2 == 0 { 1.0 } else { -1.0 };
        pline.add(40.0 * angle.cos(), 40.0 * angle.sin(), bulge);
    }

    profile(
        format!("pathological_profile1_{segment_count}"),
        30,
        1.0,
        pline,
    )
}

fn native_profiles() -> Vec<BenchmarkProfile> {
    let mut profiles = vec![
        square(),
        diamond(),
        circle(),
        rounded_rectangle(),
        profile1(),
        profile2(),
    ];
    profiles.extend(PATHOLOGICAL_SEGMENT_COUNTS.map(pathological_profile1));
    profiles
}

fn arc_bearing_native_profiles() -> Vec<BenchmarkProfile> {
    let mut profiles = vec![circle(), rounded_rectangle(), profile1(), profile2()];
    profiles.extend(PATHOLOGICAL_SEGMENT_COUNTS.map(pathological_profile1));
    profiles
}

fn no_arcs_profiles() -> Vec<BenchmarkProfile> {
    arc_bearing_native_profiles()
        .into_iter()
        .map(|native| {
            let no_arcs_pline = native
                .pline
                .arcs_to_approx_lines(ARCS_TO_LINES_ERROR)
                .expect("arc-bearing benchmark profile should convert to line segments");
            profile(
                native.id,
                native.offset_count,
                native.offset_delta,
                no_arcs_pline,
            )
        })
        .collect()
}

fn profile_modes() -> [(&'static str, Vec<BenchmarkProfile>); 2] {
    [
        ("native", native_profiles()),
        ("no_arcs", no_arcs_profiles()),
    ]
}

fn shifted_profile(profile: &Polyline<f64>, x: f64, y: f64) -> Polyline<f64> {
    let mut shifted = profile.clone();
    shifted.translate_mut(x, y);
    shifted
}

fn shifted_profiles(profile: &Polyline<f64>) -> Vec<Polyline<f64>> {
    let extents = profile.extents().expect("benchmark profile has extents");
    let half_width = (extents.max_x - extents.min_x) / 2.0;
    let half_height = (extents.max_y - extents.min_y) / 2.0;

    (0..16)
        .map(|i| {
            let angle = i as f64 / 16.0 * TAU;
            shifted_profile(profile, half_width * angle.cos(), half_height * angle.sin())
        })
        .collect()
}

fn count_boolean_result(
    result: cavalier_contours::polyline::BooleanResult<Polyline<f64>>,
) -> usize {
    result.pos_plines.len() + result.neg_plines.len()
}

fn spatial_query_sum(pline: &Polyline<f64>) -> usize {
    let index = pline.create_approx_aabb_index();
    let mut stack = Vec::new();
    let mut total = 0;

    for (i, j) in pline.iter_segment_indexes() {
        let bb = seg_fast_approx_bounding_box(pline.at(i), pline.at(j));
        let mut visitor = |_: usize| {
            total += 1;
        };
        index.visit_query_with_stack(
            bb.min_x - 0.1,
            bb.min_y - 0.1,
            bb.max_x + 0.1,
            bb.max_y + 0.1,
            &mut visitor,
            &mut stack,
        );
    }

    total
}

fn winding_grid(profile: &Polyline<f64>) -> Vec<Vector2<f64>> {
    let extents = profile.extents().expect("benchmark profile has extents");
    let expansion = (extents.max_x - extents.min_x) / 2.0;
    let min_x = extents.min_x - expansion;
    let min_y = extents.min_y - expansion;
    let max_x = extents.max_x + expansion;
    let max_y = extents.max_y + expansion;
    let width = max_x - min_x;
    let height = max_y - min_y;

    let mut points = Vec::with_capacity(100);
    for i in 0..10 {
        for j in 0..10 {
            points.push(Vector2::new(
                i as f64 / 9.0 * width + min_x,
                j as f64 / 9.0 * height + min_y,
            ));
        }
    }
    points
}

fn bench_offsets(c: &mut Criterion) {
    for (mode, profiles) in profile_modes() {
        let mut group = c.benchmark_group(format!("offset/{mode}"));

        for profile in profiles {
            group.bench_function(BenchmarkId::from_parameter(profile.id.as_str()), |b| {
                b.iter(|| {
                    let mut result_count = 0;
                    for i in 1..=profile.offset_count {
                        let offset = i as f64 * profile.offset_delta;
                        result_count += black_box(&profile.pline)
                            .parallel_offset(black_box(offset))
                            .len();
                        result_count += black_box(&profile.pline)
                            .parallel_offset(black_box(-offset))
                            .len();
                    }
                    black_box(result_count)
                });
            });
        }

        group.finish();
    }
}

fn bench_booleans(c: &mut Criterion) {
    let operations = [
        BooleanOp::Or,
        BooleanOp::And,
        BooleanOp::Not,
        BooleanOp::Xor,
    ];

    for (mode, profiles) in profile_modes() {
        let mut shifted_group = c.benchmark_group(format!("boolean/shifted/{mode}"));
        for profile in profiles.iter() {
            let shifted = shifted_profiles(&profile.pline);
            shifted_group.bench_function(BenchmarkId::from_parameter(profile.id.as_str()), |b| {
                b.iter(|| {
                    let mut result_count = 0;
                    for shifted_pline in shifted.iter() {
                        for op in operations {
                            result_count += count_boolean_result(
                                black_box(&profile.pline)
                                    .boolean(black_box(shifted_pline), black_box(op)),
                            );
                        }
                    }
                    black_box(result_count)
                });
            });
        }
        shifted_group.finish();

        let mut coincident_group = c.benchmark_group(format!("boolean/coincident/{mode}"));
        for profile in profiles {
            coincident_group.bench_function(
                BenchmarkId::from_parameter(profile.id.as_str()),
                |b| {
                    b.iter(|| {
                        let mut result_count = 0;
                        for op in operations {
                            result_count += count_boolean_result(
                                black_box(&profile.pline)
                                    .boolean(black_box(&profile.pline), black_box(op)),
                            );
                        }
                        black_box(result_count)
                    });
                },
            );
        }
        coincident_group.finish();
    }
}

fn bench_intersections(c: &mut Criterion) {
    let mut group = c.benchmark_group("intersections/current_only");
    let cases = [
        ("square_vs_circle", square().pline, circle().pline),
        (
            "profile1_vs_profile2_shifted",
            profile1().pline,
            shifted_profile(&profile2().pline, 2.5, -1.0),
        ),
    ];

    for (name, pline_a, pline_b) in cases {
        group.bench_function(BenchmarkId::from_parameter(name), |b| {
            b.iter(|| {
                let intersects = black_box(&pline_a).find_intersects(black_box(&pline_b));
                black_box(
                    intersects.basic_intersects.len() + intersects.overlapping_intersects.len(),
                )
            });
        });
    }

    group.finish();
}

fn bench_spatial_index(c: &mut Criterion) {
    for (mode, profiles) in profile_modes() {
        let mut create_group = c.benchmark_group(format!("spatial_index/create/{mode}"));
        for profile in profiles.iter() {
            create_group.bench_function(BenchmarkId::from_parameter(profile.id.as_str()), |b| {
                b.iter(|| black_box(black_box(&profile.pline).create_approx_aabb_index()));
            });
        }
        create_group.finish();

        let mut query_group = c.benchmark_group(format!("spatial_index/query_reuse_stack/{mode}"));
        for profile in profiles {
            query_group.bench_function(BenchmarkId::from_parameter(profile.id.as_str()), |b| {
                b.iter(|| black_box(spatial_query_sum(black_box(&profile.pline))));
            });
        }
        query_group.finish();
    }
}

fn bench_properties(c: &mut Criterion) {
    for (mode, profiles) in profile_modes() {
        let mut area_group = c.benchmark_group(format!("properties/area/{mode}"));
        for profile in profiles.iter() {
            area_group.bench_function(BenchmarkId::from_parameter(profile.id.as_str()), |b| {
                b.iter(|| black_box(black_box(&profile.pline).area()));
            });
        }
        area_group.finish();

        let mut extents_group = c.benchmark_group(format!("properties/extents/{mode}"));
        for profile in profiles.iter() {
            extents_group.bench_function(BenchmarkId::from_parameter(profile.id.as_str()), |b| {
                b.iter(|| black_box(black_box(&profile.pline).extents()));
            });
        }
        extents_group.finish();

        let mut path_length_group = c.benchmark_group(format!("properties/path_length/{mode}"));
        for profile in profiles.iter() {
            path_length_group.bench_function(
                BenchmarkId::from_parameter(profile.id.as_str()),
                |b| {
                    b.iter(|| black_box(black_box(&profile.pline).path_length()));
                },
            );
        }
        path_length_group.finish();

        let mut winding_group = c.benchmark_group(format!("properties/winding_number_grid/{mode}"));
        for profile in profiles {
            let points = winding_grid(&profile.pline);
            winding_group.bench_function(BenchmarkId::from_parameter(profile.id.as_str()), |b| {
                b.iter(|| {
                    let mut winding_sum = 0;
                    for point in points.iter() {
                        winding_sum += black_box(&profile.pline).winding_number(black_box(*point));
                    }
                    black_box(winding_sum)
                });
            });
        }
        winding_group.finish();
    }
}

criterion_group! {
    name = geometry_baseline;
    config = Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_millis(100))
        .measurement_time(Duration::from_millis(300));
    targets = bench_offsets, bench_booleans, bench_intersections, bench_spatial_index, bench_properties
}
criterion_main!(geometry_baseline);
