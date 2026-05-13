# Phase 41: C-API Logic Alignment Map

This map captures next steps after options-path no-modify hardening.

## Hardening Outcome

- Options-path parallel-offset now explicitly preserves input polyline vertex
  data across source-backed simple and specific matrices.
- Options-path boolean circle/rectangle matrix now explicitly preserves both
  subject and clip input polylines across all operations.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Options-path output/vertex-level deepening | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-explicit cases that name concrete old-C++ provenance. |
| P1 | Drift-failure triage template | `.planning/phases/*` | Keep drift-hook failure path deterministic before adding tests. |
| P2 | FFI parity helper extraction | `cavalier_contours_ffi/tests/test_pline.rs` | Refactor only when it reduces duplication without semantic change. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
