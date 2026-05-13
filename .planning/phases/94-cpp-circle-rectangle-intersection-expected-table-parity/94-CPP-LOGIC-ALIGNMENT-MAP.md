# Phase 94: C++ Logic Alignment Map

This map captures next deep-parity targets after circle/rectangle intersection
expected-table closure.

## Deepening Outcome

- Circle/rectangle intersection parity for historical C++ geometry now asserts:
  - exact basic-intersect cardinality (`4`)
  - exact segment index-pair attribution
  - coordinate-level expected points
  - empty overlapping-intersect output
- Prior count-only snapshot risk is reduced by executable expected-table checks.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Extend expected-table parity to operand-order and direction-variant matrix for the same C++ circle/rectangle geometry | `cavalier_contours/tests/test_cpp_offset_parity.rs` | Keep source geometry fixed; add parity evidence only. |
| P1 | Build bounded standalone primitive intersection branch matrix parity from old C++ `intrlineseg2lineseg2.hpp`, `intrlineseg2circle2.hpp`, and `intrcircle2circle2.hpp` | `cavalier_contours/tests/test_line_line_intersect.rs`, `cavalier_contours/tests/test_line_circle_intersect.rs`, `cavalier_contours/tests/test_circle_circle_intersect.rs` | Evidence/test deepening only; no geometry kernel rewrite unless a confirmed bug appears. |
| P2 | If and only if a real drift appears, triage parity mismatch at `pline_intersects` collection layer with source-traceable reproducer | `cavalier_contours/src/polyline/internal/pline_intersects.rs`, `cavalier_contours/tests/*` | Trigger on real drift, not speculative changes. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
  - `E:/Coding/CavalierContours/include/cavc/intrlineseg2lineseg2.hpp`
  - `E:/Coding/CavalierContours/include/cavc/intrlineseg2circle2.hpp`
  - `E:/Coding/CavalierContours/include/cavc/intrcircle2circle2.hpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_cpp_offset_parity.rs`
  - `cavalier_contours/tests/test_line_line_intersect.rs`
  - `cavalier_contours/tests/test_line_circle_intersect.rs`
  - `cavalier_contours/tests/test_circle_circle_intersect.rs`

