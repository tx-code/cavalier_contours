---
phase: 41
slug: capi-options-path-no-modify-hardening
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 41 Validation Strategy

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
| 41-01-01 | 41-01 | PAR-97,PAR-98 | `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` | pass |
| 41-02-01 | 41-02 | PAR-99 | `Select-String -Path .planning\phases\41-capi-options-path-no-modify-hardening\41-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Hardening Outcome","Next Alignment Targets","Decision Boundary"` | pass |
| 41-03-01 | 41-03 | PAR-97,PAR-98,PAR-99 | `cargo test --workspace -q` | pass |
| 41-03-02 | 41-03 | PAR-97,PAR-98,PAR-99 | `cargo fmt --all --check` | pass |
| 41-03-03 | 41-03 | PAR-97,PAR-98,PAR-99 | `cargo clippy --all-targets -- -D warnings` | pass |
| 41-03-04 | 41-03 | PAR-97,PAR-98,PAR-99 | `git diff --check` | pass |
| 41-03-05 | 41-03 | PAR-97,PAR-98,PAR-99 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 41-03-06 | 41-03 | PAR-97,PAR-98,PAR-99 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
