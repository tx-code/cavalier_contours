# Phase 20: capi-coincident-intersect-parity-bridge - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 20 extends the Phase 19 default-path fix through the public C-API layer
(`cavc_pline_boolean`) so parity is proven at both Rust-core and FFI boundaries.

## Decisions

- **D-01:** Use old C++ `coincident_case1` source inputs directly in FFI test
  form.
- **D-02:** Validate `operation=1` (`BooleanOp::And`) explicit mapping in the
  FFI API contract.
- **D-03:** Keep this phase focused on one high-signal bridge case before
  broader C-API matrix expansion.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi/tests/test_pline.rs`
