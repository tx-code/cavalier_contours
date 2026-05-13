# Phase 78: C-API Logic Alignment Map

This map captures next steps after boolean/self-intersect error-contract
coverage hardening.

## Deepening Outcome

- `cavc_pline_boolean` now has direct API-level tests for invalid operation
  (`2`) and null pline input (`1`) behavior.
- `cavc_pline_scan_for_self_intersect` now has direct API-level tests for
  invalid options (`2`) and null pline input (`1`) behavior.
- Runtime/header docs are aligned on self-intersect null-input naming (`pline`)
  to match actual API contract wording.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI option-bearing entry points for invalid-enum/invalid-options direct error-code assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Verify null-input and output-pointer stability contracts on remaining boolean/intersect wrappers | `cavalier_contours_ffi/src/lib.rs` | Preserve existing semantics; only tighten explicit contracts. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
