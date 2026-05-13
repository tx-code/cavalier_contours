# Phase 55: C-API Logic Alignment Map

This map captures next steps after default-input options-path specific-edge
attribution matrix deepening.

## Deepening Outcome

- Default-input options-path specific cases now carry explicit legacy
  provenance attribution in assertion diagnostics.
- Specific-edge checks co-validate output parity and input stability across
  mode/scale matrix combinations.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Execute template-driven triage when drift hook first fails in real source update. |
| P1 | Merge default and reversed specific-edge runners with helper | `cavalier_contours_ffi/tests/test_pline.rs` | Refactor only if behavior and diagnostics remain unchanged. |
| P2 | Expand source-backed attribution notes in existing specific cases | `cavalier_contours_ffi/tests/test_pline.rs` | Keep attributions concise and directly tied to old C++ specific-case comments. |

## File-Level Alignment Surface

- Rust FFI:
  - `cavalier_contours_ffi/tests/test_pline.rs`
- C++ references:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`








