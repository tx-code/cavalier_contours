---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 135
subsystem: wrap-around-start-index-rotation-options-canonical-alias-alignment
tags: [cpp-parity, find-intersects, wrap-around, start-index-rotation, options, canonical-alias]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: matrix-backed wrap-around start-index-rotation options-path parity guards
provides:
  - canonical non-zero alias names for matrix-backed wrap-around open-side-reversed start-index-rotation options branches
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [alias-to-existing-matrix-guard]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-135-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Use thin alias tests that call existing *_options_matrix_parity guards to improve canonical name coverage without duplicating assertions."
  - "Limit this wave to matrix-backed branches; leave non-matrix missing non-zero counterparts for a follow-up implementation wave."
requirements-completed: [PAR-324]
duration: 15min
completed: 2026-05-15
---

# Plan 99-135 Summary

## Completed

- Added canonical non-zero alias tests for matrix-backed wrap-around
  open-side-reversed start-index-rotation options branches:
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_options_parity`
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_start_index_rotation_options_parity`
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_options_parity`
  - `cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_options_parity`
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` with canonical alias evidence for
  this start-index-rotation options-path alignment step.
- Kept implementation risk bounded by reusing existing matrix assertions.

## Residual Gaps (Next Wave Candidates)

- The global scan for zero-length-lead vs non-zero canonical pairing in
  `*_start_index_rotation_options_parity` still has branches without matrix
  backed non-zero counterparts; these require explicit non-zero test logic
  rather than alias-only wiring.
- Current remaining basenames from that scan:
  - `cpp_reversed_endpoint_closure_basic`
  - `cpp_reversed_endpoint_closure_basic_intersect`
  - `cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_intersect`
  - `cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_intersect`
  - `cpp_wrap_around_same_order_closed_pline2_closure_basic`
  - `cpp_wrap_around_same_order_closed_pline2_closure_basic_intersect`

## Verification

- `cargo test -p cavalier_contours cpp_wrap_around_open_side_reversed_closed_pline2_closure_basic_start_index_rotation_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_open_side_reversed_normal_closed_side_closure_basic_start_index_rotation_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_start_index_rotation_options_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_open_side_reversed_closed_pline2_with_normal_closed_side_closure_basic_intersect_start_index_rotation_options_parity -- --nocapture` - pass.
- `Compare-Object` scan between `*_start_index_rotation_zero_length_lead_options_parity` and `*_start_index_rotation_options_parity` basenames in `test_cpp_offset_parity.rs` - `MissingCount=6` (listed above).
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.
