# Phase 34: capi-function-surface-parallel-offset-full-matrix-parity - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 34 closes C-API function-surface full-matrix parallel-offset and collapsed
offset parity for generated circle and half-circle source cases.

## Decisions

- **D-01:** Reuse source-backed circle and half-circle case builders already used
  in function-surface parity phases.
- **D-02:** Match old C++ function-surface semantics: one expected offset for
  outward/inward probes and empty result for collapsed deltas.
- **D-03:** Validate with vertex-level parity; closed polylines allow rotational
  equivalence, open polylines require exact order.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`
