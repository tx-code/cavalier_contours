---
phase: 35-capi-combine-self-vertex-exact-reversed-parity
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, combine-self, vertex-exact]
requires:
  - phase: 35-capi-combine-self-vertex-exact-reversed-parity
    provides: combine-self vertex exact target
provides:
  - executable vertex-exact combine-self C-API parity test
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [combine-self-vertex-exact-bridge]
key-files:
  created:
    - .planning/phases/35-capi-combine-self-vertex-exact-reversed-parity/35-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Keep property-level regression checks and add dedicated vertex-exact self-invariant test for the source-backed sample."
requirements-completed: [PAR-79, PAR-80]
duration: 8min
completed: 2026-05-14
---

# Plan 35-01 Summary

## Completed

- Added vertex-exact self-combine invariant test:
  - `pline_boolean_combine_with_self_invariants_vertex_exact_cpp_parity`
- Covered forward/reversed self and forward-reversed cross combinations for
  exclude/xor emptiness invariants.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
