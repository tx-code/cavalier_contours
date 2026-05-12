# Phase 01: absorption-contract-audit - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md - this log preserves the alternatives considered.

**Date:** 2026-05-12
**Phase:** 01-absorption-contract-audit
**Areas discussed:** Audit scope boundary, External code usage rules, Behavior comparability classification, API/FFI protection scope

---

## Audit Scope Boundary

| Option | Description | Selected |
|--------|-------------|----------|
| Full evidence inventory | Cover API/operations, tests, benchmarks, examples, README/algorithm notes, known limitations, FFI/C API. | yes |
| Implementation surface only | Cover source modules, public API, and FFI/API surfaces. | |
| Two-pass audit | High-level matrix now, deep dives in later phases. | |

**User's choice:** Full evidence inventory
**Notes:** User accepted the recommended evidence-heavy scope.

| Option | Description | Selected |
|--------|-------------|----------|
| One main matrix plus partitioned appendices | Cross-codebase comparison with source-specific evidence appendices. | yes |
| Three separate audits by codebase | Complete per-repo docs, comparison done by reader. | |
| Split by capability domain | Sections such as offset, boolean, intersection, FFI, benchmarks. | |

**User's choice:** One main matrix plus partitioned appendices
**Notes:** Chosen to support planner consumption and preserve evidence paths.

| Option | Description | Selected |
|--------|-------------|----------|
| Single `01-AUDIT.md` | All audit content in one file. | |
| `01-AUDIT.md` plus `01-PROVENANCE.md` | Split capability audit from license/provenance evidence. | yes |
| Directory-style multi-file audit | Multiple files under an audit directory. | |

**User's choice:** `01-AUDIT.md` plus `01-PROVENANCE.md`
**Notes:** User first typed `1`, then interrupted and corrected to `2`; latest choice wins.

| Option | Description | Selected |
|--------|-------------|----------|
| Evidence path required | Every main matrix entry has a path or explicit status. | yes |
| Strict complete | Every entry must have evidence before completion. | |
| Best-effort summary | Major findings only. | |

**User's choice:** Evidence path required
**Notes:** Avoids hidden blanks without turning Phase 1 into unbounded archaeology.

---

## External Code Usage Rules

| Option | Description | Selected |
|--------|-------------|----------|
| Evidence-first translation | Translate test ideas, fixtures, benchmark profiles, and behavioral expectations; implementation code is reference-only. | yes |
| Permissive reuse by license | Copy small code/test/data snippets when license permits. | |
| No copying, observation only | Record behavior and paths only. | |

**User's choice:** Evidence-first translation
**Notes:** Keeps absorption focused on evidence and behavior, not direct implementation copying.

| Option | Description | Selected |
|--------|-------------|----------|
| Source-specific rules | Old C++ is historical reference; Clipper2 is polygon-only oracle/reference. | yes |
| Unified rules | Same evidence-first handling for both sources. | |
| Capability-domain rules | Separate rules by offset/boolean/intersection/API/benchmark. | |

**User's choice:** Source-specific rules
**Notes:** Captures the semantic difference between same-lineage C++ and external Clipper2.

| Option | Description | Selected |
|--------|-------------|----------|
| Candidate registry | Record source path, capability domain, value, risk, and suggested follow-up phase. | yes |
| Immediate fixture TODOs | Write concrete TODOs for future fixture/benchmark import. | |
| Narrative notes only | Mention in audit prose. | |

**User's choice:** Candidate registry
**Notes:** Phase 1 registers candidates but does not import fixtures or benchmarks.

| Option | Description | Selected |
|--------|-------------|----------|
| Path + license + usage intent | Record source path, license, and intended usage. | |
| Path + commit hash + license + usage intent | Also pin external repo commit hash. | yes |
| Repo-level only | Only record repo-level license and usage. | |

**User's choice:** Path + commit hash + license + usage intent
**Notes:** External references should be reproducible across old C++ and Clipper2 snapshots.

---

## Behavior Comparability Classification

| Option | Description | Selected |
|--------|-------------|----------|
| Strict property parity | Normalized core properties match within tolerance without requiring literal vertex order. | yes |
| Literal output parity | Vertex count, order, coordinates, and bulges match as closely as possible. | |
| Behavioral smoke parity | Broad behavior looks similar. | |

