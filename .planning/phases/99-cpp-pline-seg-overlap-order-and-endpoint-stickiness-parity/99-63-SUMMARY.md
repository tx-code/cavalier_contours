---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 63
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, wrap-around, same-order, closure-basic, closure-basic-intersect, role-flip, nonzero-open-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: reversed-endpoint-order closure-basic-intersect nonzero-open-index guards
provides:
  - nonzero-open-index role-flip stability guards for wrap-around same-order closed-pline2 closure-basic branches
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-63-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use a zero-length lead on the open side to shift intersect attribution off index 0 while preserving same-order wrap-around closure-basic geometry and role-flip endpoint ordering semantics."
requirements-completed: [PAR-273]
duration: 12min
completed: 2026-05-15
---

# Plan 99-63 Summary

## Completed

- Added non-zero-open-index role-flip stability guard for wrap-around
  same-order closed-`pline2` closure-basic branch:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_role_flip_symmetry_nonzero_open_index`
- Added canonical-name non-zero-open-index counterpart for the same branch:
  - `wrap_around_non_circle_arc_overlap_same_order_closed_pline2_with_closure_basic_intersect_role_flip_symmetry_nonzero_open_index`
- These probes verify that after shifting open-side segment indices via a
  zero-length lead:
  - branch behavior remains `1 basic + 1 overlap`,
  - AB/BA role inversion still swaps start-index attribution as expected,
  - basic intersect point and same-order overlap endpoint ordering remain stable.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this wrap-around same-order
  nonzero-open-index evidence.

## Verification

- `cargo test -p cavalier_contours same_order_closed_pline2_with_closure_basic_role_flip_symmetry_nonzero_open_index -- --nocapture` - pass.
- `cargo test -p cavalier_contours same_order_closed_pline2_with_closure_basic_intersect_role_flip_symmetry_nonzero_open_index -- --nocapture` - pass.
- `cargo test -p cavalier_contours nonzero_open_index -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.
