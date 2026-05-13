# Phase 91: C-API Logic Alignment Map

This map captures next steps after boolean invalid-operation options-path output
stability coverage hardening.

## Deepening Outcome

- Boolean invalid-operation API contracts now have explicit options-path
  assertions proving error return behavior and output-pointer stability match
  default-path expectations.
- Contains invalid-input API contracts now include explicit options-path
  null-result-pointer assertions for bounded invalid-input behavior.
- Options-path invalid-input behavior now has stronger regression resistance
  against drift between default and explicit-options boundary contracts.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI APIs with `Specific Error Codes` for missing failure-path output stability assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Extend options-path invariance checks to remaining invalid-input branches that currently validate only default-path behavior | `cavalier_contours_ffi/tests/test_pline.rs` | Keep deterministic API-level assertions only. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
