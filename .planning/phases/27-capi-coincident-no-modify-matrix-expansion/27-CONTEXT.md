# Phase 27: capi-coincident-no-modify-matrix-expansion - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 27 expands C-API boolean no-modify invariants from simple cases to
source-backed coincident combine matrices.

## Decisions

- **D-01:** Reuse old C++ coincident case1/case2 matrix inputs for C-API
  no-modify checks.
- **D-02:** Validate both subject and clip vertex buffers before/after every
  operation.
- **D-03:** Explicitly include exclusion direction variants (`A-B`, `B-A`) in
  coincident no-modify parity checks.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`
