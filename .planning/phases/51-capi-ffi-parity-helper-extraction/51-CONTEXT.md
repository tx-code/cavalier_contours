# Phase 51: capi-ffi-parity-helper-extraction - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 51 reduces FFI parity test duplication by extracting shared helper
constructs for options initialization and mode/scale matrices, while preserving
existing test behavior.

## Decisions

- **D-01:** Extract reusable helper constructs only within
  `cavalier_contours_ffi/tests/test_pline.rs`.
- **D-02:** Keep behavioral assertions and test matrices unchanged.
- **D-03:** Keep this phase in test and planning artifacts only; no core
  geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/50-capi-options-path-reversed-self-intersects-no-modify-stress-matrix/50-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`




