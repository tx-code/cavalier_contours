# Phase 69: capi-coincident-matrix-source-coverage-guard - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 69 hardens old C++ source alignment for coincident boolean matrix helpers
by adding explicit coverage and operation-map guards for canonical
`coincident_case1/2` cases.

## Decisions

- **D-01:** Keep this phase scoped to test helper guardrails; no production
  geometry algorithm edits.
- **D-02:** Guard canonical source-backed case name coverage and operation-map
  mapping (`Or/Not/And/Xor`) for coincident helper output.
- **D-03:** Preserve existing suite behavior and diagnostics.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/68-capi-coincident-matrix-helper-extraction/68-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`
