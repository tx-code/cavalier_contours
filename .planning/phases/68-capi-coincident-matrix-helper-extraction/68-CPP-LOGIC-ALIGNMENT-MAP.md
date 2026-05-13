# Phase 68: C-API Logic Alignment Map

This map captures next steps after shared coincident matrix helper extraction.

## Deepening Outcome

- Coincident matrix suites now consume one shared source-backed helper
  (`cpp_coincident_boolean_matrix_cases`) for case metadata
  (`name`/`operation`/`subject`/`clip`).
- Canonical old C++ exclude identifiers (`excludeAFromB`, `excludeBFromA`)
  remain intact via shared helper output.
- Duplication is reduced across default/options/no-modify/output suites,
  lowering future drift risk.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Expand source-backed matrix coverage with additional old C++ behavior cases | `cavalier_contours_ffi/tests/test_pline.rs` | Add only behavior cases with explicit old C++ comments or deterministic expected-output provenance. |
| P2 | Keep shared coincident helper diagnostics stable during future expansions | `cavalier_contours_ffi/tests/test_pline.rs` | Preserve canonical names, operation mapping, and no-modify/output assertions when extending shared helper cases. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
