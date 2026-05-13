# Phase 75: C-API Logic Alignment Map

This map captures next steps after option lifecycle and CW userdata coverage
hardening.

## Deepening Outcome

- Previously uncovered option-lifecycle exports are now explicitly exercised in
  FFI tests (`create/init/free` and null-path behavior where applicable).
- `cavc_shape_set_cw_pline_userdata_values` is now explicitly exercised for
  success, bounds-check, null-shape error, and clear-path behavior.
- `test_pline` function-surface coverage increased while preserving existing
  behavior assertions.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Re-run exported-function coverage scan and close any newly discovered uncovered FFI exports with bounded tests | `cavalier_contours_ffi/tests/test_pline.rs` | Add only direct API-behavior checks with deterministic expected outcomes. |
| P1 | Continue adding source-backed provenance labels where old C++/docs mappings are explicit | `cavalier_contours_ffi/tests/test_pline.rs` | Keep labels grounded in concrete source references; avoid speculative mappings. |
| P2 | Instantiate drift triage template when first real source update mismatch appears | `.planning/phases/*` | Run drift report workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi/src/lib.rs`
