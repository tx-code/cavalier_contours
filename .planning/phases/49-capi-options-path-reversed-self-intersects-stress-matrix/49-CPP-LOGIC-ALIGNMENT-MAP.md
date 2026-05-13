# Phase 49: C-API Logic Alignment Map

This map captures next steps after reversed options-path self-intersects stress
matrix deepening.

## Deepening Outcome

- Options-path `parallel_offset` now validates default-path parity across
  reversed inputs (`invert_direction + negated delta`), `handle_self_intersects`
  mode matrix (`ALL`, `LOCAL`, `GLOBAL`), and bounded tolerance scales
  (`0.5x`, `1.0x`, `2.0x`) for source-backed simple and specific cases.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Reversed-input options-path no-modify stress matrix | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-backed reversed no-modify checks (`invert_direction + negated delta`) with mode/tolerance options. |
| P2 | FFI parity helper extraction | `cavalier_contours_ffi/tests/test_pline.rs` | Refactor only when semantic behavior is unchanged and parity assertions stay identical. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`


