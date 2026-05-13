# Phase 40: C-API Logic Alignment Map

This map captures next steps after adding the old-suite drift-detection hook.

## Hook Outcome

- Canonical drift baseline is in place for:
  - `TEST_cavc_pline.cpp`
  - `TEST_cavc_pline_function.cpp`
  - `TEST_cavc_parallel_offset.cpp`
  - `TEST_cavc_combine_plines.cpp`
- Hook command is executable and currently passes on no-drift state.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Options-path deep edge expansions | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-explicit cases traceable to old C++ suites. |
| P1 | Drift-failure triage template | `.planning/phases/*` | Keep failure handling deterministic: classify covered/equivalent/gap before writing tests. |
| P2 | FFI parity helper extraction | `cavalier_contours_ffi/tests/test_pline.rs` | Refactor only if no assertion semantics change. |

## File-Level Alignment Surface

- Hook artifacts:
  - `.planning/tools/cpp_suite_drift_baseline.json`
  - `.planning/tools/cpp_suite_drift_check.ps1`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
