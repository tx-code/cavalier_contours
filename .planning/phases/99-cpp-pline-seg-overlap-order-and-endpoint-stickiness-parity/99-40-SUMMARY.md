---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 40
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, closed-path, both-reverse, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: zero-length-lead non-zero-index role-flip guard for bounded both-closed arc1-reverse branch
provides:
  - zero-length-lead non-zero-index role-flip guard for bounded both-closed both-reverse branch
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-40-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Preserve overlap endpoint-order stability expectations under role inversion for the both-reverse closed branch while shifting indices via zero-length leads."
requirements-completed: [PAR-300]
duration: 8min
completed: 2026-05-15
---

# Plan 99-40 Summary

## Completed

- Added collection-level non-zero-index role-flip guard:
  - `non_circle_partial_arc_overlap_both_reverse_dir_both_closed_zero_length_lead_role_flip_symmetry`
- The probe prepends zero-length leads and verifies AB/BA role inversion still
  keeps:
  - one overlap and one basic intersect on both sides,
  - role-swapped start-index mapping for both basic and overlap intersects,
  - non-zero start indexes for both intersect kinds,
  - source-aligned overlap endpoint-order stability (`AB point1/point2 == BA point1/point2`).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this both-closed non-zero-index
  role-flip evidence for the `both reverse` branch family.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_both_reverse_dir_both_closed_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
