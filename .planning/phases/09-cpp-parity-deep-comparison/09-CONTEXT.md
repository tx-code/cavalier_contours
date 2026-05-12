# Phase 09: cpp-parity-deep-comparison - Context

**Gathered:** 2026-05-12  
**Status:** Ready for planning/execution

<domain>
## Phase Boundary

This phase performs deep logic comparison between old C++ CavalierContours and
Rust `cavalier_contours`, explicitly excluding Clipper-driven oracle work.
Focus areas: boolean/combine logic, offset logic, intersection primitives, and
observable topology behavior.

</domain>

<decisions>
## Implementation Decisions

- **D-01:** Prioritize C++ `TEST_cavc_combine_plines.cpp` parity first, because
  known metadata-only gap exists.
- **D-02:** Promote metadata-only C++ cases to executable Rust parity cases when
  deterministic mapping exists.
- **D-03:** Distinguish parity outcomes as `bug`, `intentional-divergence`, or
  `not-comparable` with explicit evidence.
- **D-04:** Keep Clipper out of this phase; no new Clipper-based fixtures here.
- **D-05:** If parity truth is unclear, record candidate third-party geometry
  libraries (e.g., Boost.Geometry / CGAL / GEOS bindings) for tie-break checks,
  but do not block on them in 09-01.
- **D-06:** Preserve safe Rust policy and keep API/FFI stable unless a confirmed
  bug fix requires a scoped change.

</decisions>

<canonical_refs>
## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/03-historical-c-evidence-mining/03-INVENTORY.md`
- `.planning/phases/06-robustness-gap-closure/06-ROBUSTNESS-BACKLOG.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
- `E:/Coding/CavalierContours/include/cavc/polylinecombine.hpp`
- `E:/Coding/CavalierContours/include/cavc/polylineoffset.hpp`
- `E:/Coding/CavalierContours/include/cavc/polylineintersects.hpp`
- `cavalier_contours/src/polyline/internal/pline_boolean.rs`
- `cavalier_contours/src/polyline/internal/pline_offset.rs`
- `cavalier_contours/src/polyline/internal/pline_intersects.rs`
- `cavalier_contours/tests/test_historical_cavalier_contours.rs`
- `cavalier_contours/tests/test_pline_boolean.rs`

</canonical_refs>

---

*Phase: 09-cpp-parity-deep-comparison*
