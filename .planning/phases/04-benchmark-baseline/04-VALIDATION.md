---
phase: 04
slug: benchmark-baseline
status: draft
nyquist_compliant: true
wave_0_complete: true
created: 2026-05-12
---

# Phase 04 - Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | Criterion benchmark target plus Rust workspace gates |
| **Config file** | `Cargo.toml` / `cavalier_contours/Cargo.toml` |
| **Quick run command** | `cargo bench -p cavalier_contours --bench geometry_baseline -- --test` |
| **Full suite command** | `cargo test --workspace` |
| **Estimated runtime** | smoke benchmark is practical for local validation; full benchmark measurement is manual/local |

---

## Sampling Rate

- **After benchmark target creation:** run the Criterion smoke command.
- **After historical profile mapping:** run the Criterion smoke command again.
- **Before phase completion:** run the Criterion smoke command, `cargo test
  --workspace`, `cargo fmt --all --check`, `cargo clippy --all-targets -- -D
  warnings`, and `git diff --check`.
- **Max feedback latency:** do not add multiple benchmark groups without a
  smoke compile/run check.

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Threat Ref | Secure Behavior | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|------------|-----------------|-----------|-------------------|-------------|--------|
| 04-01-01 | 04-01 | 1 | BEN-01 | T-04-01 | Benchmark dependency stays dev-only | cargo metadata / smoke | `cargo bench -p cavalier_contours --bench geometry_baseline -- --test` | wave 1 | pending |
| 04-01-02 | 04-01 | 1 | BEN-01 | T-04-02 | Current Rust operation groups compile | benchmark smoke | `cargo bench -p cavalier_contours --bench geometry_baseline -- --test` | wave 1 | pending |
| 04-02-01 | 04-02 | 2 | BEN-02 | T-04-03 | Historical profile families mapped with provenance | source/doc check | `Select-String -Path .planning\phases\04-benchmark-baseline\04-BENCHMARK-MAP.md -Pattern "square","diamond","circle","rounded rectangle","profile1","profile2","pathologicalProfile1"` | wave 2 | pending |
| 04-02-02 | 04-02 | 2 | BEN-02, BEN-03 | T-04-04 | Native and no-arcs variants are separate | benchmark smoke | `cargo bench -p cavalier_contours --bench geometry_baseline -- --test` | wave 2 | pending |
| 04-03-01 | 04-03 | 3 | BEN-03 | T-04-05 | Cost accounting is documented | doc check | `Select-String -Path .planning\phases\04-benchmark-baseline\04-BENCHMARKS.md -Pattern "conversion cost","oracle cost","setup cost","target/criterion"` | wave 3 | pending |
| 04-03-02 | 04-03 | 3 | BEN-01, BEN-02, BEN-03 | T-04-06 | Workspace remains green | workspace gate | `cargo test --workspace` | yes | pending |

*Status: pending, green, red, or flaky.*

---

## Wave 0 Requirements

Existing context covers the phase requirements:

- `.planning/phases/04-benchmark-baseline/04-CONTEXT.md`
- `.planning/phases/04-benchmark-baseline/04-RESEARCH.md`
- `.planning/phases/04-benchmark-baseline/04-PATTERNS.md`
- `.planning/codebase/TESTING.md`
- `.planning/codebase/STACK.md`

---

## Manual-Only Verifications

Full measurement with `cargo bench -p cavalier_contours --bench
geometry_baseline` is manual/local because it can be slower and produces
generated output under `target/criterion`. The smoke command remains the
automated phase gate.

---

## Validation Sign-Off

- [x] All tasks have automated verify commands or existing Wave 0 dependencies.
- [x] Sampling continuity: no 3 consecutive tasks without automated verify.
- [x] Wave 0 covers all missing references.
- [x] No watch-mode flags.
- [x] `nyquist_compliant: true` set in frontmatter.

**Approval:** approved 2026-05-12

