# Phase 66: C-API Logic Alignment Map

This map captures next steps after source-backed specific-edge matrix
source-coverage guard hardening.

## Deepening Outcome

- Specific-edge matrix constructor now asserts that all source-backed old C++
  `simpleCases` intended for this matrix path are consumed by the selection
  loop; omitted case names are surfaced in failure diagnostics.
- Specific-edge matrix coverage remains unchanged for current source-backed
  case inputs and still includes `open_rectangle_inward` from Phase 65.
- Provenance diagnostics stay centralized in `cpp_specific_edge_attribution`
  with helper-based execution unchanged.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Expand source-backed specific-edge matrix coverage with additional old C++ edge inputs | `cavalier_contours_ffi/tests/test_pline.rs` | Add only edge cases with explicit old C++ comments or deterministic expected output provenance. |
| P2 | Keep helper-based runner diagnostics and coverage guard stable during future expansions | `cavalier_contours_ffi/tests/test_pline.rs` | Preserve mode/scale/case attribution labels, no-modify checks, and omitted-case diagnostics when adding new cases. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
