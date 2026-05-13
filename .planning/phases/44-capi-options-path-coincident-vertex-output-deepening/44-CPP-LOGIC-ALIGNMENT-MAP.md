# Phase 44: C-API Logic Alignment Map

This map captures next steps after options-path coincident vertex-output
deepening.

## Deepening Outcome

- Options-path coincident case1/case2 matrices now have vertex-level output
  parity checks against default-path results.
- Remaining and subtracted sets are both covered at vertex level.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage on first real drift event. |
| P1 | Options-path tolerance matrix deepening | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-explicit tolerance/epsilon cases with clear provenance. |
| P2 | FFI parity helper extraction | `cavalier_contours_ffi/tests/test_pline.rs` | Refactor only when semantic behavior is unchanged. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
