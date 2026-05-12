# Phase 04: benchmark-baseline - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-05-12
**Phase:** 04-benchmark-baseline
**Areas discussed:** Harness shape, coverage mapping, cost accounting, verification scope

---

## Harness Shape

| Option | Description | Selected |
|--------|-------------|----------|
| Criterion-style stable benchmark harness | Add dev-only benchmark infrastructure that runs on stable Rust and produces repeatable local reports. | ✓ |
| Nightly `#[bench]` | Avoid dependency, but requires nightly and does not match current CI/MSRV posture. | |
| Ad hoc timing tests | Minimal setup, but weaker repeatability and harder provenance. | |

**User's choice:** Auto-selected to minimize questions after the user asked to continue with fewer confirmations.
**Notes:** The repository currently has no benchmark dependency or `benches/` directory. Stable Rust benchmark infrastructure best fits Phase 4.

---

## Coverage Mapping

| Option | Description | Selected |
|--------|-------------|----------|
| Map old C++ profile families into Rust operation groups | Cover offset, boolean, intersections, and spatial-index-heavy cases while preserving old profile names. | ✓ |
| Only benchmark the Phase 3 executable fixtures | Smaller, but misses benchmark-profile requirement and spatial-index-heavy cases. | |
| Broad import of old C++ benchmark tables | Too much scope and risks port-first work. | |

**User's choice:** Auto-selected.
**Notes:** Phase 3 explicitly deferred static spatial index throughput and benchmark profiles to Phase 4.

---

## Cost Accounting

| Option | Description | Selected |
|--------|-------------|----------|
| Separate native arc, converted no-arc, conversion, and oracle costs | Makes benchmark meaning explicit and keeps Clipper2 out until Phase 5. | ✓ |
| Mix conversion into operation timing | Simpler, but unclear whether geometry operation or conversion dominates. | |
| Include Clipper2 now | Out of scope before Phase 5 comparability policy. | |

**User's choice:** Auto-selected.
**Notes:** This preserves the roadmap boundary: Phase 4 records current Rust baseline and old C++ mapping; Phase 5 owns Clipper2 oracle costs.

---

## Verification Scope

| Option | Description | Selected |
|--------|-------------|----------|
| Compile/smoke benchmark locally plus normal workspace gates | Practical for development and CI-like validation without requiring long benchmark runs. | ✓ |
| Require full benchmark run as a hard gate | More evidence, but can be slow and machine-dependent. | |
| Skip benchmark execution entirely | Too weak for baseline phase. | |

**User's choice:** Auto-selected.
**Notes:** Full local measurement results should be documented, while generated output artifacts should not be committed.

---

## the agent's Discretion

- Choose exact benchmark file names and Criterion benchmark IDs.
- Decide whether to keep one benchmark target or split by operation during planning.
- Choose the final smoke command after verifying the selected harness behavior.

## Deferred Ideas

- Clipper2 oracle/runtime cost comparison belongs to Phase 5.
- Performance budgets and regression thresholds belong after baseline data exists.
- Algorithm optimization belongs to robustness/capability phases, not Phase 4.

