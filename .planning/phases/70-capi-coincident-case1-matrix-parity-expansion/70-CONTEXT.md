# Phase 70: capi-coincident-case1-matrix-parity-expansion - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 70 expands source-backed C-API boolean parity by importing full old C++
`coincident_case1` matrix expectations (`union`, `excludeAFromB`,
`excludeBFromA`, `intersect`, `xor`) into explicit default-path parity checks.

## Decisions

- **D-01:** Keep this phase limited to FFI parity tests and planning artifacts.
- **D-02:** Use old C++ expected property tuples directly for case1 matrix
  outputs.
- **D-03:** Preserve existing options/no-modify/output suites; this phase adds
  missing explicit default-path source parity for case1.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/69-capi-coincident-matrix-source-coverage-guard/69-CPP-LOGIC-ALIGNMENT-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `cavalier_contours_ffi/tests/test_pline.rs`
