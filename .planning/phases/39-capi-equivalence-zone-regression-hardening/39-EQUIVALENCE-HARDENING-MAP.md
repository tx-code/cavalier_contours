# Phase 39: Equivalence-Zone Hardening Map

This map captures the direct outcomes of source-backed equivalence-zone
hardening and defines the next boundary.

## Hardening Outcome

- Reserve equivalence zone:
  - shrink-noop and grow reserve calls now remain covered with preserved prefix
    data plus post-reserve append checks.
- Remove-sequence equivalence zone:
  - final empty-state behavior now asserts `cavc_pline_get_vertex_data` leaves
    caller buffer unchanged in-flow, mirroring old suite expectation.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Old-suite drift detection hook | `.planning/phases/*` | Re-run/update cross-suite checklist if old C++ pline/function/offset/combine suite blocks change. |
| P1 | Options-path deep edge expansions | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-explicit, reproducible edge cases with concrete legacy provenance. |
| P2 | FFI helper extraction for parity fixtures | `cavalier_contours_ffi/tests/test_pline.rs` | Refactor only if it reduces duplication without changing assertion semantics. |

## File-Level References

- C++ source:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
- Rust FFI tests:
  - `cavalier_contours_ffi/tests/test_pline.rs`
