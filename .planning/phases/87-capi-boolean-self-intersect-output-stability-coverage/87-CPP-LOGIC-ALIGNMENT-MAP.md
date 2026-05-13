# Phase 87: C-API Logic Alignment Map

This map captures next steps after boolean/self-intersect output stability
coverage hardening.

## Deepening Outcome

- Boolean/self-intersect API failure-path contracts now have direct assertions for:
  - invalid operation and null input on `cavc_pline_boolean`
  - invalid options and null input on `cavc_pline_scan_for_self_intersect`
- Failure-path output sentinel stability is now explicitly asserted for list
  pointer outputs (`pos_plines`, `neg_plines`) and the self-intersect result
  flag (`is_self_intersecting`) on covered surfaces.
- Boolean/self-intersect boundary behavior now has stronger regression
  resistance against accidental output mutation on early returns.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI APIs with `Specific Error Codes` for missing failure-path output stability assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Extend sentinel checks to remaining out-parameter surfaces that currently validate only return codes | `cavalier_contours_ffi/tests/test_pline.rs` | Keep deterministic API-level assertions only. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
