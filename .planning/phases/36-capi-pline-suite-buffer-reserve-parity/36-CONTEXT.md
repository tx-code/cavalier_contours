# Phase 36: capi-pline-suite-buffer-reserve-parity - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 36 closes remaining source-backed `TEST_cavc_pline.cpp` behavior gaps at
C-API boundary for empty-buffer read safety and reserve non-modification
invariants.

## Decisions

- **D-01:** Add explicit no-write check for `cavc_pline_get_vertex_data` on
  empty polyline using sentinel buffer.
- **D-02:** Add explicit no-modify check for `cavc_pline_reserve` on populated
  polyline to preserve existing vertex data.
- **D-03:** Keep scope to source-backed semantics only; no API surface changes.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`
