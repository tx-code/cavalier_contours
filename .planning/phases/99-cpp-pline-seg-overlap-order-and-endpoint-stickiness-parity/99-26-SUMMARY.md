---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 26
subsystem: segment-level-parity
tags: [cpp-parity, pline-seg-intersect, line-arc, two-intersect, sweep-classification]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: line-arc tangent sweep filtering probes
provides:
  - line-arc two-intersect sweep filtering parity evidence without sticky confounds
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-26-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_pline_seg_intersect.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use non-endpoint line segments against quarter-circle arcs so `numIntersects==2` branch coverage is isolated from endpoint-stickiness substitution."
requirements-completed: [PAR-286]
duration: 17min
completed: 2026-05-15
---

# Plan 99-26 Summary

## Completed

- Added source-aligned probes for old C++ `intrPlineSegs`
  `processLineArcIntr` `numIntersects == 2` sweep classification without line-endpoint
  stickiness:
  - `line_arc_two_intersections_only_one_in_sweep_non_sticky`
  - `arc_line_two_intersections_only_one_in_sweep_non_sticky`
  - `line_arc_two_intersections_both_outside_sweep_no_intersect`
  - `arc_line_two_intersections_both_outside_sweep_no_intersect`
- The probes lock:
  - one-in-sweep filtering to `OneIntersect`,
  - both-outside-sweep filtering to `NoIntersect`,
  in both line-arc and arc-line dispatch paths.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this branch-evidence batch.

## Verification

- `cargo test --workspace --test test_pline_seg_intersect line_arc_two_intersections_only_one_in_sweep_non_sticky -- --exact` - pass.
- `cargo test --workspace --test test_pline_seg_intersect arc_line_two_intersections_only_one_in_sweep_non_sticky -- --exact` - pass.
- `cargo test --workspace --test test_pline_seg_intersect line_arc_two_intersections_both_outside_sweep_no_intersect -- --exact` - pass.
- `cargo test --workspace --test test_pline_seg_intersect arc_line_two_intersections_both_outside_sweep_no_intersect -- --exact` - pass.
- `cargo test -p cavalier_contours line_arc_two_intersections_ -q` - pass.
- `cargo test -p cavalier_contours arc_line_two_intersections_ -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace` - pass.
- `gsd-sdk query validate.health` - healthy (info-only before summary creation), then healthy after summary creation.
