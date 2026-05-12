---
phase: 06
slug: robustness-gap-closure
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-05-12
---

# Phase 06 - Validation Strategy

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust integration tests plus workspace gates |
| **Primary targeted command** | `cargo test -p cavalier_contours --test test_shape_parallel_offset -- --nocapture` |
| **Full suite command** | `cargo test --workspace` |
| **Lint command** | `cargo clippy --all-targets -- -D warnings` |

## Sampling Rate

- After backlog creation: run doc checks for ranked entries and all required
  robustness categories.
- After focused regressions: run targeted shape offset test.
- After production fix: run targeted test again, then workspace gates.
- Before completion: run targeted test, workspace test, fmt, clippy, diff, and
  GSD health.

## Per-Task Verification Map

| Task ID | Plan | Requirement | Test Type | Automated Command | Status |
|---------|------|-------------|-----------|-------------------|--------|
| 06-01-01 | 06-01 | ROB-01 | doc check | `Select-String -Path .planning\phases\06-robustness-gap-closure\06-ROBUSTNESS-BACKLOG.md -Pattern "offset","boolean","intersection","tolerance","degenerate","repeat","tangent","overlap","open/closed"` | pending |
| 06-02-01 | 06-02 | ROB-02 | targeted test | `cargo test -p cavalier_contours --test test_shape_parallel_offset -- --nocapture` | pending |
| 06-03-01 | 06-03 | ROB-03 | targeted test | `cargo test -p cavalier_contours --test test_shape_parallel_offset -- --nocapture` | pending |
| 06-04-01 | 06-04 | ROB-04 | workspace gate | `cargo test --workspace` | pending |
| 06-04-02 | 06-04 | ROB-04 | GSD health | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | pending |

## Manual-Only Verifications

No manual UI or live Clipper2 build is required for Phase 6. Deferred Clipper2
fixtures can be ranked without becoming executable in this phase.

## Validation Sign-Off

- [x] All tasks have automated verify commands or existing Wave 0 dependencies.
- [x] Sampling continuity: no 3 consecutive tasks without automated verify.
- [x] Wave 0 covers all missing references.
- [x] No watch-mode flags.
- [x] `nyquist_compliant: true` set in frontmatter.

**Approval:** approved 2026-05-12

