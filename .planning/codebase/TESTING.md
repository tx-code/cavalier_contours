# Testing

Generated: 2026-05-12

## Standard Commands

- `cargo test --workspace`: run all workspace tests.
- `cargo build --workspace`: build all crates.
- `cargo fmt --all --check`: verify formatting.
- `cargo clippy --all-targets -- -D warnings`: run lints as errors.
- `cargo doc --workspace --no-deps`: build docs with warning checks in CI.

## CI Coverage

GitHub Actions run build and test jobs on:

- Ubuntu
- Windows
- macOS

The lint job also runs rustfmt, clippy, and documentation generation.

## Core Test Layout

Core integration tests live in `cavalier_contours/tests/`.

Key files include:

- `test_pline_parallel_offset.rs`
- `test_pline_boolean.rs`
- `test_shape_parallel_offset.rs`
- `test_pline_seg_intersect.rs`
- `test_line_circle_intersect.rs`
- `test_circle_circle_intersect.rs`

## Shared Test Utilities

`cavalier_contours/tests/test_utils/` provides reusable helpers for:

- fuzzy AABB comparisons;
- property set comparisons;
- modified polyline input variants;
- debug JSON output.

## Regression Pattern

Offset and boolean tests often compare output properties:

- vertex count;
- area;
- path length;
- extents;
- user data.

This is useful because equivalent geometry may have different vertex order or
direction. Reuse this pattern for algorithm regressions.

## FFI Tests

FFI tests live in `cavalier_contours_ffi/tests/` and exercise raw pointer API
behavior, status codes, transforms, offsetting, booleans, containment, AABB
indexing, polyline lists, and shape operations.

## Risk-Focused Testing

Add tests for:

- numerical tolerance boundaries;
- repeat-position or redundant vertices;
- tangent and overlapping intersections;
- line-arc and arc-arc cases;
- open versus closed polyline behavior;
- FFI null pointer and bounds handling when ABI code changes.
