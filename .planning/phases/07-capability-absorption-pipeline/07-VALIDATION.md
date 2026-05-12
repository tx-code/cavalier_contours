---
phase: 07
slug: capability-absorption-pipeline
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-05-12
---

# Phase 07 - Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust integration tests and doctests through Cargo |
| **Config file** | `Cargo.toml` workspace |
| **Quick run command** | `cargo test -p cavalier_contours --test <selected_test>` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | ~10 seconds locally for the workspace test suite |

---

## Sampling Rate

- **After every task commit:** Run the selected targeted test command when a
  selected test exists; otherwise run `git diff --check`.
- **After every plan wave:** Run `cargo test --workspace`.
- **Before phase completion:** Full suite, format, clippy, generated-output
  check, GSD state validation, and GSD health must be green.
- **Max feedback latency:** about 30 seconds for standard local gates.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 07-01-01 | 07-01 | 1 | CAP-01 | N/A | N/A | doc/source | `Select-String -Path .planning\phases\07-capability-absorption-pipeline\07-CAPABILITY-CANDIDATES.md -Pattern "absorb-now","defer","not-comparable"` | yes | pending |
| 07-02-01 | 07-02 | 2 | CAP-02,CAP-03,DEM-01 | N/A | N/A | doc/source | `Select-String -Path .planning\phases\07-capability-absorption-pipeline\07-CAPABILITY-DESIGN.md -Pattern "Selected candidate","Semantic fit","FFI impact","UI impact"` | yes | pending |
| 07-03-01 | 07-03 | 3 | CAP-02,CAP-03 | N/A | N/A | integration | `cargo test -p cavalier_contours --test <selected_test>` | selected by 07-02 | pending |
| 07-04-01 | 07-04 | 4 | CAP-01,CAP-02,CAP-03,DEM-01 | N/A | N/A | full gate | `cargo test --workspace` | yes | pending |

---

## Wave 0 Requirements

Existing Rust integration-test infrastructure covers Phase 7. No new framework
installation is required.

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| Demo visual validation, only if selected by 07-02 | DEM-01 | UI changes are conditional on the selected capability | If `07-CAPABILITY-DESIGN.md` marks UI impact as required, run the relevant demo path and record expected visual behavior in `07-VERIFICATION.md`. |

---

## Validation Sign-Off

- [x] All tasks have automated verify commands or explicit conditional gates.
- [x] Sampling continuity avoids three consecutive tasks without automated verify.
- [x] Wave 0 requirements are covered by existing infrastructure.
- [x] No watch-mode flags are used.
- [x] Feedback latency target is under 30 seconds for normal local gates.
- [x] `nyquist_compliant: true` set in frontmatter.

**Approval:** approved 2026-05-12
