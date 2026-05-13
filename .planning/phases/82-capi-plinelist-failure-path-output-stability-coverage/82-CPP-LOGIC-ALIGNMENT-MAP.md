# Phase 82: C-API Logic Alignment Map

This map captures next steps after plinelist failure-path output stability
coverage hardening.

## Deepening Outcome

- Plinelist API failure-path contracts now have direct API-level assertions for:
  - null input (`cavc_plinelist_get_count/get_pline/pop/take`)
  - empty list path (`cavc_plinelist_get_pline/pop/take`)
  - out-of-bounds index path (`cavc_plinelist_get_pline/take`)
- Failure-path output sentinel stability is now explicitly asserted for both
  count and pline out parameters.
- Plinelist boundary behavior now has stronger regression resistance against
  accidental output mutation on early returns.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI APIs with `Specific Error Codes` for missing failure-path output stability assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Extend failure-path output stability checks to remaining nullable out-pointer surfaces with mixed success/failure contracts | `cavalier_contours_ffi/tests/test_pline.rs` | Keep deterministic API-level assertions only. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
