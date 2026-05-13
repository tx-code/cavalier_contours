# Phase 77: capi-userdata-getter-bounds-contract-hardening - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 77 hardens C-API userdata getter correctness by adding explicit
out-of-bounds checks in CCW/CW userdata getters and aligning header docs/tests
to the explicit error contract.

## Decisions

- **D-01:** Keep this phase scoped to FFI runtime/headers/tests and planning
  artifacts.
- **D-02:** Align getter error semantics with sibling count/setter APIs:
  null shape => `1`, out-of-bounds index => `2`.
- **D-03:** Treat panic-based implicit failure as unacceptable for this public
  C-API surface.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi.h`
- `cavalier_contours_ffi/tests/test_pline.rs`
