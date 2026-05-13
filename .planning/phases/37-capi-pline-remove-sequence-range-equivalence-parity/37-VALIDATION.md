---
phase: 37
slug: capi-pline-remove-sequence-range-equivalence-parity
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 37 Validation Strategy

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
| 37-01-01 | 37-01 | PAR-85,PAR-86 | `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` | pass |
| 37-02-01 | 37-02 | PAR-87 | `Select-String -Path .planning\phases\37-capi-pline-remove-sequence-range-equivalence-parity\37-CPP-CAPI-PLINE-REMOVE-SEQUENCE-RANGE-EQUIVALENCE-PARITY.md -Pattern "remove","range","sequence","vertex","equivalence","pline"` | pass |
| 37-02-02 | 37-02 | PAR-87 | `Select-String -Path .planning\phases\37-capi-pline-remove-sequence-range-equivalence-parity\37-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Priority","Decision","File","Module"` | pass |
| 37-03-01 | 37-03 | PAR-85,PAR-86,PAR-87 | `cargo test --workspace -q` | pass |
| 37-03-02 | 37-03 | PAR-85,PAR-86,PAR-87 | `cargo fmt --all --check` | pass |
| 37-03-03 | 37-03 | PAR-85,PAR-86,PAR-87 | `cargo clippy --all-targets -- -D warnings` | pass |
| 37-03-04 | 37-03 | PAR-85,PAR-86,PAR-87 | `git diff --check` | pass |
| 37-03-05 | 37-03 | PAR-85,PAR-86,PAR-87 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 37-03-06 | 37-03 | PAR-85,PAR-86,PAR-87 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
