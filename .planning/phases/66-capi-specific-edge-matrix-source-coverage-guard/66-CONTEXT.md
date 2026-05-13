# Phase 66: capi-specific-edge-matrix-source-coverage-guard - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 66 hardens source-backed specific-edge options-path alignment by adding a
coverage guard that fails when any old C++ source-backed `simpleCases` input is
omitted from the matrix constructor flow.

## Decisions

- **D-01:** Keep using the shared specific-edge runner helper introduced in
  Phase 56 and expanded through Phase 65.
- **D-02:** Keep old C++ source-backed case imports unchanged; this phase adds a
  guardrail, not new geometry behavior.
- **D-03:** Keep this phase in test and planning artifacts only; no core
  geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/65-capi-specific-edge-matrix-open-rectangle-inward-expansion/65-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`











