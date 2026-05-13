---
phase: 86
slug: capi-shape-userdata-getter-output-stability-coverage
status: complete
nyquist_compliant: true
created: 2026-05-15
---

# Phase 86 Validation Strategy

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
| 86-01-01 | 86-01 | PAR-232,PAR-233 | `cargo test -p cavalier_contours_ffi --test test_pline -q` | pass |
| 86-02-01 | 86-02 | PAR-234 | `Select-String -Path .planning\phases\86-capi-shape-userdata-getter-output-stability-coverage\86-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Deepening Outcome","Next Alignment Targets","Decision Boundary"` | pass |
| 86-03-01 | 86-03 | PAR-232,PAR-233,PAR-234 | `cargo test --workspace -q` | pass |
| 86-03-02 | 86-03 | PAR-232,PAR-233,PAR-234 | `cargo fmt --all --check` | pass |
| 86-03-03 | 86-03 | PAR-232,PAR-233,PAR-234 | `cargo clippy --all-targets -- -D warnings` | pass |
| 86-03-04 | 86-03 | PAR-232,PAR-233,PAR-234 | `git diff --check` | pass |
| 86-03-05 | 86-03 | PAR-232,PAR-233,PAR-234 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 86-03-06 | 86-03 | PAR-232,PAR-233,PAR-234 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
