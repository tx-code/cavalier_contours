---
phase: 17
slug: cpp-coincident-combine-matrix-parity-expansion
status: complete
nyquist_compliant: true
created: 2026-05-13
---

# Phase 17 Validation Strategy

## Core Gates

- `cargo test -p cavalier_contours --test test_cpp_combine_parity -- --nocapture`
- `cargo test --workspace`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours`
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours`

## Task Verification Map

| Task ID | Plan | Requirement | Command | Status |
|---------|------|-------------|---------|--------|
| 17-01-01 | 17-01 | PAR-25,PAR-26 | `cargo test -p cavalier_contours --test test_cpp_combine_parity -- --nocapture` | pass |
| 17-02-01 | 17-02 | PAR-26,PAR-27 | `Select-String -Path .planning\phases\17-cpp-coincident-combine-matrix-parity-expansion\17-CPP-COINCIDENT-COMBINE-PARITY.md -Pattern "bug","divergence","sliver","not-comparable"` | pass |
| 17-02-02 | 17-02 | PAR-26,PAR-27 | `Select-String -Path .planning\phases\17-cpp-coincident-combine-matrix-parity-expansion\17-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Priority","File","Module"` | pass |
| 17-03-01 | 17-03 | PAR-25,PAR-26,PAR-27 | `cargo test --workspace` | pass |
| 17-03-02 | 17-03 | PAR-25,PAR-26,PAR-27 | `cargo fmt --all --check` | pass |
| 17-03-03 | 17-03 | PAR-25,PAR-26,PAR-27 | `cargo clippy --all-targets -- -D warnings` | pass |
| 17-03-04 | 17-03 | PAR-25,PAR-26,PAR-27 | `git diff --check` | pass |
| 17-03-05 | 17-03 | PAR-25,PAR-26,PAR-27 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 17-03-06 | 17-03 | PAR-25,PAR-26,PAR-27 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |

