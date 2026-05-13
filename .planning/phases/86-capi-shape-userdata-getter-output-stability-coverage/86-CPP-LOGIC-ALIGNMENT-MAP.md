# Phase 86: C-API Logic Alignment Map

This map captures next steps after shape userdata getter output stability
coverage hardening.

## Deepening Outcome

- Shape userdata getter API failure-path contracts now have direct assertions for:
  - null input on `cavc_shape_get_ccw_pline_userdata_count`
  - null/OOB input on `cavc_shape_get_ccw_pline_userdata_values`
  - null input on `cavc_shape_get_cw_pline_userdata_count`
  - null/OOB input on `cavc_shape_get_cw_pline_userdata_values`
- Failure-path output sentinel stability is now explicitly asserted for scalar
  and userdata-buffer out parameters on covered shape userdata getter surfaces.
- Shape userdata getter boundary behavior now has stronger regression resistance
  against accidental output mutation on early returns.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining shape/pline FFI APIs with `Specific Error Codes` for missing failure-path output stability assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Extend sentinel checks to remaining out-parameter surfaces that currently validate only return codes | `cavalier_contours_ffi/tests/test_pline.rs` | Keep deterministic API-level assertions only. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
