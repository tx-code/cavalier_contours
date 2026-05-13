# Phase 49: capi-options-path-reversed-self-intersects-stress-matrix - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 49 deepens options-path reversed-input coverage by executing a bounded
stress matrix (`mode x tolerance scale`) across source-backed simple and
specific `parallel_offset` cases under `invert_direction + negated delta`.

## Decisions

- **D-01:** Reuse only source-backed `parallel_offset` simple/specific case
  inputs from old C++ suite.
- **D-02:** Validate default-path parity for both property and vertex outputs
  per mode/scale combination on reversed inputs.
- **D-03:** Keep this phase in FFI tests and planning artifacts only; no core
  geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/48-capi-options-path-self-intersects-stress-matrix/48-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`


