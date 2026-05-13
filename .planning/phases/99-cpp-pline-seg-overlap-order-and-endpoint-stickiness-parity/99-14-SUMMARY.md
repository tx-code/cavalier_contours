---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 14
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, ordering, closed-open]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: reversed-overlap-endpoint-order non-circle baseline probe
provides:
  - closed/open reversed-order parity evidence with closure-edge outcomes
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-14-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Treat closure-edge intersections as explicit expected basics when they are independent geometry events, not overlap-adjacent dedup artifacts."
requirements-completed: [PAR-274]
duration: 12min
completed: 2026-05-14
---

# Plan 99-14 Summary

## Completed

- Added closed/open variants for reversed-overlap-endpoint-order non-circle
  probe:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_intersect`
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_intersect`
- Added explicit expected basic-intersect assertions for closure-edge crossings:
  - `start_index1=2, start_index2=1, point=(2.0, -1.0)` for closed `pline1`
  - `start_index1=0, start_index2=3, point=(2.0, 0.0)` for closed `pline2`
- Refreshed `99-CPP-LOGIC-ALIGNMENT-MAP.md` to record closure-edge behavior and
  next bounded target.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap -q` - pass.
- `cargo test --workspace -q` - pass.
- `cargo fmt --all` - pass.
- `cargo fmt --all --check` - pass.
- `cargo clippy --all-targets -- -D warnings` - pass.
- `git diff --check` - pass.
- `gsd-sdk query state.validate --json --workspace E:/Coding/cavalier_contours` - valid.
- `gsd-sdk query validate.health --json --workspace E:/Coding/cavalier_contours` - healthy.
