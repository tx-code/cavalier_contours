---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 41
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, closed-path, arc2-reverse, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: zero-length-lead non-zero-index role-flip guard for bounded both-closed both-reverse branch
provides:
  - zero-length-lead non-zero-index role-flip guard for bounded both-closed arc2-reverse branch
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-41-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Maintain the no-basic branch for arc2-reverse closed geometry under zero-length lead shift by preserving original arc bulges on the second vertex and using zero-bulge duplicate leads."
requirements-completed: [PAR-301]
duration: 8min
completed: 2026-05-15
---

# Plan 99-41 Summary

## Completed

- Added collection-level non-zero-index role-flip guard:
  - `non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_zero_length_lead_role_flip_symmetry`
- The probe prepends zero-length leads and verifies AB/BA role inversion still
  keeps:
  - one overlap and zero basic intersects on both sides,
  - role-swapped overlap start-index mapping,
  - non-zero overlap start indexes,
  - source-aligned overlap endpoint reversal (`AB point1/point2 == BA point2/point1`).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this both-closed non-zero-index
  role-flip evidence for the `arc2 reverse` branch family.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_arc2_reverse_dir_both_closed_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
