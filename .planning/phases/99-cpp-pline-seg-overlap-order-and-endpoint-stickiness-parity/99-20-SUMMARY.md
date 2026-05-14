---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 20
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, wrap-around, non-circle, closure-edge]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: closed-pline1 closure-edge wrap-around arc/arc probes
provides:
  - closed-pline2 closure-edge wrap-around parity evidence
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-20-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Mirror stable closed-pline1 closure-edge geometry into closed-pline2 variants and keep explicit independent-basic assertions to avoid conflating real crossings with dedup behavior."
requirements-completed: [PAR-280]
duration: 12min
completed: 2026-05-14
---

# Plan 99-20 Summary

## Completed

- Added closed-`pline2` closure-edge wrap-around probes:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect`
- Both probes assert:
  - one overlap on wrap-around segment (`start_index2 = 2`)
  - one independent basic intersect at `(2.0, 2.0)` with explicit segment indexes
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` and advanced next P1 target.

## Verification

- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect -q` - pass.
- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect -q` - pass.
- `cargo test --workspace -q` - pass.
- `cargo fmt --all` - pass.
- `cargo fmt --all --check` - pass.
- `cargo clippy --all-targets -- -D warnings` - pass.
- `git diff --check` - pass.
- `gsd-sdk query state.validate --json --workspace E:/Coding/cavalier_contours` - valid.
- `gsd-sdk query validate.health --json --workspace E:/Coding/cavalier_contours` - healthy.
