# Phase 99: C++ Logic Alignment Map

This map captures next deep parity targets after pline segment overlap-order and
endpoint-stickiness branch closure.

## Deepening Outcome

- Added executable expected-case parity for old C++ `intrPlineSegs` branch
  families:
  - line-line overlap-order according to second segment direction
  - line-arc endpoint-stickiness path
  - arc-line endpoint-stickiness path
  - two-intersect ordering according to second segment direction in line-arc
    and arc-line paths
- Intersect point outputs are explicitly asserted as parity evidence.
- Added bounded collection-level parity for overlap-adjacent duplicate-filter
  behavior in `find_intersects`:
  - `overlap_endpoint_basic_intersect_deduplication`
- Added bounded collection-level parity for `skip_intr_at_end` endpoint-elision
  symmetry in open vs closed paths:
  - `skip_intr_at_end_open_pline1_uses_next_segment_index`
  - `skip_intr_at_end_closed_pline1_uses_next_segment_index`
  - `skip_intr_at_end_open_pline2_uses_next_segment_index`
  - `skip_intr_at_end_closed_pline2_uses_next_segment_index`

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Add bounded mixed line/arc overlap-adjacent collection-level probes for duplicate-filter and endpoint attribution | `cavalier_contours/src/polyline/internal/pline_intersects.rs`, `cavalier_contours/tests/test_pline_seg_intersect.rs` | Add parity tests first; only touch core on confirmed mismatch. |
| P1 | Add closed/open asymmetry probes where endpoint-elision competes with arc overlap ordering | `cavalier_contours/src/polyline/internal/pline_intersects.rs`, `cavalier_contours/tests/test_pline_seg_intersect.rs` | Keep source-traceable cases and avoid speculative refactors. |
| P2 | Extend combine/offset-derived intersection fixture parity only when direct C++ source mapping exists | `cavalier_contours/tests/test_cpp_combine_parity.rs`, `cavalier_contours/tests/test_cpp_offset_parity.rs` | Keep provenance explicit and no-Clipper. |

## File-Level Alignment Surface

- C++ reference:
  - `E:/Coding/CavalierContours/include/cavc/plinesegment.hpp`
  - `E:/Coding/CavalierContours/include/cavc/polylineintersects.hpp`
- Rust parity tests:
  - `cavalier_contours/tests/test_pline_seg_intersect.rs`
  - `cavalier_contours/src/polyline/internal/pline_intersects.rs`
