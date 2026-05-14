---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 61
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, reversed-endpoint-order, closure-basic, role-flip, nonzero-open-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: start-index-rotation zero-length counterpart matrix closure
provides:
  - nonzero-open-index role-flip stability guards for non-circle reversed-endpoint-order closure-basic branches
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-61-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use a zero-length lead on the open side to shift intersect attribution off index 0 while preserving the same bounded closure-basic overlap/basic geometry."
requirements-completed: [PAR-273]
duration: 10min
completed: 2026-05-15
---

# Plan 99-61 Summary

## Completed

- Added non-zero-open-index role-flip stability guard for closed-`pline2`
  reversed-endpoint-order closure-basic branch:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline2_with_closure_basic_role_flip_symmetry_nonzero_open_index`
- Added non-zero-open-index role-flip stability guard for closed-`pline1`
  reversed-endpoint-order closure-basic branch:
  - `non_circle_partial_arc_overlap_reversed_endpoint_order_closed_pline1_with_closure_basic_role_flip_symmetry_nonzero_open_index`
- These probes verify that after shifting open-side segment indices via a
  zero-length lead:
  - branch behavior remains `1 basic + 1 overlap`,
  - AB/BA role inversion still swaps start-index attribution as expected,
  - basic intersect point stays stable and overlap endpoint-order swap behavior
    remains unchanged.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this additional nonzero-open-index
  evidence.

## Verification

- `cargo test -p cavalier_contours nonzero_open_index -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.