**User's choice:** Strict property parity
**Notes:** Better fit for geometric equivalence than literal vertex ordering.

| Option | Description | Selected |
|--------|-------------|----------|
| Arc approximation and tolerance-bound comparisons | Use approximate parity only with recorded approximation/tolerance policy. | yes |
| Any close-enough behavior | Any similar-looking result can be approximate. | |
| Do not use approximate parity | Only exact, divergence, or not-comparable labels. | |

**User's choice:** Arc approximation and tolerance-bound comparisons
**Notes:** Lets Clipper2 participate without weakening evidence quality.

| Option | Description | Selected |
|--------|-------------|----------|
| Decision-backed divergence | Only when PROJECT/ROADMAP/CONTEXT or phase artifacts explicitly say Rust should differ. | yes |
| Maintainer judgment divergence | Auditor may mark divergence subjectively. | |
| Avoid divergence label in Phase 1 | Use exact/approx/not-comparable/gap only. | |

**User's choice:** Decision-backed divergence
**Notes:** Prevents bugs or missing behavior being disguised as intended behavior.

| Option | Description | Selected |
|--------|-------------|----------|
| Model mismatch vs missing desired behavior | `not comparable` for model/scope mismatch, `gap` for missing desired behavior. | yes |
| No difference | Treat both as not comparable. | |
| Everything becomes gap unless excluded | Aggressively treat differences as gaps. | |

**User's choice:** Model mismatch vs missing desired behavior
**Notes:** Protects the arc-aware model while preserving real absorption opportunities.

---

## API/FFI Protection Scope

| Option | Description | Selected |
|--------|-------------|----------|
| All public integration surfaces | Rust public API, Rust C FFI plus generated header, old C++ header API, old C++ C API, Clipper2 public operations. | yes |
| Rust public API plus Rust C FFI only | Focus on mainline surfaces. | |
| Rust API plus old C++ API only | Focus on same-lineage migration. | |

**User's choice:** All public integration surfaces
**Notes:** Phase 1 includes both migration and Clipper2 operation boundary concerns.

| Option | Description | Selected |
|--------|-------------|----------|
| Yes, classify risk per surface | Classify API/FFI surface change sensitivity. | yes |
| Only list surfaces | Inventory without classification. | |
| Only protect FFI | Treat FFI as the only sensitive surface. | |

**User's choice:** Yes, classify risk per surface, but not as protection-by-default
**Notes:** User later clarified this is a fork, so classification must not imply default compatibility protection.

| Option | Description | Selected |
|--------|-------------|----------|
| Compatibility-first unless phase explicitly changes it | Default preserve Rust public API, serde shape, FFI ABI, and generated header. | |
| Rust API flexible, FFI strict | Rust API can move, FFI is protected. | |
| Audit only, no policy | Record surfaces without default protection policy. | |
| User correction: fork-owned surfaces are changeable | API/FFI do not need protection by default because this is a fork. | yes |

**User's choice:** Fork-owned surfaces are changeable; compatibility is not a default constraint
**Notes:** User interrupted the prior compatibility-first choice and corrected the premise.

| Option | Description | Selected |
|--------|-------------|----------|
| Freedom-oriented labels | `fork-owned/changeable`, `migration-sensitive`, `reference-only`, `external-oracle`. | yes |
| Keep risk labels but weaker | Keep protected-style terms but weaken their meaning. | |
| No classification | List API surfaces without classification. | |

**User's choice:** Freedom-oriented labels
**Notes:** Matches the fork reality while preserving planning signal.

| Option | Description | Selected |
|--------|-------------|----------|
| Impact note required | Record changed surface, rationale, and effects on tests/examples/FFI/header/docs. | yes |
| No special record | Change code normally. | |
| Migration note only when user-facing | Record only obvious user-facing changes. | |

**User's choice:** Impact note required
**Notes:** This is a consistency and traceability rule, not a compatibility freeze.

---

## the agent's Discretion

None. The user selected concrete options for all discussed gray areas.

## Deferred Ideas

None.
