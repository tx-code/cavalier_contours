# Phase 31: capi-half-circle-closest-point-strict-index-parity - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 31 closes half-circle closest-point strict index parity at C-API boundary
using source-backed generated matrix expectations.

## Decisions

- **D-01:** Reuse source-backed half-circle matrix variants from prior Rust-core
  parity work.
- **D-02:** Keep strict index checking for all closest-point probes, including
  open/closed and alignment variants.
- **D-03:** Keep scope in tests/reporting only (no ABI surface expansion in this
  phase).

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`
