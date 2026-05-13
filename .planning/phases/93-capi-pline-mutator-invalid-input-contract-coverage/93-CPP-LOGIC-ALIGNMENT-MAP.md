# Phase 93: C-API Logic Alignment Map

This map captures next steps after pline mutator invalid-input contract
coverage hardening.

## Deepening Outcome

- Pline mutator API null-input contracts now have direct assertions for:
  - `cavc_pline_set_vertex_data`
  - `cavc_pline_set_is_closed`
  - `cavc_pline_clear`
  - `cavc_pline_set_vertex`
  - `cavc_pline_remove`
- Pline mutator OOB contracts now have explicit assertions for:
  - `cavc_pline_set_vertex`
  - `cavc_pline_remove`
- Pline mutator invalid-input behavior now has stronger regression resistance
  against silent drift in return-code contracts.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Audit remaining FFI APIs with `Specific Error Codes` for missing invalid-input or failure-path output-stability assertions | `cavalier_contours_ffi/tests/test_pline.rs` | Add tests only; avoid geometry algorithm changes. |
| P1 | Extend default/options-path invariance checks for remaining invalid-input branches that currently validate only one path | `cavalier_contours_ffi/tests/test_pline.rs` | Keep deterministic API-level assertions only. |
| P2 | Instantiate drift triage template on first real source update mismatch | `.planning/phases/*` | Run drift workflow only on real source delta. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/src/lib.rs`
  - `cavalier_contours_ffi/tests/test_pline.rs`
  - `cavalier_contours_ffi.h`
