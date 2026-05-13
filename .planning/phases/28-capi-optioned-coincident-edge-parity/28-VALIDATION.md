---
phase: 28
slug: capi-optioned-coincident-edge-parity
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 28 Validation Strategy

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
| 28-01-01 | 28-01 | PAR-58,PAR-59 | `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` | pass |
| 28-02-01 | 28-02 | PAR-60 | `Select-String -Path .planning\phases\28-capi-optioned-coincident-edge-parity\28-CPP-CAPI-OPTIONED-COINCIDENT-PARITY.md -Pattern "collapsed_area_eps","coincident","options-path","no-modify","case1","case2"` | pass |
| 28-02-02 | 28-02 | PAR-60 | `Select-String -Path .planning\phases\28-capi-optioned-coincident-edge-parity\28-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Priority","Decision","File","Module"` | pass |
| 28-03-01 | 28-03 | PAR-58,PAR-59,PAR-60 | `cargo test --workspace -q` | pass |
| 28-03-02 | 28-03 | PAR-58,PAR-59,PAR-60 | `cargo fmt --all --check` | pass |
| 28-03-03 | 28-03 | PAR-58,PAR-59,PAR-60 | `cargo clippy --all-targets -- -D warnings` | pass |
| 28-03-04 | 28-03 | PAR-58,PAR-59,PAR-60 | `git diff --check` | pass |
| 28-03-05 | 28-03 | PAR-58,PAR-59,PAR-60 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 28-03-06 | 28-03 | PAR-58,PAR-59,PAR-60 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
