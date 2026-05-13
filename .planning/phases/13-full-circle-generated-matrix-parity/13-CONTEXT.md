# Phase 13: full-circle-generated-matrix-parity - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 13 deepens old C++ vs Rust function-level parity (no Clipper) by
executing full generated circle matrices from
`TEST_cavc_pline_function.cpp::addCircleCases`.

## Decisions

- **D-01:** Keep tolerance management centralized in parity tests using
  `test_utils::PlineProperties` constants.
- **D-02:** Use strict index checks only where old C++ expectations are
  explicit (vertex-on-vertex closest-point expectations).
- **D-03:** Keep offset/collapsed-offset matrix parity as the next phase.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours/tests/test_cpp_pline_function_parity.rs`
