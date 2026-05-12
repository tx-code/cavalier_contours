---
phase: 04
slug: benchmark-baseline
status: verified
verified: 2026-05-12
requirements:
  - BEN-01
  - BEN-02
  - BEN-03
---

# Phase 04 Verification

## Goal Achievement

Phase 4 establishes repeatable benchmark coverage for current Rust behavior and
historical old C++ benchmark families. The benchmark target is
`cavalier_contours/benches/geometry_baseline.rs`.

## Requirement Coverage

| Requirement | Status | Evidence |
|-------------|--------|----------|
| BEN-01 | Covered | Benchmark groups cover offsets, booleans, current-only intersections, spatial-index creation/query, and property workloads. |
| BEN-02 | Covered | `04-BENCHMARK-MAP.md` maps old C++ profile families and benchmark source files to Rust benchmark IDs. |
| BEN-03 | Covered | `04-BENCHMARKS.md` documents native arc, no-arcs conversion, setup, generated output, and oracle cost policy. |

## Verification Commands

| Command | Result |
|---------|--------|
| `cargo bench -p cavalier_contours --bench geometry_baseline -- --test` | Passed |
| `cargo bench -p cavalier_contours --bench geometry_baseline` | Passed |
| `cargo test --workspace` | Passed |
| `cargo fmt --all --check` | Passed |
| `cargo clippy --all-targets -- -D warnings` | Passed |
| `git diff --check` | Passed |
| `git status --short -- target` | No generated benchmark output staged or tracked |

## Scope Check

- Production geometry code under `cavalier_contours/src/` was not changed.
- `cavalier_contours_ffi.h` was not regenerated.
- No Clipper2 runtime or oracle call was introduced.
- UI code was not changed.
- Benchmark output under `target/criterion` remains local generated evidence.

## Notes

The full benchmark uses a broad-coverage Criterion configuration:
`sample_size = 10`, `warm_up_time = 100ms`, and `measurement_time = 300ms`.
This records a repeatable local baseline, not a performance budget or CI
threshold.

