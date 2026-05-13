# Phase 50: capi-options-path-reversed-self-intersects-no-modify-stress-matrix - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 50 deepens options-path reversed-input coverage by executing a bounded
no-modify stress matrix (`mode x tolerance scale`) across source-backed simple
and specific `parallel_offset` cases under `invert_direction + negated delta`.

## Decisions

- **D-01:** Reuse only source-backed `parallel_offset` simple/specific case
  inputs from old C++ suite.
- **D-02:** Validate input vertex stability per mode/scale combination on
  reversed inputs.
- **D-03:** Keep this phase in FFI tests and planning artifacts only; no core
  geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/49-capi-options-path-reversed-self-intersects-stress-matrix/49-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`



