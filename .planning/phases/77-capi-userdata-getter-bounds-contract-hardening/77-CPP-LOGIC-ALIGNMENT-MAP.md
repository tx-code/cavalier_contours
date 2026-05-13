# Phase 77: C-API Logic Alignment Map

This map captures next steps after userdata getter bounds-contract hardening.

## Deepening Outcome

- CCW/CW userdata getters now return explicit out-of-bounds error code `2`
  instead of relying on implicit panic/unwind behavior.
- Header docs and tests are aligned to the explicit getter error contract
  (`shape` null => `1`, index out-of-bounds => `2`).
- C-API userdata read contract is now behavior-consistent across setter/count/get
  surfaces.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI getters for implicit panic-bound paths and harden with explicit error codes where contract expects it | `cavalier_contours_ffi/src/lib.rs` | Change only where behavior aligns with established API error-code patterns. |
| P1 | Expand direct behavior tests for newly hardened getter/setter contracts | `cavalier_contours_ffi/tests/test_pline.rs` | Prefer deterministic API-level assertions over indirect behavior inference. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
