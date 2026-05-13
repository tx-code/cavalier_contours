---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 17
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, dedup, wrap-around, non-circle, ordering]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: mixed line/arc wrap-around dedup parity evidence
provides:
  - non-circle arc/arc wrap-around dedup parity evidence for endpoint ordering
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-17-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use `closed_pline1` geometry where overlap lies on closing segment and avoid independent closure-edge crossings, keeping `basic_intersects` emptiness a valid dedup assertion."
requirements-completed: [PAR-277]
duration: 11min
completed: 2026-05-14
---

# Plan 99-17 Summary

## Completed

- Added non-circle arc/arc wrap-around endpoint-dedup probes:
  - `wrap_around_non_circle_arc_overlap_deduplication_same_order_closed_pline1`
  - `wrap_around_non_circle_arc_overlap_deduplication_reversed_order_closed_pline1`
- Both probes assert:
  - one overlap intersect on closing segment (`start_index1 = 2`)
  - zero basic intersects after duplicate filtering
  - expected overlap endpoint ordering according to second-segment direction
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this new coverage and next P1.

## Verification

- `cargo test -p cavalier_contours wrap_around_non_circle_arc_overlap_deduplication -q` - pass.
- `cargo test --workspace -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo clippy --all-targets -- -D warnings` - pass.
- `git diff --check` - pass.
- `gsd-sdk query state.validate --json --workspace E:/Coding/cavalier_contours` - valid.
- `gsd-sdk query validate.health --json --workspace E:/Coding/cavalier_contours` - healthy.
