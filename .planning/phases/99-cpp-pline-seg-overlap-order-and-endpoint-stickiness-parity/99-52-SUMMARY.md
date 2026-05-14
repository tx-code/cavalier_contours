---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 52
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, both-closed, mixed-line-arc, adjacent-dedup, closed-pline1-rotation, closed-pline2-rotation, nonzero-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: both-closed mixed line/arc adjacent-dedup role-flip symmetry guards for explicit closed-pline1/2 start-index-rotated branches
provides:
  - zero-length-lead non-zero-index role-flip guards for both-closed mixed line/arc adjacent-dedup closed-pline1/2-rotated branches
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-52-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use duplicate non-overlap vertices on the rotated closed side in zero-length-lead variants so the branch remains one-overlap/zero-basic and only exercises non-zero index-shift behavior."
requirements-completed: [PAR-273]
duration: 8min
completed: 2026-05-15
---

# Plan 99-52 Summary

## Completed

- Added collection-level non-zero-index role-flip guards:
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry`
  - `overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry`
- These probes verify AB/BA role inversion under zero-length lead index shift
  still keeps:
  - one overlap + zero basic intersects on both sides,
  - explicit non-zero start-index attribution on the rotated side,
  - stable overlap endpoint ordering consistent with this branch family.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with closed-pline1/2-rotated
  zero-length-lead evidence for this mixed line/arc both-closed adjacent-dedup branch.

## Verification

- `cargo test -p cavalier_contours overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_closed_pline2_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo test -p cavalier_contours overlap_endpoint_arc_adjacent_basic_intersect_deduplication_both_closed_start_index_rotation_closed_pline1_zero_length_lead_role_flip_symmetry -q` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
