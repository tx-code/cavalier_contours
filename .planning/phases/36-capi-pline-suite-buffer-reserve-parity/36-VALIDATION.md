---
phase: 36
slug: capi-pline-suite-buffer-reserve-parity
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 36 Validation Strategy

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
| 36-01-01 | 36-01 | PAR-82,PAR-83 | `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` | pass |
| 36-02-01 | 36-02 | PAR-84 | `Select-String -Path .planning\phases\36-capi-pline-suite-buffer-reserve-parity\36-CPP-CAPI-PLINE-SUITE-BUFFER-RESERVE-PARITY.md -Pattern "buffer","reserve","empty","no-write","no-modify","pline"` | pass |
| 36-02-02 | 36-02 | PAR-84 | `Select-String -Path .planning\phases\36-capi-pline-suite-buffer-reserve-parity\36-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Priority","Decision","File","Module"` | pass |
| 36-03-01 | 36-03 | PAR-82,PAR-83,PAR-84 | `cargo test --workspace -q` | pass |
| 36-03-02 | 36-03 | PAR-82,PAR-83,PAR-84 | `cargo fmt --all --check` | pass |
| 36-03-03 | 36-03 | PAR-82,PAR-83,PAR-84 | `cargo clippy --all-targets -- -D warnings` | pass |
| 36-03-04 | 36-03 | PAR-82,PAR-83,PAR-84 | `git diff --check` | pass |
| 36-03-05 | 36-03 | PAR-82,PAR-83,PAR-84 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 36-03-06 | 36-03 | PAR-82,PAR-83,PAR-84 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
