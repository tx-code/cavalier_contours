---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 64
subsystem: collection-level-parity
tags: [cpp-parity, find-intersects, non-circle, wrap-around, open-side-reversed, closure-basic, closure-basic-intersect, role-flip, nonzero-open-index]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: same-order closed-pline2 nonzero-open-index closure-basic guards
provides:
  - canonical-name nonzero-open-index role-flip guards for remaining wrap-around open-side-reversed closure-basic families
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [bounded-regression-probe]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-64-SUMMARY.md
  modified:
    - cavalier_contours/src/polyline/internal/pline_intersects.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Reuse proven open-side zero-length-lead constructions and add explicit canonical-name assertion probes rather than alias wrappers."
requirements-completed: [PAR-273]
duration: 16min
completed: 2026-05-15
---

# Plan 99-64 Summary

## Completed

- Added canonical-name nonzero-open-index role-flip guard for wrap-around
  open-side-reversed + closed-side-reversed closure-basic branch:
  - `wrap_around_non_circle_arc_overlap_closed_side_reversed_closure_basic_role_flip_symmetry_nonzero_open_index`
- Added canonical-name nonzero-open-index role-flip guards for wrap-around
  open-side-reversed closed-`pline2` closure-basic branches:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_role_flip_symmetry_nonzero_open_index`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_closure_basic_intersect_role_flip_symmetry_nonzero_open_index`
- Added canonical-name nonzero-open-index role-flip guards for wrap-around
  open-side-reversed + normal-closed-side closure-basic branches:
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_role_flip_symmetry_nonzero_open_index`
  - `wrap_around_non_circle_arc_overlap_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_role_flip_symmetry_nonzero_open_index`
- All five probes verify nonzero-open-index shift keeps `1 basic + 1 overlap`,
  preserves AB/BA role-swapped index attribution, and preserves each branch's
  expected overlap endpoint-ordering semantics.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` and recorded matrix closure:
  `BASE_COUNT=11`, `MISSING_COUNT=0` for
  `*_closure_basic*_role_flip_symmetry` vs `*_nonzero_open_index`.

## Verification

- `cargo test -p cavalier_contours nonzero_open_index -- --nocapture` - pass.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.
