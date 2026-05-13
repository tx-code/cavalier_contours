---
phase: 39
slug: capi-equivalence-zone-regression-hardening
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 39 Validation Strategy

## Core Gates

- `cargo test --workspace -q`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`
- `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours`
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours`

## Task Verification Map

| Task ID | Plan | Requirement | Command | Status |
|---------|------|-------------|---------|--------|
| 39-01-01 | 39-01 | PAR-91,PAR-92 | `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` | pass |
| 39-02-01 | 39-02 | PAR-93 | `Select-String -Path .planning\phases\39-capi-equivalence-zone-regression-hardening\39-EQUIVALENCE-HARDENING-MAP.md -Pattern "Hardening Outcome","Next Alignment Targets","Decision Boundary"` | pass |
| 39-03-01 | 39-03 | PAR-91,PAR-92,PAR-93 | `cargo test --workspace -q` | pass |
| 39-03-02 | 39-03 | PAR-91,PAR-92,PAR-93 | `cargo fmt --all --check` | pass |
| 39-03-03 | 39-03 | PAR-91,PAR-92,PAR-93 | `cargo clippy --all-targets -- -D warnings` | pass |
| 39-03-04 | 39-03 | PAR-91,PAR-92,PAR-93 | `git diff --check` | pass |
| 39-03-05 | 39-03 | PAR-91,PAR-92,PAR-93 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 39-03-06 | 39-03 | PAR-91,PAR-92,PAR-93 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
