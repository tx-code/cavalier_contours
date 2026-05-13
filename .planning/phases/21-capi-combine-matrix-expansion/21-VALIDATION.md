---
phase: 21
slug: capi-combine-matrix-expansion
status: complete
nyquist_compliant: true
created: 2026-05-13
---

# Phase 21 Validation Strategy

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
| 21-01-01 | 21-01 | PAR-37,PAR-38 | `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` | pass |
| 21-02-01 | 21-02 | PAR-38,PAR-39 | `Select-String -Path .planning\phases\21-capi-combine-matrix-expansion\21-CPP-CAPI-COMBINE-MATRIX-PARITY.md -Pattern "circle_rectangle","coincident_case2","cavc_pline_boolean","parity"` | pass |
| 21-02-02 | 21-02 | PAR-39 | `Select-String -Path .planning\phases\21-capi-combine-matrix-expansion\21-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Priority","Decision","File","Module"` | pass |
| 21-03-01 | 21-03 | PAR-37,PAR-38,PAR-39 | `cargo test --workspace -q` | pass |
| 21-03-02 | 21-03 | PAR-37,PAR-38,PAR-39 | `cargo fmt --all --check` | pass |
| 21-03-03 | 21-03 | PAR-37,PAR-38,PAR-39 | `cargo clippy --all-targets -- -D warnings` | pass |
| 21-03-04 | 21-03 | PAR-37,PAR-38,PAR-39 | `git diff --check` | pass |
| 21-03-05 | 21-03 | PAR-37,PAR-38,PAR-39 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 21-03-06 | 21-03 | PAR-37,PAR-38,PAR-39 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
