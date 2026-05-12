# Phase 10-01: C++ `pline_function` Parity Report

## Scope

This report maps selected expectations from old C++
`tests/tests/TEST_cavc_pline_function.cpp` into executable Rust parity tests.

## C++ to Rust Function-Level Map

| C++ function test surface | Rust parity surface | Notes |
|---------------------------|---------------------|-------|
| `cavc_get_area` | `PlineSource::area()` | Signed area parity on C++ circle-aligned cases. |
| `cavc_get_path_length` | `PlineSource::path_length()` | Circle path-length parity. |
| `cavc_get_extents` | `PlineSource::extents()` | Circle extents parity. |
| `cavc_get_winding_number` | `PlineSource::winding_number()` | Outside/inside point winding parity for CCW/CW circles. |
| `combine_with_self_invariants` | `PlineSource::boolean()` with self | OR/AND/NOT/XOR invariants against self. |

## Executed Evidence

Command:
`cargo test -p cavalier_contours --test test_cpp_pline_function_parity -- --nocapture`

Tests:
- `cpp_circle_function_metrics_parity`
- `cpp_circle_winding_number_parity`
- `cpp_combine_with_self_invariants_parity`

## Classification

- `bug`: none confirmed in 10-01.
- `intentional-divergence`: none observed in selected function-level cases.
- `not-comparable`: broader closest-point and full generated-case matrix from
  old C++ file is not imported yet; retained for follow-up expansion.

## 10-01 Decision

Selected high-signal function-level parity checks are executable and green.
Next expansion should target additional old-C++ closest-point and generated case
families as a separate bounded slice.
