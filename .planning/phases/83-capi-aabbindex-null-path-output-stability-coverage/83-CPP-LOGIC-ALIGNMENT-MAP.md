# Phase 83: C-API Logic Alignment Map

This map captures next steps after aabbindex null-path output stability
coverage hardening.

## Deepening Outcome

- Aabbindex API failure-path contracts now have direct null-input assertions for:
  - `cavc_pline_create_approx_aabbindex`
  - `cavc_pline_create_aabbindex`
  - `cavc_aabbindex_get_extents`
- Null-path output sentinel stability is now explicitly asserted for both
  aabbindex pointer out parameters and extents scalar out parameters.
- Aabbindex boundary behavior now has stronger regression resistance against
  accidental output mutation on early returns.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI APIs with `Specific Error Codes` for missing failure-path output stability assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Expand null-path output stability checks for remaining nullable scalar/pointer out-parameter surfaces | `cavalier_contours_ffi/tests/test_pline.rs` | Keep deterministic API-level assertions only. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
