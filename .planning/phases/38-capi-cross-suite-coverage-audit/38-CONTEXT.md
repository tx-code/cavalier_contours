# Phase 38: capi-cross-suite-coverage-audit - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 38 performs a cross-suite coverage audit mapping old C++ C-API test
blocks to current Rust FFI evidence and identifies any residual source-explicit
gaps.

## Decisions

- **D-01:** Use old C++ test files as canonical suite boundaries.
- **D-02:** Accept API-surface evolution only when equivalent behavior is
  explicitly verified on current APIs.
- **D-03:** Treat uncovered source-explicit blocks as actionable follow-up
  targets in next phases.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`
