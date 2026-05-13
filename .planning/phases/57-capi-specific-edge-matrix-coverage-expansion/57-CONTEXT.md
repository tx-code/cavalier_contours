# Phase 57: capi-specific-edge-matrix-coverage-expansion - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 57 expands source-backed specific-edge options-path matrix coverage by
adding an additional old C++ edge case into the helper-driven reversed/default
parity/no-modify matrix flow.

## Decisions

- **D-01:** Keep using the shared specific-edge runner helper introduced in
  Phase 56.
- **D-02:** Expand matrix coverage with only old C++ source-explicit edge
  cases and direct provenance labels.
- **D-03:** Keep this phase in test and planning artifacts only; no core
  geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/56-capi-specific-edge-runner-helper-extraction/56-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`









