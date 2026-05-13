---
phase: 32
slug: capi-function-surface-combine-self-matrix-parity
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 32 Validation Strategy

## Core Gates

- `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture`
- `cargo test --workspace -q`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours`
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours`

## Task Verification Map

| Task ID | Plan | Requirement | Command | Status |
|---------|------|-------------|---------|--------|
| 32-01-01 | 32-01 | PAR-70,PAR-71 | `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` | pass |
| 32-02-01 | 32-02 | PAR-72 | `Select-String -Path .planning\phases\32-capi-function-surface-combine-self-matrix-parity\32-CPP-CAPI-FUNCTION-SURFACE-COMBINE-SELF-PARITY.md -Pattern "combine","self","circle","half-circle","matrix","union","intersect","exclude","xor"` | pass |
| 32-02-02 | 32-02 | PAR-72 | `Select-String -Path .planning\phases\32-capi-function-surface-combine-self-matrix-parity\32-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Priority","Decision","File","Module"` | pass |
| 32-03-01 | 32-03 | PAR-70,PAR-71,PAR-72 | `cargo test --workspace -q` | pass |
| 32-03-02 | 32-03 | PAR-70,PAR-71,PAR-72 | `cargo fmt --all --check` | pass |
| 32-03-03 | 32-03 | PAR-70,PAR-71,PAR-72 | `cargo clippy --all-targets -- -D warnings` | pass |
| 32-03-04 | 32-03 | PAR-70,PAR-71,PAR-72 | `git diff --check` | pass |
| 32-03-05 | 32-03 | PAR-70,PAR-71,PAR-72 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 32-03-06 | 32-03 | PAR-70,PAR-71,PAR-72 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
