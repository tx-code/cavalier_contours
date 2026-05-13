---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 18
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, dedup, wrap-around, non-circle, ordering]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: non-circle arc/arc wrap-around probes with closed `pline1`
provides:
  - complementary closed `pline2` non-circle wrap-around dedup parity evidence
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-18-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Tune closed-`pline2` support geometry to prevent unrelated segment crossings so zero-basic dedup assertions remain meaningful."
requirements-completed: [PAR-278]
duration: 13min
completed: 2026-05-14
---

# Plan 99-18 Summary

## Completed

- Added complementary non-circle arc/arc wrap-around probes with closed
  `pline2`:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline2`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline2`
- Both tests assert:
  - one overlap intersect on `pline2` closing segment (`start_index2 = 2`)
  - zero remaining basic intersects after dedup
  - expected endpoint ordering by second-segment direction
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` and advanced next P1 boundary.

## Verification

- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_deduplication_ -q` - pass.
- `cargo test --workspace -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo clippy --all-targets -- -D warnings` - pass.
- `git diff --check` - pass.
- `gsd-sdk query state.validate --json --workspace E:/Coding/cavalier_contours` - valid.
- `gsd-sdk query validate.health --json --workspace E:/Coding/cavalier_contours` - healthy.
