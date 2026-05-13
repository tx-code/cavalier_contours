# Phase 54: C-API Logic Alignment Map

This map captures next steps after default-input options-path output/no-modify
merge-matrix deepening.

## Deepening Outcome

- Default-input options-path stress validation now co-checks output parity and
  input stability in a single mode/scale matrix loop.
- Failure diagnostics remain mode/scale-attributed while reducing split-surface
  reasoning across separate tests.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Extend default-input merged matrix with source-backed edge-case attributions | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-explicit edge cases with clear provenance from old C++ suite. |
| P2 | Extract merged matrix runner helper | `cavalier_contours_ffi/tests/test_pline.rs` | Refactor only if behavior and failure diagnostics remain unchanged. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`







