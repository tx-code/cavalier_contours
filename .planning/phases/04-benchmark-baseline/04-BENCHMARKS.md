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

## Initial Coverage

Wave 1 establishes current Rust benchmark groups for:

- `offset/current`: public `parallel_offset` calls.
- `boolean/current`: shifted and coincident `boolean` calls across all boolean
  operations.
- `intersections/current_only`: public `find_intersects` cases. These are
  current-only because the historical C++ benchmark suite does not include a
  matching intersection benchmark file.
- `spatial_index/current`: `create_approx_aabb_index` and query-with-reused-stack
  style workloads.
- `properties/current`: area, extents, path length, and winding-number calls.

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

