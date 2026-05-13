# Phase 56: C-API Logic Alignment Map

This map captures next steps after specific-edge runner helper extraction.

## Deepening Outcome

- Specific-edge provenance mapping is centralized in
  `cpp_specific_edge_attribution`.
- Reversed and default-input specific-edge matrix execution now shares
  `run_parallel_offset_options_specific_edge_attribution_matrix`, preserving
  parity/no-modify diagnostics across mode/scale combinations.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Expand source-backed specific-case matrix coverage | `cavalier_contours_ffi/tests/test_pline.rs` | Add only old C++ source-explicit cases with direct provenance notes. |
| P2 | Keep helper-based runner diagnostics stable during future expansions | `cavalier_contours_ffi/tests/test_pline.rs` | Preserve mode/scale/case attribution labels and no-modify checks when adding new cases. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`









