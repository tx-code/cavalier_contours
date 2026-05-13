# Phase 14: circle-offset-and-collapse-matrix-parity - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 14 deepens old C++ vs Rust function-level parity (no Clipper) by
executing generated circle matrix `parallel_offset` and collapsed-offset
expectations from `TEST_cavc_pline_function.cpp::addCircleCases`.

## Decisions

- **D-01:** Execute circle generated matrix offset parity before half-circle
  offset matrix due lower formula complexity and clearer expected shape.
- **D-02:** Validate offset results at both property level and vertex level
  (allowing closed-curve start-index rotation).
- **D-03:** Keep half-circle offset vertex matrices as explicit next phase.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
