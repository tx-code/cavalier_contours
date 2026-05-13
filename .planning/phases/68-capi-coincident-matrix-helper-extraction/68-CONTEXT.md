# Phase 68: capi-coincident-matrix-helper-extraction - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 68 reduces drift risk in C-API coincident matrix parity tests by
extracting a shared source-backed case helper used across default/options and
no-modify/output suites, without changing behavior.

## Decisions

- **D-01:** Keep this phase scoped to test helper structure and metadata reuse;
  no production geometry algorithm edits.
- **D-02:** Reuse one canonical coincident case list (`name`, `operation`,
  `subject`, `clip`) across matrix suites to prevent name/operation drift.
- **D-03:** Preserve all existing assertions and expected outputs.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/67-capi-coincident-exclude-name-canonicalization/67-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`
