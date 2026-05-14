---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 35
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, open-path, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: collection-level role-flip symmetry guard for bounded reversed-endpoint-order open-path branch
provides:
  - zero-length-lead non-zero-index role-flip guard for bounded reversed-endpoint-order open-path branch
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-35-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use a zero-bulge duplicate lead vertex before the original first arc vertex so segment geometry is preserved while start indexes shift to non-zero."
requirements-completed: [PAR-295]
duration: 13min
completed: 2026-05-15
---

# Plan 99-35 Summary

## Completed

- Added collection-level non-zero-index role-flip guard:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_zero_length_lead_role_flip_symmetry`
- The probe prepends zero-length lead segments (with preserved arc bulge on the
  original first arc vertex) and verifies AB/BA role inversion still keeps:
  - one overlap and zero basic intersects on both sides,
  - role-swapped overlap start-index mapping,
  - non-zero overlap start indexes,
  - stable overlap endpoint sets (order may reverse).
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this non-zero-index open-path
  evidence for the `reversed endpoint order` branch family.

## Verification

- `cargo test -p cavalier_contours non_circle_partial_arc_overlap_reversed_endpoint_order_with_adjacent_line_flip_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
