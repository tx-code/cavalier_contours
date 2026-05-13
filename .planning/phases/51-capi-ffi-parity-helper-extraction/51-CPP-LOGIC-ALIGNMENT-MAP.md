# Phase 51: C-API Logic Alignment Map

This map captures next steps after FFI parity helper extraction.

## Deepening Outcome

- Shared helper constructs now centralize options-path setup for parity tests:
  `CPP_TOLERANCE_SCALE_MATRIX`, `CPP_SELF_INTERSECTS_INCLUDE_MODES`, and
  `init_parallel_offset_options`.
- Existing parity/no-modify assertions remain unchanged in semantics.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Reversed-input options-path output/no-modify merge matrix | `cavalier_contours_ffi/tests/test_pline.rs` | Merge reversed parity and no-modify checks only if readability and failure diagnostics remain clear. |
| P2 | Extend reversed matrix with source-backed edge-case attributions | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-explicit edge cases with clear provenance from old C++ suite. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`




