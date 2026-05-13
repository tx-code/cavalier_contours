# Phase 17: C++ Coincident Combine Parity Report

## Scope

This report captures imported coincident combine parity from old C++
`TEST_cavc_combine_plines.cpp::createCoincidentCases` for:

- `Or`
- `Not`
- `And`
- `Xor`

across `coincident_case1_*` and `coincident_case2_*` case families.

## Imported Matrix Coverage

Executed in `cavalier_contours/tests/test_cpp_combine_parity.rs`:

- coincident cases executed: **10**
- combine modes covered: **4**

Validated outputs:

- result count and unordered geometry-property parity
  (`abs(area)`, `path_length`, `extents`)
- explicit divergence capture for one bounded case

## Classification

- `parity`: `coincident_case1_union`, `coincident_case1_excludeAFromB`,
  `coincident_case1_excludeBFromA`, `coincident_case1_xor`, all
  `coincident_case2_*`.
- `intentional-divergence`:
  - `coincident_case1_intersect`:
    - old C++ expectation is empty
    - Rust currently preserves a tiny zero-area sliver
      (`vertex_count=2`, `path_length≈0.02`)
- `bug`: none confirmed in this phase.
- `not-comparable`: none introduced in this imported matrix.

## Evidence

- `cargo test -p cavalier_contours --test test_cpp_combine_parity -- --nocapture` - pass.

