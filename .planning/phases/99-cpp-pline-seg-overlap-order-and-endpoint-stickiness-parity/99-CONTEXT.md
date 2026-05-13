# Phase 99: cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity - Context

**Gathered:** 2026-05-15  
**Status:** Ready for execution

## Phase Boundary

Phase 99 deepens old C++ polyline segment-intersection parity by adding
source-traceable branch evidence for `intrPlineSegs` behavior in
`plinesegment.hpp`:

- line-line overlap ordering by second segment direction
- line-arc and arc-line endpoint-stickiness behavior
- line-arc and arc-line two-intersect ordering by second segment direction

## Decisions

- **D-01:** Keep this phase scoped to Rust parity tests and planning artifacts.
- **D-02:** Add bounded branch-matrix assertions before any core geometry edit.
- **D-03:** Keep no-Clipper scope and preserve source-traceable expectations.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `cavalier_contours/tests/test_pline_seg_intersect.rs`
- `E:/Coding/CavalierContours/include/cavc/plinesegment.hpp`
- `E:/Coding/CavalierContours/include/cavc/polylineintersects.hpp`

