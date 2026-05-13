# Phase 70: C-API Logic Alignment Map

This map captures next steps after explicit coincident_case1 default-path matrix
parity expansion.

## Deepening Outcome

- C-API default-path parity now explicitly executes full source-backed
  `coincident_case1` matrix expectations (`union`, `excludeAFromB`,
  `excludeBFromA`, `intersect`, `xor`) using old C++ expected properties.
- Case1 exclude direction variants remain operation-correct and source-traceable
  in explicit parity output checks.
- Existing options/no-modify/output suites remain unchanged and continue to
  exercise broader coincident helper flows.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Expand source-backed matrix coverage with additional old C++ behavior cases | `cavalier_contours_ffi/tests/test_pline.rs` | Add only behavior cases with explicit old C++ comments or deterministic expected-output provenance. |
| P2 | Keep coincident helper and guard diagnostics stable during future expansions | `cavalier_contours_ffi/tests/test_pline.rs` | Preserve canonical names, operation mapping, and explicit coverage guard failures when extending helper cases. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
