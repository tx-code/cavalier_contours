# Phase 47: C-API Logic Alignment Map

This map captures next steps after self-intersects mode no-modify matrix
deepening.

## Deepening Outcome

- Self-intersects include modes (`ALL`, `LOCAL`, `GLOBAL`) now have input
  no-modify matrix coverage across source-backed simple and specific offset
  cases.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Options-path stress matrix expansion | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-explicit stress cases with concrete legacy provenance. |
| P2 | FFI parity helper extraction | `cavalier_contours_ffi/tests/test_pline.rs` | Refactor only when semantic behavior is unchanged. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
