# Phase 38: C-API Logic Alignment Map

This map captures next steps after the cross-suite coverage audit.

## Audit Outcome

- Old C++ suite blocks reviewed:
  - `TEST_cavc_pline.cpp`
  - `TEST_cavc_pline_function.cpp`
  - `TEST_cavc_parallel_offset.cpp`
  - `TEST_cavc_combine_plines.cpp`
- Outcome:
  - no hard uncovered source-explicit blocks detected
  - API-evolution equivalence areas are documented and explicitly tested

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Regression hardening for documented equivalence zones | `cavalier_contours_ffi/tests/test_pline.rs` + `.planning/phases/*` | Add tests only if they tighten source-backed equivalence areas (`reserve`, `remove` sequence) without inventing synthetic semantics. |
| P1 | Change-detection audit hook for old suite drift | `.planning/phases/*` | Re-run cross-suite checklist whenever upstream old C++ test blocks change. |
| P2 | Options-path deep edge catalog | `.planning/phases/*` + `cavalier_contours_ffi/tests/test_pline.rs` | Keep additions driven by demonstrated parity gaps rather than speculative synthetic cases. |

## File-Level Alignment Surface

- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi/src/lib.rs`
