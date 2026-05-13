---
phase: 16-cpp-offset-matrix-expansion-and-reversed-parity
plan: 01
subsystem: parity-tests
tags: [cpp-parity, offset-matrix, reverse-parity]
requires:
  - phase: 16-cpp-offset-matrix-expansion-and-reversed-parity
    provides: phase context and C++ source mapping
provides:
  - executable C++ offset simple/specific matrix parity tests
  - executable reversed-input offset parity checks
affects: [parity-tests]
tech-stack:
  added: []
  patterns: [matrix parity import, source-traceable expectation sets]
key-files:
  created:
    - .planning/phases/16-cpp-offset-matrix-expansion-and-reversed-parity/16-01-SUMMARY.md
  modified:
    - cavalier_contours/tests/test_cpp_offset_parity.rs
key-decisions:
  - "Mirror C++ simple/specific offset expectations and reversed-input parity rule."
requirements-completed: [PAR-22, PAR-23]
duration: 20min
completed: 2026-05-13
---

# Plan 16-01 Summary

## Completed

- Expanded `test_cpp_offset_parity.rs` from a narrow slice to a broader C++
  `parallel_offset` matrix import:
  - simple cases (rectangles and diamonds),
  - specific edge cases (`offset_arc_just_past_line1`,
    `intersect_ontop_first_vertex`, `collapsed_rectangle`).
- Added reversed-input parity test coverage (reverse input + negated delta with
  sign-adjusted area expectation).
- Added input-immutability check for offset execution.

## Verification

- `cargo test -p cavalier_contours --test test_cpp_offset_parity -- --nocapture` - pass.

