# Phase 32: capi-function-surface-combine-self-matrix-parity - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 32 closes remaining function-surface `combine_with_self_invariants` parity
at the C-API boundary for circle and closed half-circle matrix cases.

## Decisions

- **D-01:** Reuse source-backed circle and half-circle matrix case builders.
- **D-02:** Mirror old C++ invariants exactly: union/intersect return self,
  exclude/xor return empty.
- **D-03:** Validate output vertex equality and input no-modify behavior for each
  matrix case.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`
