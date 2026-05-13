# Phase 11: closest-point-and-generated-matrix-parity-expansion - Context

**Gathered:** 2026-05-13  
**Status:** Ready for execution

## Phase Boundary

Phase 11 expands function-level parity from Phase 10 into:
- C++ closest-point expectations in `TEST_cavc_pline_function.cpp`
- bounded generated case matrices from the same source

Clipper remains out of scope.

## Decisions

- **D-01:** Preserve source-level traceability for every imported case.
- **D-02:** For closest-point index ties, use explicit `index-check` policy
  (`strict` or `skip`) matching old C++ test intent.
- **D-03:** Keep matrix import bounded and classify residual gaps explicitly.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `cavalier_contours/tests/test_cpp_pline_function_parity.rs`

