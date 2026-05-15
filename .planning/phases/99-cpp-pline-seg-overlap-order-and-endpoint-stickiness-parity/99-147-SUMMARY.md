---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 147
subsystem: nonrole-matrix-to-options-canonical-gap-closure
tags: [cpp-parity, options, options-matrix, canonical-alias]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: existing non-role options-matrix parity guards
provides:
  - canonical non-role options parity names for remaining matrix-only branches
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [alias-to-existing-validated-guard]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-147-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Add canonical non-role options aliases that delegate directly to existing matrix guards."
requirements-completed: [PAR-324]
duration: 10min
completed: 2026-05-15
---

# Plan 99-147 Summary

## Completed

- Added 16 canonical non-role `*_options_parity` aliases:
  - `cpp_overlap_and_basic_intersection_options_parity`
  - `cpp_skip_intr_at_end_options_parity`
  - `cpp_non_circle_closed_overlap_adjacent_dedup_options_parity`
  - `cpp_opposing_direction_closed_overlap_adjacent_dedup_options_parity`
  - `cpp_overlap_endpoint_arc_adjacent_dedup_options_parity`
  - `cpp_reversed_endpoint_adjacent_line_flip_both_closed_options_parity`
  - `cpp_arc1_reverse_dir_both_closed_options_parity`
  - `cpp_both_reverse_dir_both_closed_options_parity`
  - `cpp_arc2_reverse_dir_both_closed_options_parity`
  - `cpp_wrap_around_closed_pline1_dedup_options_parity`
  - `cpp_wrap_around_closed_pline2_dedup_options_parity`
  - `cpp_wrap_around_same_order_closed_pline2_closure_basic_options_parity`
  - `cpp_wrap_around_closed_pline1_closure_basic_options_parity`
  - `cpp_wrap_around_both_closed_dedup_options_parity`
  - `cpp_wrap_around_overlap_endpoint_dedup_options_parity`
  - `cpp_wrap_around_overlap_endpoint_arc_adjacent_dedup_options_parity`
- Each alias delegates to its corresponding existing
  `*_options_matrix_parity` guard.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with this closure evidence.

## Verification

- `cargo test -p cavalier_contours cpp_overlap_and_basic_intersection_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_skip_intr_at_end_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_non_circle_closed_overlap_adjacent_dedup_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_opposing_direction_closed_overlap_adjacent_dedup_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_overlap_endpoint_arc_adjacent_dedup_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_reversed_endpoint_adjacent_line_flip_both_closed_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_arc1_reverse_dir_both_closed_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_both_reverse_dir_both_closed_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_arc2_reverse_dir_both_closed_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_closed_pline1_dedup_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_closed_pline2_dedup_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_same_order_closed_pline2_closure_basic_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_closed_pline1_closure_basic_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_both_closed_dedup_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_overlap_endpoint_dedup_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_overlap_endpoint_arc_adjacent_dedup_options_parity -- --nocapture` - pass.
- `CppRoleFlipParityBroadMissingCount=0` for mapping
  `cpp_*role_flip*parity* -> cpp_*parity*`.
- `OptionsMatrixMissingCount=0` for mapping
  `*_options_matrix_parity -> *_options_parity`.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.
