---
phase: 80
slug: capi-shape-polyline-accessor-invalid-input-contract-coverage
status: complete
nyquist_compliant: true
created: 2026-05-15
---

# Phase 80 Validation Strategy

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
| 80-01-01 | 80-01 | PAR-214,PAR-215 | `cargo test -p cavalier_contours_ffi --test test_pline -q` | pass |
| 80-02-01 | 80-02 | PAR-216 | `Select-String -Path .planning\phases\80-capi-shape-polyline-accessor-invalid-input-contract-coverage\80-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Deepening Outcome","Next Alignment Targets","Decision Boundary"` | pass |
| 80-03-01 | 80-03 | PAR-214,PAR-215,PAR-216 | `cargo test --workspace -q` | pass |
| 80-03-02 | 80-03 | PAR-214,PAR-215,PAR-216 | `cargo fmt --all --check` | pass |
| 80-03-03 | 80-03 | PAR-214,PAR-215,PAR-216 | `cargo clippy --all-targets -- -D warnings` | pass |
| 80-03-04 | 80-03 | PAR-214,PAR-215,PAR-216 | `git diff --check` | pass |
| 80-03-05 | 80-03 | PAR-214,PAR-215,PAR-216 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 80-03-06 | 80-03 | PAR-214,PAR-215,PAR-216 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
