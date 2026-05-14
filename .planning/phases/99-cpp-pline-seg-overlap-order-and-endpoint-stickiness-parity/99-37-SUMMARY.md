---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 37
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, open-path, both-reverse, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: zero-length-lead non-zero-index role-flip guard for bounded arc1-reverse open-path branch
provides:
  - zero-length-lead non-zero-index role-flip guard for bounded both-reverse open-path branch
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-37-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use zero-bulge duplicate lead vertices before original first arc vertices so overlap geometry remains unchanged while overlap start indexes shift to non-zero."
requirements-completed: [PAR-297]
duration: 10min
completed: 2026-05-15
---

# Plan 99-37 Summary

## Completed

- Added collection-level non-zero-index role-flip guard:
  - `non_circle_partial_arc_overlap_both_reverse_dir_with_adjacent_line_flip_zero_length_lead_role_flip_symmetry`
- The probe prepends zero-length leads and verifies AB/BA role inversion still
  keeps:
  - one overlap and zero basic intersects on both sides,
  - role-swapped overlap start-index mapping,
  - non-zero overlap start indexes,
  - stable overlap endpoint sets (order may reverse).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with non-zero-index open-path
  evidence for the `both reverse` branch family.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_both_reverse_dir_with_adjacent_line_flip_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
