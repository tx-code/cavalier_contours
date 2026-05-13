---
phase: 22
slug: capi-combine-self-invariants-bridge
status: complete
nyquist_compliant: true
created: 2026-05-13
---

# Phase 22 Validation Strategy

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
| 22-01-01 | 22-01 | PAR-40,PAR-41 | `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` | pass |
| 22-02-01 | 22-02 | PAR-41,PAR-42 | `Select-String -Path .planning\phases\22-capi-combine-self-invariants-bridge\22-CPP-CAPI-COMBINE-SELF-INVARIANTS-PARITY.md -Pattern "self","reversed","mixed","cavc_pline_boolean","parity"` | pass |
| 22-02-02 | 22-02 | PAR-42 | `Select-String -Path .planning\phases\22-capi-combine-self-invariants-bridge\22-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Priority","Decision","File","Module"` | pass |
| 22-03-01 | 22-03 | PAR-40,PAR-41,PAR-42 | `cargo test --workspace -q` | pass |
| 22-03-02 | 22-03 | PAR-40,PAR-41,PAR-42 | `cargo fmt --all --check` | pass |
| 22-03-03 | 22-03 | PAR-40,PAR-41,PAR-42 | `cargo clippy --all-targets -- -D warnings` | pass |
| 22-03-04 | 22-03 | PAR-40,PAR-41,PAR-42 | `git diff --check` | pass |
| 22-03-05 | 22-03 | PAR-40,PAR-41,PAR-42 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 22-03-06 | 22-03 | PAR-40,PAR-41,PAR-42 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
