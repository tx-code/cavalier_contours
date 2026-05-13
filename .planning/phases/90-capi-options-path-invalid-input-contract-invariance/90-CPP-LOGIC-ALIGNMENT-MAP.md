# Phase 90: C-API Logic Alignment Map

This map captures next steps after options-path invalid-input contract
invariance coverage hardening.

## Deepening Outcome

- Boolean invalid-input API contracts now have explicit options-path assertions
  proving null-input return behavior and output-pointer stability match
  default-path contracts.
- Contains invalid-input API contracts now have explicit options-path assertions
  proving deterministic `CAVC_CONTAINS_RESULT_INVALID_INPUT` output writes match
  default-path contracts.
- Options-path invalid-input behavior now has stronger regression resistance
  against drift between default and explicit-options boundary contracts.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI APIs with `Specific Error Codes` for missing failure-path output stability assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Extend sentinel/invalid-output checks to remaining out-parameter surfaces that currently validate only return codes | `cavalier_contours_ffi/tests/test_pline.rs` | Keep deterministic API-level assertions only. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
