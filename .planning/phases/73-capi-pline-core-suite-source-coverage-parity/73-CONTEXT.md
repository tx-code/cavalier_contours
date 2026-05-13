# Phase 73: capi-pline-core-suite-source-coverage-parity - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 73 hardens C-API pline core parity by explicitly covering old
`TEST_cavc_pline.cpp` core cases in one source-backed parity suite and enforcing
coverage integrity through a source-case coverage guard.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Mirror old core case semantics in a dedicated parity suite while
  preserving existing parity tests.
- **D-03:** Add a generic source-case coverage guard for missing/duplicate drift
  detection in the pline core suite.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/72-capi-circle-rectangle-source-matrix-guard-reuse/72-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`
