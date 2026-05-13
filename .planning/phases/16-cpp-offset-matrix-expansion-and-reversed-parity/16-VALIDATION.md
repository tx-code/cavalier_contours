---
phase: 16
slug: cpp-offset-matrix-expansion-and-reversed-parity
status: complete
nyquist_compliant: true
created: 2026-05-13
---

# Phase 16 Validation Strategy

## Core Gates

- `cargo test -p cavalier_contours --test test_cpp_offset_parity -- --nocapture`
- `cargo test --workspace`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours`
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours`

## Task Verification Map

| Task ID | Plan | Requirement | Command | Status |
|---------|------|-------------|---------|--------|
| 16-01-01 | 16-01 | PAR-22,PAR-23 | `cargo test -p cavalier_contours --test test_cpp_offset_parity -- --nocapture` | pass |
| 16-02-01 | 16-02 | PAR-24 | `Select-String -Path .planning\phases\16-cpp-offset-matrix-expansion-and-reversed-parity\16-CPP-OFFSET-MATRIX-PARITY.md -Pattern "bug","collapsed","reversed","not-comparable"` | pass |
| 16-02-02 | 16-02 | PAR-24 | `Select-String -Path .planning\phases\16-cpp-offset-matrix-expansion-and-reversed-parity\16-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Priority","File","Module"` | pass |
| 16-03-01 | 16-03 | PAR-22,PAR-23,PAR-24 | `cargo test --workspace` | pass |
| 16-03-02 | 16-03 | PAR-22,PAR-23,PAR-24 | `cargo fmt --all --check` | pass |
| 16-03-03 | 16-03 | PAR-22,PAR-23,PAR-24 | `cargo clippy --all-targets -- -D warnings` | pass |
| 16-03-04 | 16-03 | PAR-22,PAR-23,PAR-24 | `git diff --check` | pass |
| 16-03-05 | 16-03 | PAR-22,PAR-23,PAR-24 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 16-03-06 | 16-03 | PAR-22,PAR-23,PAR-24 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |

