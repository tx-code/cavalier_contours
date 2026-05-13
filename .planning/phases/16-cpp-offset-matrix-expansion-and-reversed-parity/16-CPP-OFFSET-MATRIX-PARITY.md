# Phase 16: C++ Offset Matrix Parity Report

## Scope

This report captures expanded old C++ `parallel_offset` parity imported from:

- `TEST_cavc_parallel_offset.cpp::createSimpleCases`
- `TEST_cavc_parallel_offset.cpp::createSpecificCases`
- `TEST_cavc_parallel_offset.cpp::reversed_parallel_offset_test`

## Imported Matrix Coverage

Executed in `cavalier_contours/tests/test_cpp_offset_parity.rs`:

- simple cases: **9**
- specific cases: **3**
- reversed parity checks: **12** (all imported cases)
- input immutability check: **1**

Validated outputs:

- result count and unordered property-set parity (`vertex_count`, `area`,
  `path_length`, `extents`)
- collapsed result parity (`collapsed_rectangle` expected empty)
- reversed parity rule:
  - reverse input orientation
  - negate offset delta
  - expected area sign inversion
- input polyline remains unchanged after offset call

## Classification

- `bug`: none confirmed in this imported matrix.
- `intentional-divergence`: none observed.
- `not-comparable`: direct one-to-one C++ intersection expected-table import
  remains out of this phase boundary.

## Evidence

- `cargo test -p cavalier_contours --test test_cpp_offset_parity -- --nocapture` - pass.

