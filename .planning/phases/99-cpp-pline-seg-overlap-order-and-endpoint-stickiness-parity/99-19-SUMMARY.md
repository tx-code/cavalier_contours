---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 19
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, wrap-around, non-circle, closure-edge]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: non-circle arc/arc wrap-around dedup baseline coverage
provides:
  - closure-edge expected-basic parity evidence for wrap-around arc/arc probes
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-19-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Keep wrap-around overlap intact while intentionally routing a support edge through a known independent crossing and asserting the resulting basic intersect."
requirements-completed: [PAR-279]
duration: 8min
completed: 2026-05-14
---

# Plan 99-19 Summary

## Completed

- Added wrap-around non-circle arc/arc closure-edge variants (closed `pline1`):
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline1_with_closure_basic_intersect`
  - `wrap_around_non_circle_arc_overlap_reversed_order_closed_pline1_with_closure_basic_intersect`
- Both variants assert:
  - one overlap intersect on wrap-around segment (`start_index1 = 2`)
  - one independent basic intersect at `(2.0, 2.0)` with explicit start indexes
  - expected overlap endpoint ordering for same/reversed second-segment direction
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` and advanced next P1 target.

## Verification

- `cargo test -p cavalier_contours closure_basic_intersect -q` - pass.
- `cargo test --workspace -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo clippy --all-targets -- -D warnings` - pass.
- `git diff --check` - pass.
- `gsd-sdk query state.validate --json --workspace E:/Coding/cavalier_contours` - valid.
- `gsd-sdk query validate.health --json --workspace E:/Coding/cavalier_contours` - healthy.
