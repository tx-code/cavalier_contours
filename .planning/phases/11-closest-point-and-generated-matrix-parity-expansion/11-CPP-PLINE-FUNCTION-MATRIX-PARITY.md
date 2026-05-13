# Phase 11-02: C++ Function Matrix Parity Report

## Scope

This report captures a bounded generated-case subset imported from old C++
`TEST_cavc_pline_function.cpp::addHalfCircleCases`.

## Imported Subset

| C++ generated family | Rust parity coverage | Status |
|----------------------|----------------------|--------|
| `ccw_half_circle_x_aligned` (radius=5, center=(1,1), open) | `cpp_generated_half_circle_matrix_subset_parity` | pass |
| `ccw_half_circle_x_aligned` (radius=5, center=(1,1), closed) | `cpp_generated_half_circle_matrix_subset_parity` | pass |

Validated properties:
- `area`
- `path_length`
- `extents`
- `winding_number` (open vs closed expectation)

## Classification

- `bug`: none confirmed in imported subset.
- `intentional-divergence`: none observed in imported subset.
- `not-comparable`: broader generated matrices (other centers/orientations/reverse
  variants plus full closest-point index matrix) remain out of this bounded
  import and are explicitly deferred.

## Decision

Generated-matrix parity is now partially executable with evidence. Continue
expansion in follow-up slices when higher coverage is needed.

