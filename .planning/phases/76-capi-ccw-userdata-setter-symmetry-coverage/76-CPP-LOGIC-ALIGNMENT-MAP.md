# Phase 76: C-API Logic Alignment Map

This map captures next steps after CCW userdata setter symmetry hardening.

## Deepening Outcome

- `cavc_shape_set_ccw_pline_userdata_values` now has direct setter-behavior
  coverage for success, null-shape error, out-of-bounds error, and clear path.
- CCW setter contract coverage is now symmetric with previously covered CW
  setter behavior for count/value roundtrip and clear semantics.
- FFI setter behavior checks no longer depend only on downstream offset-output
  observations.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Re-run export-surface coverage scan and close any newly introduced uncovered behavior paths | `cavalier_contours_ffi/tests/test_pline.rs` | Add only deterministic API-behavior checks. |
| P1 | Continue adding explicit source/docs provenance labels for direct FFI behavior tests where mappings are concrete | `cavalier_contours_ffi/tests/test_pline.rs` | Keep provenance claims strictly evidence-backed. |
| P2 | Instantiate drift triage template when first real source update mismatch appears | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi/src/lib.rs`
