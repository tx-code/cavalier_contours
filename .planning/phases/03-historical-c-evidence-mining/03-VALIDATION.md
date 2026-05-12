---
phase: 03
slug: historical-c-evidence-mining
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-05-12
---

# Phase 03 - Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust integration tests |
| **Config file** | `Cargo.toml` / workspace `Cargo.lock` |
| **Quick run command** | `cargo test -p cavalier_contours --test test_historical_cavalier_contours` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | repo-dependent; run targeted tests after each task and full suite before completion |

---

## Sampling Rate

- **After every task commit:** Run the targeted test command for files touched by that task.
- **After every plan wave:** Run `cargo test -p cavalier_contours --test test_fixture_harness` plus the Phase 3 targeted test when it exists.
- **Before `$gsd-verify-work`:** `cargo test --workspace`, `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings`, and `git diff --check` must be green.
- **Max feedback latency:** keep task-level checks targeted; run the full suite only at plan or phase boundaries.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 03-01-01 | 03-01 | 1 | FIX-03 | T-03-01 | No untracked external code copy | docs/source audit | `Test-Path .planning/phases/03-historical-c-evidence-mining/03-INVENTORY.md` | yes | pending |
| 03-02-01 | 03-02 | 2 | FIX-03 | T-03-02 | Test-only schema changes only | targeted Rust test | `cargo test -p cavalier_contours --test test_fixture_harness` | yes | pending |
| 03-02-02 | 03-02 | 2 | FIX-03 | T-03-03 | Historical executable fixtures stay green | targeted Rust test | `cargo test -p cavalier_contours --test test_historical_cavalier_contours` | wave 2 | pending |
| 03-03-01 | 03-03 | 3 | FIX-03 | T-03-04 | Metadata-only gaps do not assert behavior | targeted Rust test | `cargo test -p cavalier_contours --test test_historical_cavalier_contours` | wave 2 | pending |
| 03-03-02 | 03-03 | 3 | FIX-03 | T-03-05 | Full workspace remains green | workspace gate | `cargo test --workspace` | yes | pending |

*Status: pending, green, red, or flaky.*

---

## Wave 0 Requirements

Existing infrastructure covers all phase requirements:

- `cavalier_contours/tests/test_utils/fixture_schema.rs`
- `cavalier_contours/tests/test_utils/fixture_harness.rs`
- `cavalier_contours/tests/test_fixture_harness.rs`
- `.planning/phases/03-historical-c-evidence-mining/03-CONTEXT.md`
- `.planning/phases/03-historical-c-evidence-mining/03-RESEARCH.md`

---

## Manual-Only Verifications

None. C API and spatial-index evidence is inventory/metadata in this phase and
does not require manual UI or external service validation.

---

## Validation Sign-Off

- [x] All tasks have automated verify commands or existing Wave 0 dependencies.
- [x] Sampling continuity: no 3 consecutive tasks without automated verify.
- [x] Wave 0 covers all missing references.
- [x] No watch-mode flags.
- [x] `nyquist_compliant: true` set in frontmatter.

**Approval:** approved 2026-05-12
