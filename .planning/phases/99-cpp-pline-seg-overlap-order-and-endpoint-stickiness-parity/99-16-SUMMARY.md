---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 16
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, dedup, wrap-around, arc-adjacent]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: line-line wrap-around dedup parity probes
provides:
  - mixed line/arc wrap-around dedup parity evidence
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-16-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use arc-adjacent wrap-around geometry that keeps overlap on the closing segment and avoids unrelated closure-edge crossings."
requirements-completed: [PAR-276]
duration: 10min
completed: 2026-05-14
---

# Plan 99-16 Summary

## Completed

- Added mixed line/arc wrap-around endpoint-dedup probes:
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline1`
  - `wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline2`
- Both probes assert one overlap and zero basics, validating dedup at vertex
  `0` when overlap includes the closing-segment endpoint.
- Refreshed `99-CPP-LOGIC-ALIGNMENT-MAP.md` to capture this coverage and set
  the next P1 target to arc/arc wrap-around variants.

## Verification

- `cargo test -p cavalier_contours wrap_around_overlap_endpoint_arc_adjacent_deduplication_closed_pline -q` - pass.
- `cargo test --workspace -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo clippy --all-targets -- -D warnings` - pass.
- `git diff --check` - pass.
- `gsd-sdk query state.validate --json --workspace E:/Coding/cavalier_contours` - valid.
- `gsd-sdk query validate.health --json --workspace E:/Coding/cavalier_contours` - healthy.
