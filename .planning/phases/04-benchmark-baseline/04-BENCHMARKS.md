# Phase 04 Benchmark Baseline

**Created:** 2026-05-12
**Target:** `cavalier_contours/benches/geometry_baseline.rs`

## Commands

Smoke/compile gate:

```powershell
cargo bench -p cavalier_contours --bench geometry_baseline -- --test
```

Full local baseline command:

```powershell
cargo bench -p cavalier_contours --bench geometry_baseline
```

The full command writes generated Criterion output under `target/criterion`.
Those generated outputs are local measurement artifacts and must not be
committed.

## Harness Mode

The benchmark target uses Criterion `0.8.2` with:

- `sample_size = 10`
- `warm_up_time = 100ms`
- `measurement_time = 300ms`
- crate default feature set includes `unsafe_optimizations` (forwarded to
  `static_aabb2d_index`)

This is a repeatable baseline configuration for broad coverage across many
profile families. It is not a production regression threshold.

## Coverage

The benchmark target establishes current Rust and historical-profile-shaped
benchmark groups for:

- `offset/native` and `offset/no_arcs`: public `parallel_offset` calls.
- `boolean/shifted/native`, `boolean/shifted/no_arcs`,
  `boolean/coincident/native`, and `boolean/coincident/no_arcs`: public
  `boolean` calls across all boolean operations.
- `intersections/current_only`: public `find_intersects` cases. These are
  current-only because the historical C++ benchmark suite does not include a
  matching intersection benchmark file.
- `spatial_index/create/native`, `spatial_index/create/no_arcs`,
  `spatial_index/query_reuse_stack/native`, and
  `spatial_index/query_reuse_stack/no_arcs`: `create_approx_aabb_index` and
  query-with-reused-stack style workloads. Query IDs run with prebuilt index
  and reused query buffers outside the timed loop, matching old C++ `QuerySetup`.
- `properties/area/*`, `properties/extents/*`, `properties/path_length/*`, and
  `properties/winding_number_grid/*`: property calls over mapped profile
  families.

Historical profile and operation mapping is recorded in `04-BENCHMARK-MAP.md`.

## Optional Rust/C++ Snapshot

An additional local cross-language snapshot was recorded on 2026-05-15:

- `.planning/phases/04-benchmark-baseline/04-RUST-CPP-SNAPSHOT-2026-05-15.md`
- `.planning/phases/04-benchmark-baseline/04-RUST-CPP-PROPERTIES-SNAPSHOT-2026-05-15.md`

This is comparative evidence only (same profile families, different benchmark
harness implementations). It is not a release gate or CI threshold.

## Environment Fields

Record these fields when capturing a full local baseline:

- Date and timezone.
- OS and CPU if available.
- `git rev-parse HEAD`
- `rustc -Vv`
- `cargo -V`
- Benchmark command and whether `-- --test` smoke mode or full Criterion
  measurement mode was used.
- Cargo profile notes, including workspace dev dependency optimization.

## Captured Baseline Run

| Field | Value |
|-------|-------|
| Date | 2026-05-12 |
| Timezone | China Standard Time |
| OS | Microsoft Windows NT 10.0.26200.0 |
| CPU | 12th Gen Intel(R) Core(TM) i5-12600KF |
| Source revision | `688a72fd2d2a1255f742dd56eb49649ffbb8b82e` before final verification-doc commit |
| `rustc -Vv` | `rustc 1.92.0 (ded5c06cf 2025-12-08)`, host `x86_64-pc-windows-msvc`, LLVM `21.1.3` |
| `cargo -V` | `cargo 1.92.0 (344c4567c 2025-10-21)` |
| Smoke mode | `cargo bench -p cavalier_contours --bench geometry_baseline -- --test` passed |
| Full measurement mode | `cargo bench -p cavalier_contours --bench geometry_baseline` passed |
| Generated output | Created under `target/criterion`; not committed |

## Cost Accounting

- Setup cost is excluded when a benchmark builds deterministic shifted inputs,
  query stacks, or other reusable state before the timed loop.
- Native arc costs are measured through current Rust arc-aware polyline APIs.
- Conversion cost is excluded from no-arcs variants unless a future benchmark ID
  explicitly names conversion measurement.
- Oracle cost is excluded. Clipper2 runtime and oracle policy belong to Phase 5,
  not Phase 4.
- Criterion harness overhead is accepted as part of the stable local measurement
  harness.

## Scope Exclusions

Phase 4 does not define performance budgets, CI regression thresholds, Clipper2
comparisons, FFI benchmarks, FFI header regeneration, production algorithm
changes, or UI changes.
