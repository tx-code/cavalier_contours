# Phase 56: capi-specific-edge-runner-helper-extraction - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 56 reduces FFI parity test duplication by extracting a shared
specific-edge attribution helper and a reusable default/reversed matrix runner
for options-path `parallel_offset` parity/no-modify checks.

## Decisions

- **D-01:** Centralize specific-case provenance attribution in one helper
  function.
- **D-02:** Reuse one shared matrix runner for both reversed and default-input
  specific-edge parity tests.
- **D-03:** Keep this phase in test and planning artifacts only; no core
  geometry algorithm edits.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/55-capi-default-specific-edge-attribution-matrix/55-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`









