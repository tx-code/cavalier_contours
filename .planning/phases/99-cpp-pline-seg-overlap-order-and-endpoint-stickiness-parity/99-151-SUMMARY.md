---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 151
subsystem: closed-side-reversed-closure-basic-options-to-matrix-gap-closure
tags: [cpp-parity, options, options-matrix, canonical-alias, wrap-around]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: existing closed-side-reversed closure-basic options parity guards
provides:
  - canonical matrix names for closed-side-reversed closure-basic options families
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [alias-to-existing-validated-guard]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-151-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Add matrix aliases delegating to existing options-parity guards."
requirements-completed: [PAR-324]
duration: 5min
completed: 2026-05-15
---

# Plan 99-151 Summary

## Completed

- Added 6 canonical matrix aliases:
  - `cpp_wrap_around_closed_side_reversed_closure_basic_nonzero_open_index_options_matrix_parity`
  - `cpp_wrap_around_closed_side_reversed_closure_basic_role_flip_nonzero_open_index_options_matrix_parity`
  - `cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_options_matrix_parity`
  - `cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_role_flip_options_matrix_parity`
  - `cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_zero_length_lead_options_matrix_parity`
  - `cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_zero_length_lead_role_flip_options_matrix_parity`
- Each alias delegates to the corresponding existing `*_options_parity` guard.
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` evidence for this closure.

## Verification

- `cargo test -p cavalier_contours cpp_wrap_around_closed_side_reversed_closure_basic_nonzero_open_index_options_matrix_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_closed_side_reversed_closure_basic_role_flip_nonzero_open_index_options_matrix_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_options_matrix_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_role_flip_options_matrix_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_zero_length_lead_options_matrix_parity -- --nocapture` - pass.
- `cargo test -p cavalier_contours cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_zero_length_lead_role_flip_options_matrix_parity -- --nocapture` - pass.
- `OptionsToMatrixMissingCount` decreases for this family and remains script-auditable.
- `OptionsMatrixMissingCount=0`.
- `CppRoleFlipParityBroadMissingCount=0`.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.
