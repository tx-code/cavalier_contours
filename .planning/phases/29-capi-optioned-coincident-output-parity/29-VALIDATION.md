---
phase: 29
slug: capi-optioned-coincident-output-parity
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 29 Validation Strategy

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
| 29-01-01 | 29-01 | PAR-61,PAR-62 | `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` | pass |
| 29-02-01 | 29-02 | PAR-63 | `Select-String -Path .planning\phases\29-capi-optioned-coincident-output-parity\29-CPP-CAPI-OPTIONED-COINCIDENT-OUTPUT-PARITY.md -Pattern "default-path","options-path","coincident","case1","case2","A-B","B-A","parity"` | pass |
| 29-02-02 | 29-02 | PAR-63 | `Select-String -Path .planning\phases\29-capi-optioned-coincident-output-parity\29-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Priority","Decision","File","Module"` | pass |
| 29-03-01 | 29-03 | PAR-61,PAR-62,PAR-63 | `cargo test --workspace -q` | pass |
| 29-03-02 | 29-03 | PAR-61,PAR-62,PAR-63 | `cargo fmt --all --check` | pass |
| 29-03-03 | 29-03 | PAR-61,PAR-62,PAR-63 | `cargo clippy --all-targets -- -D warnings` | pass |
| 29-03-04 | 29-03 | PAR-61,PAR-62,PAR-63 | `git diff --check` | pass |
| 29-03-05 | 29-03 | PAR-61,PAR-62,PAR-63 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 29-03-06 | 29-03 | PAR-61,PAR-62,PAR-63 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
