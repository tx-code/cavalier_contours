use cavalier_contours::core::math::Vector2;
use cavalier_contours::polyline::{
    BooleanOp, PlineCreation, PlineSource, PlineSourceMut, Polyline, seg_fast_approx_bounding_box,
};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn closed_polyline(vertices: &[(f64, f64, f64)]) -> Polyline<f64> {
    let mut pline = Polyline::<f64>::with_capacity(vertices.len(), true);
    for &(x, y, bulge) in vertices {
        pline.add(x, y, bulge);
    }
    pline
}

fn current_square() -> Polyline<f64> {
    closed_polyline(&[
        (-15.0, -15.0, 0.0),
        (25.0, -15.0, 0.0),
        (25.0, 25.0, 0.0),
        (-15.0, 25.0, 0.0),
    ])
}

fn current_circle() -> Polyline<f64> {
    closed_polyline(&[(-35.0, 5.0, 1.0), (45.0, 5.0, 1.0)])
}

fn current_arc_profile() -> Polyline<f64> {
    closed_polyline(&[
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 1.0),
        (10.0, 0.0, -0.5),
        (10.0, 10.0, 0.5),
        (14.0, 20.0, -0.5),
        (0.0, 20.0, 0.0),
    ])
}

fn current_profiles() -> Vec<(&'static str, Polyline<f64>)> {
    vec![
        ("square", current_square()),
        ("circle", current_circle()),
        ("arc_profile", current_arc_profile()),
    ]
}

fn shifted_profile(profile: &Polyline<f64>, x: f64, y: f64) -> Polyline<f64> {
    let mut shifted = profile.clone();
    shifted.translate_mut(x, y);
    shifted
}

fn count_boolean_result(result: cavalier_contours::polyline::BooleanResult<Polyline<f64>>) -> usize {
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

fn bench_offsets(c: &mut Criterion) {
    let mut group = c.benchmark_group("offset/current");

    for (name, pline) in current_profiles() {
        group.bench_with_input(BenchmarkId::from_parameter(name), &pline, |b, pline| {
            b.iter(|| {
                let mut result_count = 0;
                for offset in [0.25, 1.0, -0.25, -1.0] {
                    result_count += black_box(pline).parallel_offset(black_box(offset)).len();
                }
                black_box(result_count)
            });
        });
    }

    group.finish();
}

fn bench_booleans(c: &mut Criterion) {
    let mut group = c.benchmark_group("boolean/current");
    let operations = [BooleanOp::Or, BooleanOp::And, BooleanOp::Not, BooleanOp::Xor];

    for (name, pline) in current_profiles() {
        let shifted = shifted_profile(&pline, 7.5, 4.0);
        group.bench_function(BenchmarkId::new("shifted", name), |b| {
            b.iter(|| {
                let mut result_count = 0;
                for op in operations {
                    result_count += count_boolean_result(
                        black_box(&pline).boolean(black_box(&shifted), black_box(op)),
                    );
                }
                black_box(result_count)
            });
        });

        group.bench_function(BenchmarkId::new("coincident", name), |b| {
            b.iter(|| {
                let mut result_count = 0;
                for op in operations {
                    result_count +=
                        count_boolean_result(black_box(&pline).boolean(black_box(&pline), op));
                }
                black_box(result_count)
            });
        });
    }

    group.finish();
}

fn bench_intersections(c: &mut Criterion) {
    let mut group = c.benchmark_group("intersections/current_only");
    let cases = [
        ("square_vs_circle", current_square(), current_circle()),
        (
            "arc_profile_vs_shifted",
            current_arc_profile(),
            shifted_profile(&current_arc_profile(), 2.5, -1.0),
        ),
    ];

    for (name, pline_a, pline_b) in cases {
        group.bench_function(BenchmarkId::from_parameter(name), |b| {
            b.iter(|| {
                let intersects = black_box(&pline_a).find_intersects(black_box(&pline_b));
                black_box(intersects.basic_intersects.len() + intersects.overlapping_intersects.len())
            });
        });
    }

    group.finish();
}

fn bench_spatial_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("spatial_index/current");

    for (name, pline) in current_profiles() {
        group.bench_with_input(BenchmarkId::new("create", name), &pline, |b, pline| {
            b.iter(|| black_box(black_box(pline).create_approx_aabb_index()));
        });

        group.bench_with_input(
            BenchmarkId::new("query_reuse_stack", name),
            &pline,
            |b, pline| b.iter(|| black_box(spatial_query_sum(black_box(pline)))),
        );
    }

    group.finish();
}

fn bench_properties(c: &mut Criterion) {
    let mut group = c.benchmark_group("properties/current");

    for (name, pline) in current_profiles() {
        group.bench_with_input(BenchmarkId::new("area", name), &pline, |b, pline| {
            b.iter(|| black_box(black_box(pline).area()));
        });
        group.bench_with_input(BenchmarkId::new("extents", name), &pline, |b, pline| {
            b.iter(|| black_box(black_box(pline).extents()));
        });
        group.bench_with_input(BenchmarkId::new("path_length", name), &pline, |b, pline| {
            b.iter(|| black_box(black_box(pline).path_length()));
        });
        group.bench_with_input(
            BenchmarkId::new("winding_number_grid", name),
            &pline,
            |b, pline| {
                let extents = pline.extents().expect("benchmark profile has extents");
                let points = [
                    Vector2::new(extents.min_x, extents.min_y),
                    Vector2::new((extents.min_x + extents.max_x) * 0.5, extents.min_y),
                    Vector2::new(extents.max_x, extents.max_y),
                    Vector2::new((extents.min_x + extents.max_x) * 0.5, extents.max_y),
                ];
                b.iter(|| {
                    let mut winding_sum = 0;
                    for point in points {
                        winding_sum += black_box(pline).winding_number(black_box(point));
                    }
                    black_box(winding_sum)
                });
            },
        );
    }

    group.finish();
}

criterion_group! {
    name = geometry_baseline;
    config = Criterion::default().sample_size(10);
    targets = bench_offsets, bench_booleans, bench_intersections, bench_spatial_index, bench_properties
}
criterion_main!(geometry_baseline);
