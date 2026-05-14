---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 38
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, closed-path, reversed-endpoint-order, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: zero-length-lead non-zero-index role-flip guard for bounded both-reverse open-path branch
provides:
  - zero-length-lead non-zero-index role-flip guard for bounded both-closed reversed-endpoint-order branch
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-38-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "For both-closed reversed-endpoint-order branch, keep endpoint-order assertion directional (AB reversed vs BA) while enforcing role-swapped index mapping under zero-length lead shift."
requirements-completed: [PAR-298]
duration: 9min
completed: 2026-05-15
---

# Plan 99-38 Summary

## Completed

- Added collection-level non-zero-index role-flip guard:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_zero_length_lead_role_flip_symmetry`
- The probe prepends zero-length leads and verifies AB/BA role inversion still
  keeps:
  - one overlap and three basic intersects on both sides,
  - role-swapped start-index mapping for both basic and overlap intersects,
  - non-zero overlap start indexes,
  - source-aligned overlap endpoint reversal (`AB point1/point2 == BA point2/point1`).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this both-closed non-zero-index
  role-flip evidence.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_both_closed_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
