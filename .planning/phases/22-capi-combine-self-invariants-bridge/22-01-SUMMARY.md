---
phase: 22-capi-combine-self-invariants-bridge
plan: 01
subsystem: ffi-parity
tags: [cpp-parity, ffi, self-invariants]
requires:
  - phase: 22-capi-combine-self-invariants-bridge
    provides: invariants bridge target
provides:
  - executable C-API combine-with-self invariants parity test
affects: [ffi-tests]
tech-stack:
  added: []
  patterns: [invariant-bridge]
key-files:
  created:
    - .planning/phases/22-capi-combine-self-invariants-bridge/22-01-SUMMARY.md
  modified:
    - cavalier_contours_ffi/tests/test_pline.rs
key-decisions:
  - "Use old C++ invariant matrix semantics with FFI operation mapping (Or/And/Not/Xor)."
requirements-completed: [PAR-40, PAR-41]
duration: 10min
completed: 2026-05-13
---

# Plan 22-01 Summary

## Completed

- Added `pline_boolean_combine_with_self_invariants_cpp_parity` in
  `cavalier_contours_ffi/tests/test_pline.rs`.
- Covered:
  - union with self => self
  - intersect with self => self
  - not/xor with self => empty
  - reversed + mixed-orientation empty-result cases for not/xor.

## Verification

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` - pass.
