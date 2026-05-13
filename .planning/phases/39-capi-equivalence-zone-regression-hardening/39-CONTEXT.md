# Phase 39: capi-equivalence-zone-regression-hardening - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 39 hardens documented API-evolution equivalence zones identified in Phase 38,
focusing on source-backed `reserve` and remove-sequence behaviors in the C-API test
surface.

## Decisions

- **D-01:** Keep scope strictly source-backed and avoid synthetic semantics.
- **D-02:** Strengthen equivalence assertions in executable FFI tests where old C++
  behavior had implicit/no-write guarantees.
- **D-03:** Publish a post-hardening alignment boundary before moving to drift-detection
  hooks.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/38-capi-cross-suite-coverage-audit/38-CROSS-SUITE-COVERAGE-CHECKLIST.md`
- `.planning/phases/38-capi-cross-suite-coverage-audit/38-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`
