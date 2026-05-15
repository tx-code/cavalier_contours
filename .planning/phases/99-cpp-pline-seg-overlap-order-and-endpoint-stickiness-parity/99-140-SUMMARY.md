---
phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
plan: 140
subsystem: role-flip-to-nonrole-canonical-options-alias-closure
tags: [cpp-parity, find-intersects, options, role-flip, canonical-alias, start-index-rotation]
requires:
  - phase: 99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity
    provides: existing role-flip options-path guards across wrap-around and reversed-endpoint branches
provides:
  - canonical non-role aliases for all previously missing role-flip-only options-path branch names
affects: [tests, planning-docs]
tech-stack:
  added: []
  patterns: [alias-to-existing-validated-guard]
key-files:
  created:
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-140-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
    - .planning/phases/99-cpp-pline-seg-overlap-order-and-endpoint-stickiness-parity/99-CPP-LOGIC-ALIGNMENT-MAP.md
key-decisions:
  - "Close remaining role-flip-only naming gaps by delegating canonical non-role names to existing validated role-flip guards."
  - "Preserve behavioral assertions and only expand callable canonical surface."
requirements-completed: [PAR-324]
duration: 20min
completed: 2026-05-15
---

# Plan 99-140 Summary

## Completed

- Added 13 canonical non-role aliases that previously had only
  `*_role_flip_options_parity` coverage:
  - `cpp_reversed_endpoint_adjacent_line_flip_both_closed_start_index_rotation_options_parity`
  - `cpp_reversed_endpoint_adjacent_line_flip_both_closed_start_index_rotation_zero_length_lead_options_parity`
  - `cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_options_parity`
  - `cpp_wrap_around_closed_side_reversed_closure_basic_start_index_rotation_zero_length_lead_options_parity`
  - `cpp_wrap_around_non_circle_arc_overlap_deduplication_reversed_order_start_index_rotation_options_parity`
  - `cpp_wrap_around_non_circle_arc_overlap_deduplication_reversed_order_start_index_rotation_zero_length_lead_options_parity`
  - `cpp_wrap_around_non_circle_arc_overlap_deduplication_same_order_start_index_rotation_options_parity`
  - `cpp_wrap_around_non_circle_arc_overlap_deduplication_same_order_start_index_rotation_zero_length_lead_options_parity`
  - `cpp_wrap_around_open_side_reversed_closed_side_reversed_options_parity`
  - `cpp_wrap_around_overlap_endpoint_arc_adjacent_dedup_start_index_rotation_options_parity`
  - `cpp_wrap_around_overlap_endpoint_arc_adjacent_dedup_start_index_rotation_zero_length_lead_options_parity`
  - `cpp_wrap_around_overlap_endpoint_dedup_start_index_rotation_options_parity`
  - `cpp_wrap_around_overlap_endpoint_dedup_start_index_rotation_zero_length_lead_options_parity`
- Updated `99-CPP-LOGIC-ALIGNMENT-MAP.md` evidence for this closure.

## Verification

- Batch run of all 13 new alias tests via `cargo test -q -p cavalier_contours <test_name>` loop - pass (`ALL_ALIAS_TESTS_PASS`).
- Spot checks:
  - `cargo test -p cavalier_contours cpp_reversed_endpoint_adjacent_line_flip_both_closed_start_index_rotation_options_parity -- --nocapture` - pass.
  - `cargo test -p cavalier_contours cpp_wrap_around_open_side_reversed_closed_side_reversed_options_parity -- --nocapture` - pass.
  - `cargo test -p cavalier_contours cpp_wrap_around_overlap_endpoint_dedup_start_index_rotation_zero_length_lead_options_parity -- --nocapture` - pass.
- Canonical gap scan:
  - `RoleFlipMissingCount=0` for mapping `*_role_flip_options_parity -> *_options_parity`.
- `cargo fmt --all --check` - pass.
- `cargo test --workspace -q` - pass.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` - healthy.
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` - pass.
