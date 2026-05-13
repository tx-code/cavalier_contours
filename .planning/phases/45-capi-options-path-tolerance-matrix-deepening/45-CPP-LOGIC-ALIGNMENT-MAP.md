# Phase 45: C-API Logic Alignment Map

This map captures next steps after options-path tolerance-matrix deepening.

## Deepening Outcome

- Options-path boolean circle/rectangle matrix now has bounded `pos_equal_eps`
  scale-matrix equivalence checks against default-path output.
- Options-path parallel-offset simple/specific matrices now have bounded
  tolerance scale-matrix equivalence checks against default-path output.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Options-path stress matrix expansion | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-explicit stress cases with concrete legacy provenance. |
| P2 | FFI parity helper extraction | `cavalier_contours_ffi/tests/test_pline.rs` | Refactor only when semantic behavior remains unchanged. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
