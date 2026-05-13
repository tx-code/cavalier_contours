# Phase 57: C-API Logic Alignment Map

This map captures next steps after source-backed specific-edge matrix coverage
expansion.

## Deepening Outcome

- Specific-edge matrix coverage now includes one additional source-backed old
  C++ edge case (`closed_rectangle_coincident`) in both reversed/default
  options-path helper flows.
- Provenance diagnostics stay centralized in `cpp_specific_edge_attribution`
  with helper-based execution unchanged.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Expand source-backed specific-edge matrix coverage with additional old C++ edge inputs | `cavalier_contours_ffi/tests/test_pline.rs` | Add only edge cases with explicit old C++ comments or deterministic expected output provenance. |
| P2 | Keep helper-based runner diagnostics stable during future expansions | `cavalier_contours_ffi/tests/test_pline.rs` | Preserve mode/scale/case attribution labels and no-modify checks when adding new cases. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`









