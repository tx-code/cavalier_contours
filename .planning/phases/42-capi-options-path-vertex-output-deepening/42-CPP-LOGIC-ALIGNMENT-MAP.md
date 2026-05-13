# Phase 42: C-API Logic Alignment Map

This map captures next steps after options-path vertex-output deepening.

## Deepening Outcome

- Options-path boolean circle/rectangle operation matrix now has vertex-level
  output parity checks against default-path results.
- Options-path parallel-offset simple/specific matrices now have vertex-level
  output parity checks against default-path results.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift-failure triage template | `.planning/phases/*` | Keep drift-hook failure response deterministic before adding new cases. |
| P1 | Options-path coincident vertex-output deepening | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-explicit coincident cases with concrete old-C++ provenance. |
| P2 | FFI parity helper extraction | `cavalier_contours_ffi/tests/test_pline.rs` | Refactor only when it reduces duplication without semantic changes. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
