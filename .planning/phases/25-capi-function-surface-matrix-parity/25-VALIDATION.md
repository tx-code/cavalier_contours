---
phase: 25
slug: capi-function-surface-matrix-parity
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 25 Validation Strategy

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
| 25-01-01 | 25-01 | PAR-49 | `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` | pass |
| 25-02-01 | 25-02 | PAR-50,PAR-51 | `Select-String -Path .planning\phases\25-capi-function-surface-matrix-parity\25-CPP-CAPI-FUNCTION-SURFACE-MATRIX-PARITY.md -Pattern "area","path","extents","winding","closest-point","not-comparable","cavc_pline_eval_wn"` | pass |
| 25-02-02 | 25-02 | PAR-51 | `Select-String -Path .planning\phases\25-capi-function-surface-matrix-parity\25-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Priority","Decision","File","Module"` | pass |
| 25-03-01 | 25-03 | PAR-49,PAR-50,PAR-51 | `cargo test --workspace -q` | pass |
| 25-03-02 | 25-03 | PAR-49,PAR-50,PAR-51 | `cargo fmt --all --check` | pass |
| 25-03-03 | 25-03 | PAR-49,PAR-50,PAR-51 | `cargo clippy --all-targets -- -D warnings` | pass |
| 25-03-04 | 25-03 | PAR-49,PAR-50,PAR-51 | `git diff --check` | pass |
| 25-03-05 | 25-03 | PAR-49,PAR-50,PAR-51 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 25-03-06 | 25-03 | PAR-49,PAR-50,PAR-51 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
