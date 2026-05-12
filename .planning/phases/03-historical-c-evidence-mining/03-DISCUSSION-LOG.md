# Phase 03: historical-c-evidence-mining - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-05-12
**Phase:** 03-historical-c-evidence-mining
**Areas discussed:** Evidence scope and priorities, Translation shape, Mismatch handling, C API / spatial index boundary

---

## Evidence Scope and Priorities

| Option | Description | Selected |
|--------|-------------|----------|
| A | Curated core samples: translate high-value offset, combine/boolean, and polyline property cases; inventory C API and spatial index. | yes |
| B | Add C API behavior evidence as migration-sensitive fixture or metadata notes without changing FFI surface. | yes |
| C | Add spatial-index behavior evidence as inventory and notes; defer performance treatment. | yes |
| D | Broadly import most old C++ parameterized cases. | no |

**User's choice:** ABC
**Notes:** The phase should cover core executable evidence and include C API and
spatial-index evidence, but avoid broad historical test import.

---

## Translation Shape

| Option | Description | Selected |
|--------|-------------|----------|
| A | Manually curated typed Rust fixtures using Phase 2 schema and harness. | yes |
| B | Semi-automatic conversion script from C++ parameter tables. | no |
| C | Inventory only, with no executable fixture translation. | no |
| D | Add JSON, RON, TOML, or another external fixture file format. | no |

**User's choice:** A
**Notes:** Phase 3 should not introduce parsers, generators, or new fixture
file formats.

---

## Mismatch Handling

| Option | Description | Selected |
|--------|-------------|----------|
| A | Import only executable fixtures that currently pass through the harness. | yes |
| B | Import mismatches as red tests. | no |
| C | Record mismatches as metadata-only gap or non-comparable evidence. | yes |
| D | Fix Rust algorithms until all imported old C++ cases pass. | no |

**User's choice:** AC
**Notes:** Phase 3 should keep the workspace test gate green while still making
historical mismatches visible for later phases.

---

## C API / Spatial Index Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| A | C API only as inventory and migration-sensitive metadata; no FFI surface changes. | yes |
| B | C API as executable FFI tests in this phase. | no |
| C | Spatial index as inventory and behavior notes; defer benchmark/performance work. | yes |
| D | Spatial index as executable fixtures in this phase. | no |

**User's choice:** AC
**Notes:** C API and spatial-index evidence belongs in Phase 3, but FFI
execution and performance work do not.

---

## the agent's Discretion

- Choose the exact curated case count and fixture IDs.
- Choose whether C API and spatial-index inventory is best represented in a
  dedicated Phase 3 artifact, metadata-only records, or both.

## Deferred Ideas

- Broad C++ fixture generation.
- Red tests for known historical mismatches.
- FFI execution tests and header regeneration.
- Spatial-index benchmarks and benchmark profile mapping.
- Algorithm fixes for old C++ parity gaps.
