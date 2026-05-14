---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 23
subsystem: segment-level-parity
tags: [cpp-parity, pline-seg-intersect, circle-circle, sweep-classification]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: one-sweep circle-circle two-intersection parity probes
provides:
  - both-in-sweep and both-outside-sweep circle-circle branch parity evidence
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-23-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_pline_seg_intersect.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use two equal-radius offset circles with semicircle sweeps to deterministically exercise both-in and both-outside classification without endpoint-stickiness ambiguity."
requirements-completed: [PAR-283]
duration: 14min
completed: 2026-05-15
---

# Plan 99-23 Summary

## Completed

- Added segment-level probes for old C++ `intrPlineSegs`
  `Circle2Circle2IntrType::TwoIntersects` sweep extremes:
  - `arc_arc_two_circle_intersections_both_in_sweeps`
  - `arc_arc_two_circle_intersections_both_in_sweeps_flipped_roles`
  - `arc_arc_two_circle_intersections_both_outside_sweeps_no_intersect`
  - `arc_arc_two_circle_intersections_both_outside_sweeps_no_intersect_flipped_roles`
- The new probes explicitly lock:
  - `TwoIntersects` when both geometric circle intersection points are inside
    both arc sweeps,
  - `NoIntersect` when both geometric circle intersection points are outside
    both sweeps,
  - parameter-role inversion behavior for both families.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` to record this new branch coverage.

## Verification

- `cargo test --workspace --test test_pline_seg_intersect arc_arc_two_circle_intersections_both_in_sweeps -- --exact` - pass.
- `cargo test --workspace --test test_pline_seg_intersect arc_arc_two_circle_intersections_both_in_sweeps_flipped_roles -- --exact` - pass.
- `cargo test --workspace --test test_pline_seg_intersect arc_arc_two_circle_intersections_both_outside_sweeps_no_intersect -- --exact` - pass.
- `cargo test --workspace --test test_pline_seg_intersect arc_arc_two_circle_intersections_both_outside_sweeps_no_intersect_flipped_roles -- --exact` - pass.
- `cargo test -p cavalier_contours arc_arc_two_circle_intersections_ -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace` - pass.
- `gsd-sdk query validate.health` - healthy (info-only before summary creation), then healthy after summary creation.
