# Phase 35: C++ C-API Combine-Self Vertex-Exact Reversed Parity

This report records C-API combine-with-self invariant parity closure at
vertex-exact level for the source-backed nontrivial sample polyline, including
reversed combinations.

## Scope

- C++ source reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust FFI test surface:
  - `cavalier_contours_ffi/tests/test_pline.rs`

## Added Coverage

- Vertex-exact combine-self test:
  - `pline_boolean_combine_with_self_invariants_vertex_exact_cpp_parity`

Coverage dimensions:

- union/intersect: forward self + reversed self exact vertex outputs
- exclude/xor: forward self, reversed self, reversed-forward cross combinations
- source-backed nontrivial sample polyline with arcs

## Classification

| Domain | Classification | Notes |
|--------|----------------|-------|
| C-API combine-self vertex-exact parity | parity | Union/intersect return exact expected vertex sequences for forward and reversed inputs. |
| Reversed/cross combination emptiness invariants | parity | Exclude/xor remain empty across forward/reversed and reversed-forward combinations. |
| Property-to-vertex parity strengthening | parity | Existing property-level invariants are strengthened by explicit vertex-level assertions. |

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
