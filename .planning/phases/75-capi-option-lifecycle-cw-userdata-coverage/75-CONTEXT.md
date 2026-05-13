# Phase 75: capi-option-lifecycle-cw-userdata-coverage - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 75 closes remaining FFI surface coverage gaps by adding:

- option-object lifecycle tests (create/init/free semantics for uncovered exports),
  and
- CW userdata setter behavior tests on shapes.

## Decisions

- **D-01:** Keep this phase scoped to FFI tests and planning artifacts.
- **D-02:** Validate lifecycle parity by comparing create/default objects with
  init-written defaults where applicable.
- **D-03:** Validate CW userdata setter behavior on success, null-shape,
  out-of-bounds, and clear-path semantics.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/74-capi-aabbindex-extents-source-parity/74-CPP-LOGIC-ALIGNMENT-MAP.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`
