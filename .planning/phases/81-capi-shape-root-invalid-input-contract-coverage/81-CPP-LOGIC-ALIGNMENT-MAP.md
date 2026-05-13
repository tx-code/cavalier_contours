# Phase 81: C-API Logic Alignment Map

This map captures next steps after shape-root invalid-input contract coverage
hardening.

## Deepening Outcome

- Shape-root functions now have direct API-level null-input contract assertions
  for error code `1`:
  - `cavc_shape_create`
  - `cavc_shape_parallel_offset`
  - `cavc_shape_get_ccw_count`
  - `cavc_shape_get_cw_count`
- Failure-path output sentinel stability is now explicitly asserted for root
  shape pointer outputs and count outputs under null-input contracts.
- Shape-surface runtime/header docs are aligned on `cavc_shape_create`
  references in covered shape sections.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI APIs with `Specific Error Codes` for missing failure-path output stability assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Scan shape-surface runtime/header docs for residual wording drift beyond covered root/accessor sections | `cavalier_contours_ffi/src/lib.rs`, `cavalier_contours_ffi.h` | Wording-only edits unless a real behavior mismatch is discovered. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
