# Phase 79: C-API Logic Alignment Map

This map captures next steps after contains/extents invalid-input contract
coverage hardening.

## Deepening Outcome

- `cavc_pline_contains` now has direct API-level tests that assert null-input
  error code `1` and `CAVC_CONTAINS_RESULT_INVALID_INPUT` result write-back
  behavior when `result` is non-null.
- `cavc_pline_eval_extents` now has direct API-level tests that assert
  degenerate-input error code `2` and output sentinel stability on failure.
- Invalid-input contract behavior is now explicit at the API boundary for both
  contains and extents surfaces.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI functions with documented specific error codes for unasserted failure-path output behavior | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Verify header/runtime doc wording matches implemented invalid-input contracts on shape helper surfaces | `cavalier_contours_ffi/src/lib.rs`, `cavalier_contours_ffi.h` | Wording-only edits unless a real behavior mismatch is discovered. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
