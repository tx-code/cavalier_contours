# Phase 72: C-API Logic Alignment Map

This map captures next steps after circle-rectangle source-matrix guard reuse.

## Deepening Outcome

- Explicit default-path circle-rectangle matrix now fails fast on
  source-backed case-count/name/operation drift.
- Circle-rectangle options/no-modify parity paths now share one canonical
  operation sequence with the default matrix parity flow.
- Existing property and vertex parity assertions remain unchanged; this phase is
  guard/order hardening only.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Expand explicit default-path source-backed matrix parity using additional old C++ behavior groups with deterministic expected properties | `cavalier_contours_ffi/tests/test_pline.rs` | Add only behavior groups with clear old C++ expected-output provenance. |
| P1 | Keep source-mapping and operation-order diagnostics stable while extending parity matrices | `cavalier_contours_ffi/tests/test_pline.rs` | Preserve canonical case-count/name/operation and operation-sequence guard surfaces. |
| P2 | Instantiate drift triage template when first real source update mismatch is observed | `.planning/phases/*` | Execute drift template only on real source delta, not synthetic guard tests. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
