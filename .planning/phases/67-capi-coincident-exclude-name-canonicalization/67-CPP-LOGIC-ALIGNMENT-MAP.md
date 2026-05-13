# Phase 67: C-API Logic Alignment Map

This map captures next steps after canonical coincident exclude naming
alignment.

## Deepening Outcome

- Coincident exclude case labels now match old C++ canonical identifiers
  (`excludeAFromB`, `excludeBFromA`) for case1/case2 metadata across default,
  options-path, and no-modify matrix suites.
- Behavior expectations and operation routing remain unchanged; this phase is
  naming/diagnostic alignment only.
- Source-backed diagnostics remain explicit and closer to old C++ case naming.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Expand source-backed matrix coverage with additional old C++ behavior cases | `cavalier_contours_ffi/tests/test_pline.rs` | Add only behavior cases with explicit old C++ comments or deterministic expected-output provenance. |
| P2 | Keep canonical case naming and helper diagnostics stable during future expansions | `cavalier_contours_ffi/tests/test_pline.rs` | Preserve canonical old C++ case identifiers, mode/scale labels, and no-modify diagnostics. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
