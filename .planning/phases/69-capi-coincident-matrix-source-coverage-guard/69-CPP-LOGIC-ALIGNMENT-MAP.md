# Phase 69: C-API Logic Alignment Map

This map captures next steps after coincident matrix source-coverage guard
hardening.

## Deepening Outcome

- Shared coincident matrix helper now has explicit source-backed guardrails for
  canonical case name coverage and operation-map stability.
- Guard diagnostics fail fast on omitted source-backed cases, case-count drift,
  and operation-map mismatches.
- Existing matrix behavior and assertions remain unchanged.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Expand source-backed matrix coverage with additional old C++ behavior cases | `cavalier_contours_ffi/tests/test_pline.rs` | Add only behavior cases with explicit old C++ comments or deterministic expected-output provenance. |
| P2 | Keep shared coincident helper guard diagnostics stable during future expansions | `cavalier_contours_ffi/tests/test_pline.rs` | Preserve canonical names, operation mapping, and explicit guard failures when extending helper cases. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
