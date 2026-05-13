---
phase: 35
slug: capi-combine-self-vertex-exact-reversed-parity
status: complete
nyquist_compliant: true
created: 2026-05-14
---

# Phase 35 Validation Strategy

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
| 35-01-01 | 35-01 | PAR-79,PAR-80 | `cargo test -p cavalier_contours_ffi --test test_pline -- --nocapture` | pass |
| 35-02-01 | 35-02 | PAR-81 | `Select-String -Path .planning\phases\35-capi-combine-self-vertex-exact-reversed-parity\35-CPP-CAPI-COMBINE-SELF-VERTEX-EXACT-PARITY.md -Pattern "combine","self","vertex","reversed","union","intersect","exclude","xor"` | pass |
| 35-02-02 | 35-02 | PAR-81 | `Select-String -Path .planning\phases\35-capi-combine-self-vertex-exact-reversed-parity\35-CPP-LOGIC-ALIGNMENT-MAP.md -Pattern "Priority","Decision","File","Module"` | pass |
| 35-03-01 | 35-03 | PAR-79,PAR-80,PAR-81 | `cargo test --workspace -q` | pass |
| 35-03-02 | 35-03 | PAR-79,PAR-80,PAR-81 | `cargo fmt --all --check` | pass |
| 35-03-03 | 35-03 | PAR-79,PAR-80,PAR-81 | `cargo clippy --all-targets -- -D warnings` | pass |
| 35-03-04 | 35-03 | PAR-79,PAR-80,PAR-81 | `git diff --check` | pass |
| 35-03-05 | 35-03 | PAR-79,PAR-80,PAR-81 | `gsd-sdk query state.validate --project-dir E:\Coding\cavalier_contours` | pass |
| 35-03-06 | 35-03 | PAR-79,PAR-80,PAR-81 | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pass |
