---
phase: 05
slug: clipper2-oracle-boundary
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-05-12
---

# Phase 05 - Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Rust integration tests plus dev-only report emission |
| **Config file** | Workspace `Cargo.toml`; no new production dependency expected |
| **Quick run command** | `cargo test -p cavalier_contours --test test_clipper2_oracle_fixtures -- --nocapture` |
| **Report command** | `$env:CAVC_CLIPPER2_ORACLE_REPORT="1"; cargo test -p cavalier_contours --test test_clipper2_oracle_fixtures -- --nocapture` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | quick and full gates should remain practical; report output is local-only |

---

## Sampling Rate

- **After inventory:** verify selected IDs and source paths are present in
  `05-CLIPPER2-INVENTORY.md`.
- **After fixture implementation:** run the targeted integration test.
- **After report path:** run the report command and check
  `target/clipper2-oracle/clipper2-oracle-report.md` exists locally but is not
  staged.
- **Before phase completion:** run targeted test, report command,
  `cargo test --workspace`, `cargo fmt --all --check`, `cargo clippy
  --all-targets -- -D warnings`, `git diff --check`, and GSD health.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 05-01-01 | 05-01 | 1 | FIX-04 | T-05-01 | Sources are classified before execution | doc check | `Select-String -Path .planning\phases\05-clipper2-oracle-boundary\05-CLIPPER2-INVENTORY.md -Pattern "Polygons.txt","Offsets.txt","TestPolygons.cpp","TestOffsets.cpp","triangulation"` | wave 1 | pending |
| 05-02-01 | 05-02 | 2 | FIX-04, ORC-01 | T-05-02 | Executable fixtures cite provenance and stay test-only | targeted test | `cargo test -p cavalier_contours --test test_clipper2_oracle_fixtures -- --nocapture` | wave 2 | pending |
| 05-02-02 | 05-02 | 2 | ORC-02 | T-05-03 | Offset approximation/delta mapping is documented | source/doc check | `Select-String -Path cavalier_contours\tests\test_clipper2_oracle_fixtures.rs -Pattern "JoinType","EndType","delta","approximation"` | wave 2 | pending |
| 05-03-01 | 05-03 | 3 | ORC-01, ORC-03 | T-05-04 | Report classifies pass/gap/not comparable/deferred | report command | `$env:CAVC_CLIPPER2_ORACLE_REPORT="1"; cargo test -p cavalier_contours --test test_clipper2_oracle_fixtures -- --nocapture` | wave 3 | pending |
| 05-04-01 | 05-04 | 4 | FIX-04, ORC-01, ORC-02, ORC-03 | T-05-05 | Evidence is committed, generated output is not | workspace gate | `cargo test --workspace` | wave 4 | pending |
| 05-04-02 | 05-04 | 4 | FIX-04, ORC-01, ORC-02, ORC-03 | T-05-06 | Planning state is consistent | GSD health | `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours` | yes | pending |

*Status: pending, green, red, or flaky.*

---

## Wave 0 Requirements

Existing context covers the phase requirements:

- `.planning/phases/05-clipper2-oracle-boundary/05-CONTEXT.md`
- `.planning/phases/05-clipper2-oracle-boundary/05-RESEARCH.md`
- `.planning/phases/05-clipper2-oracle-boundary/05-PATTERNS.md`
- `.planning/phases/01-absorption-contract-audit/01-PROVENANCE.md`
- `.planning/phases/02-fixture-schema-and-property-harness/02-VERIFICATION.md`
- `.planning/codebase/TESTING.md`

---

## Manual-Only Verifications

Live C++ Clipper2 compilation is optional/manual in this phase. The required
workspace gates use Clipper2 source data and expected public behavior, not a
runtime Clipper2 dependency.

---

## Validation Sign-Off

- [x] All tasks have automated verify commands or existing Wave 0 dependencies.
- [x] Sampling continuity: no 3 consecutive tasks without automated verify.
- [x] Wave 0 covers all missing references.
- [x] No watch-mode flags.
- [x] `nyquist_compliant: true` set in frontmatter.

**Approval:** approved 2026-05-12

