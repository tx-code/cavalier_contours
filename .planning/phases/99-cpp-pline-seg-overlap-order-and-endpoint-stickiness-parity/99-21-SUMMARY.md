---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 21
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, wrap-around, non-circle, closure-edge, ordering]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: closed-pline2 closure-edge wrap-around baseline counterparts
provides:
  - closed-pline2 reversed-order closure-edge wrap-around parity evidence
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-21-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use a closed-pline2 support line crossing the open-side reversed arc case to preserve one independent basic while forcing reversed overlap endpoint ordering."
requirements-completed: [PAR-281]
duration: 10min
completed: 2026-05-14
---

# Plan 99-21 Summary

## Completed

- Added/stabilized closed-`pline2` closure-edge counterparts:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect`
- Confirmed the reversed closed-`pline2` variant asserts:
  - one overlap with `point1 = (3, 1)` and `point2 = (2, 0)`
  - one independent basic intersect at `(2, 2)`
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with the new closure-edge ordering evidence.

## Verification

- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect -q` - pass.
- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect -q` - pass.
- `cargo test -p cavalier_contours closure_basic_intersect -q` - pass.
- `cargo test --workspace -q` - pass.
- `cargo fmt --all` - pass.
- `cargo fmt --all --check` - pass.
- `cargo clippy --all-targets -- -D warnings` - pass.
- `git diff --check` - pass.
- `gsd-sdk query state.validate --json --workspace E:/Coding/cavalier_contours` - valid.
- `gsd-sdk query validate.health --json --workspace E:/Coding/cavalier_contours` - healthy.
