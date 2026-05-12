# Plan 04-01 Summary: Current Rust Measurement Baseline

**Completed:** 2026-05-12
**Status:** Complete

## Changes

- Added Criterion `0.8.2` as a core-crate dev-dependency and declared the
  `geometry_baseline` benchmark target.
- Created `cavalier_contours/benches/geometry_baseline.rs` with current Rust
  benchmark groups for offsets, booleans, intersections, spatial-index-heavy
  workloads, and polyline properties.
- Created `04-BENCHMARKS.md` with smoke/full benchmark commands, generated
  output policy, environment fields, cost-accounting notes, and Phase 4 scope
  exclusions.

## Verification

- `cargo bench -p cavalier_contours --bench geometry_baseline -- --test` passed.
- `git status --short -- target` reported no generated benchmark output to
  commit.

## Notes

- Criterion `0.8.2` declares `rust-version = 1.86`, which is compatible with
  this workspace's Rust 1.88 MSRV.
- The initial intersection group is labeled `current_only` because the old C++
  benchmark suite has no matching intersection benchmark source.

