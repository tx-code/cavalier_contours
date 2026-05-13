# Phase 23: capi-parallel-offset-matrix-bridge - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 23 bridges old C++ `parallel_offset` parity through Rust FFI
`cavc_pline_parallel_offset`, including:

- simple matrix cases
- specific matrix cases
- reversed-input parity
- input no-modify invariants

## Decisions

- **D-01:** Reuse source-traceable case/expected sets already validated in
  Rust-core parity tests.
- **D-02:** Match property sets unordered and compare absolute area for
  robustness across orientation.
- **D-03:** Keep options defaults in bridge tests and focus on legacy C-API
  behavioral parity.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours/tests/test_cpp_offset_parity.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`
