# Phase 84: C-API Logic Alignment Map

This map captures next steps after pline-eval failure-path output stability
coverage hardening.

## Deepening Outcome

- Pline-eval API failure-path contracts now have direct assertions for:
  - null input on `cavc_pline_eval_path_length`
  - null input on `cavc_pline_eval_area`
  - null input on `cavc_pline_eval_wn`
  - null input on `cavc_pline_eval_extents`
  - null/empty input on `cavc_pline_eval_closest_point`
- Failure-path output sentinel stability is now explicitly asserted for scalar,
  point, and index out parameters on covered pline-eval surfaces.
- Pline-eval boundary behavior now has stronger regression resistance against
  accidental output mutation on early returns.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI APIs with `Specific Error Codes` for missing failure-path output stability assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Extend sentinel checks to remaining out-parameter surfaces that currently verify only return codes | `cavalier_contours_ffi/tests/test_pline.rs` | Keep deterministic API-level assertions only. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
