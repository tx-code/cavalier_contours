# Phase 18: Coincident Intersect Collapsed-Filter Parity Report

## Scope

This report captures parity behavior for `coincident_case1_intersect` from old
C++ `TEST_cavc_combine_plines.cpp`, comparing:

- default boolean path
- collapsed-area filtered boolean path (`collapsed_area_eps`)

## Executed Evidence

Source test:
- `cavalier_contours/tests/test_cpp_combine_parity.rs`

Executed checks:
- default path in `cpp_coincident_matrix_geometry_parity_holds`
- filtered path in
  `cpp_coincident_case1_intersect_with_collapsed_filter_matches_cpp_empty`

## Classification

- default path:
  - `intentional-divergence`
  - Rust retains a tiny zero-area sliver (`vertex_count=2`,
    `path_length≈0.02`) where old C++ expected empty
- `collapsed_area_eps` filtered path:
  - `parity`
  - Rust returns empty result, matching old C++ expectation
- `bug`: none confirmed in this bounded phase
- `not-comparable`: none introduced

## Decision

Keep current default behavior unchanged and preserve the collapsed-filter path
as an explicit alignment route until a broader default-threshold impact study
is completed.

## Evidence

- `cargo test -p cavalier_contours --test test_cpp_combine_parity -- --nocapture` - pass.

