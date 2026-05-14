---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 22
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, wrap-around, non-circle, closure-edge, ordering]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: open-side-reversed + closed-side-reversed role-flip symmetry probes
provides:
  - closed-side-reversed closure-basic expected-case parity evidence
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-22-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Promote open-side-reversed + closed-side-reversed geometry from symmetry-only probes to explicit expected-case baselines with segment-index assertions."
requirements-completed: [PAR-282]
duration: 12min
completed: 2026-05-15
---

# Plan 99-22 Summary

## Completed

- Added closed-side-reversed closure-basic expected-case probes:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closed_side_reversed_closure_basic_intersect`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closed_side_reversed_closure_basic_intersect_flipped_roles`
- Both probes assert:
  - one overlap on the closure-edge segment with reversed endpoint ordering
    `(3, 1) -> (2, 0)`
  - one independent basic intersect at `(2, 2)`
  - explicit segment indexes in base and parameter-role-flipped calls.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this additional bounded P1 coverage.

## Verification

- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closed_side_reversed_closure_basic_intersect -q` - pass.
- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closed_side_reversed_closure_basic_intersect_flipped_roles -q` - pass.
- `cargo test -p cavalier_contours closure_basic_intersect -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace` - pass.
- `gsd-sdk query validate.health` - healthy (info-only before summary creation), then healthy after summary creation.
