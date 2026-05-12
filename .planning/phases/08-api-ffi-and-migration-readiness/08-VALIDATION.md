---
phase: 08
slug: api-ffi-and-migration-readiness
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-05-12
---

# Phase 08 - Validation Strategy

## Test Infrastructure

| Property | Value |
|----------|-------|
| Framework | Rust integration tests and doctests via Cargo |
| Quick run command | `cargo test -p cavalier_contours --test test_pline_boolean rect_clip -- --nocapture` |
| Full suite command | `cargo test --workspace` |
| Expected runtime | ~10-20 seconds locally |

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | Status |
|---------|------|------|-------------|-----------|-------------------|--------|
| 08-01-01 | 08-01 | 1 | API-01,API-02 | doc/source | `Select-String -Path .planning\phases\08-api-ffi-and-migration-readiness\08-COMPATIBILITY-AUDIT.md -Pattern "Rust API Delta","FFI Delta","Header Delta"` | pending |
| 08-02-01 | 08-02 | 2 | API-01,API-02 | docs/surface | `git diff --name-only` | pending |
| 08-03-01 | 08-03 | 3 | API-03 | docs | `Select-String -Path MIGRATION.md -Pattern "CavalierContours","Rust","FFI"` | pending |
| 08-03-02 | 08-03 | 3 | API-01,API-02,API-03 | full gate | `cargo test --workspace` | pending |

## Phase Completion Gates

- `cargo test -p cavalier_contours --test test_pline_boolean rect_clip -- --nocapture`
- `cargo test --workspace`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`
- `git status --short -- target cavalier_contours/target`
- `gsd-sdk query state.validate`
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours`

## Validation Sign-Off

- [x] Each plan has explicit automated verification commands.
- [x] Full workspace gates are required before phase close.
- [x] FFI/header drift is explicitly checked.
- [x] `nyquist_compliant: true` set.
