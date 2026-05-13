# Phase 67: capi-coincident-exclude-name-canonicalization - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 67 deepens old C++ naming alignment by canonicalizing coincident exclude
case labels to match source identifiers (`excludeAFromB`, `excludeBFromA`)
across C-API boolean matrix suites, without changing geometry behavior.

## Decisions

- **D-01:** Keep this phase label-only in test metadata; no production geometry
  algorithm edits.
- **D-02:** Use canonical old C++ case labels consistently for coincident case1
  and case2 exclude variants across default/options/no-modify matrix flows.
- **D-03:** Preserve existing expected outputs, operation wiring, and
  no-modify assertions.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/66-capi-specific-edge-matrix-source-coverage-guard/66-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`
