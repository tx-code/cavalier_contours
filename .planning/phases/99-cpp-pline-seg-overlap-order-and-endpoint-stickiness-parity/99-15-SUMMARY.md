---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 15
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, dedup, wrap-around, closed-open]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: overlap-adjacent duplicate-filter baseline probes
provides:
  - wrap-around dedup parity evidence at closure boundary
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-15-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use line-line geometry at the closure boundary to isolate duplicate filtering and avoid independent closure-edge crossings."
requirements-completed: [PAR-275]
duration: 9min
completed: 2026-05-14
---

# Plan 99-15 Summary

## Completed

- Added wrap-around endpoint-dedup collection-level probes:
  - `wrap_around_overlap_endpoint_deduplication_closed_pline1`
  - `wrap_around_overlap_endpoint_deduplication_closed_pline2`
- Both probes hit overlap on the closing segment and verify the adjacent basic
  intersection at vertex `0` is removed via wrap-around duplicate filtering.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` to record this new boundary and the
  next P1 target.

## Verification

- `cargo test -p cavalier_contours wrap_around_overlap_endpoint_deduplication_closed_pline -q` - pass.
- `cargo test --workspace -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo clippy --all-targets -- -D warnings` - pass.
- `git diff --check` - pass.
- `gsd-sdk query state.validate --json --workspace E:/Coding/cavalier_contours` - valid.
- `gsd-sdk query validate.health --json --workspace E:/Coding/cavalier_contours` - healthy.
