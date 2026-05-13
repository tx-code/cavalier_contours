# Phase 73: C-API Logic Alignment Map

This map captures next steps after pline core source-coverage suite parity
hardening.

## Deepening Outcome

- C-API pline core parity now explicitly covers source-backed old C++ core
  cases (`new`, `set_capacity`-equivalent reserve, `set_vertex_data`,
  `add_vertex`, `remove_range`-equivalent sequence, `clear`).
- Source-case coverage guard now enforces missing/duplicate/count drift
  detection for this core suite.
- Existing pline parity tests remain active and unchanged; this phase adds
  explicit source-backed core-suite coverage integrity.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Expand source-backed provenance labels for additional non-matrix pline/api tests with old C++ counterparts | `cavalier_contours_ffi/tests/test_pline.rs` | Add only labels/cases with deterministic old source mapping. |
| P1 | Keep source-case coverage guard semantics stable (count/missing/duplicate) during future suite growth | `cavalier_contours_ffi/tests/test_pline.rs` | Preserve current guard diagnostics format and failure intent. |
| P2 | Instantiate drift triage template when first real source update mismatch appears | `.planning/phases/*` | Run drift report workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
