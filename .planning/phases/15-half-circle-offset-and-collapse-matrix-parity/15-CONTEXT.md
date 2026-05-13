# Phase 15: half-circle-offset-and-collapse-matrix-parity - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 15 deepens old C++ vs Rust function-level parity (no Clipper) by
executing generated half-circle matrix `parallel_offset` and
collapsed-offset expectations from
`TEST_cavc_pline_function.cpp::addHalfCircleCases`.

## Decisions

- **D-01:** Use source-traceable formulas from old C++ for closed half-circle
  outward connection arcs and inward intersection bulges.
- **D-02:** Validate offset outputs at property and vertex level; allow
  closed-curve start-index rotation only for closed expected results.
- **D-03:** Keep remaining follow-up focused on broader tie-cases and
  additional C++ function suites.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
