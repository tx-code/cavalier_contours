# Phase 80: C-API Logic Alignment Map

This map captures next steps after shape polyline accessor invalid-input
contract coverage hardening.

## Deepening Outcome

- Shape polyline accessor functions (`count`, `is_closed`, `vertex_data`) now
  have direct API-level null-shape (`1`) and out-of-bounds (`2`) assertions.
- Failure-path output sentinel stability is now explicitly asserted for count,
  is_closed, and vertex buffer outputs when calls fail early.
- FFI runtime/header docs are aligned on shape parameter naming and cw/ccw
  wording for the covered accessor surfaces.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI APIs with documented specific error codes for missing output-stability assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Validate shape-surface docs for any residual parameter naming drift in runtime/header comments | `cavalier_contours_ffi/src/lib.rs`, `cavalier_contours_ffi.h` | Wording-only edits unless a real behavior mismatch is discovered. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
