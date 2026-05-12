# Phase 02: fixture-schema-and-property-harness - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md; this log preserves the alternatives considered.

**Date:** 2026-05-12
**Phase:** 02-fixture-schema-and-property-harness
**Areas discussed:** Fixture format and layout, Fixture schema minimum fields,
Property comparison strategy, Harness integration, Seed fixture scope

---

## Fixture Format and Layout

| Question | Options considered | User's choice |
|----------|--------------------|---------------|
| Fixture should use which primary format? | Rust typed fixtures; JSON fixtures; RON/TOML fixtures; hybrid Rust seed plus JSON schema document | Rust typed fixtures |
| Where should fixture schema and harness code live? | Extend `tests/test_utils/`; create `tests/fixtures/`; keep it in one new test file | Extend `tests/test_utils/`, with seed tests in `test_fixture_harness.rs` |
| Should Phase 2 create an actual fixture data directory? | Do not create one yet; reserve empty directory; create directory with README | Do not create one yet |
| Should schema types stay test-only? | Test-only; core crate dev-only module; new internal crate/module | Test-only |

**Notes:** The user chose the narrow path that proves the harness without adding
parser dependencies or empty scaffolding.

---

## Fixture Schema Minimum Fields

| Question | Options considered | User's choice |
|----------|--------------------|---------------|
| How strict should provenance fields be? | Every fixture requires full provenance; only external fixtures require provenance; define fields but do not require seed fixtures to fill them | Every fixture requires full provenance |
| How should geometry model be represented? | Enumerated geometry model; free-text tag; infer from source only | Enumerated geometry model |
| How should operation be represented? | Enum operation plus operation-specific input; one generic input structure; only offset and boolean in Phase 2 | Enum operation plus operation-specific input |
| How much expected property data is required? | Minimum property set per comparison mode; all properties required; all expected properties optional | Minimum property set per comparison mode |

**Notes:** Current Rust seed fixtures also carry full provenance, avoiding a
special case before external fixture mining begins.

---

## Property Comparison Strategy

| Question | Options considered | User's choice |
|----------|--------------------|---------------|
| Which comparison modes should schema support? | Three core modes; full Phase 1 taxonomy; only approximate properties | Full Phase 1 taxonomy |
| How should intentional divergence and gap cases behave in tests? | Record but do not assert by default; generate ignored failing tests; require alternative Rust expected properties | Record but do not assert by default |
| How should tolerance be managed? | Reuse helper defaults with per-fixture override; one global tolerance; every fixture must declare tolerance | Unified tolerance policy/helper with centralized defaults and per-fixture override |
| Which properties should be asserted by default? | Layered default set; all properties by default; only result count and extents by default | Layered default set |

**Notes:** The user explicitly asked for helper support and unified tolerance
management rather than scattered raw epsilons.

---

## Harness Integration

| Question | Options considered | User's choice |
|----------|--------------------|---------------|
| How should runner API be designed? | Generic `run_fixture`; one runner per operation; only assertion helpers | Generic `run_fixture` |
| Which operation execution paths should Phase 2 cover? | Offset plus boolean plus contains/properties seed; only offset and boolean; all audited operation enums | Offset plus boolean plus contains/properties seed |
| How detailed should failure output be? | Structured diff-style output; existing helper debug print; minimal assert message | Structured diff-style output |
| Should fixture metadata be exposed for later backlog/reporting? | Test-only metadata collector; no collector yet; generate report in Phase 2 | Test-only metadata collector |

**Notes:** Formal reporting remains out of Phase 2; metadata exists for later
gap/oracle/reporting phases.

---

## Seed Fixture Scope

| Question | Options considered | User's choice |
|----------|--------------------|---------------|
| What source should Phase 2 seed fixtures use? | Current Rust behavior only; add small old C++ seed; add small Clipper2 polygon seed | Current Rust behavior only |
| Which minimal operations should seed fixtures cover? | One seed per runner path; only offset and boolean; add degenerate robustness seeds | One seed per runner path |
| Should seed fixtures cover non-executable taxonomy cases? | One metadata-only not-comparable or gap seed; only executable exact/approx seeds; multiple metadata-only cases | One metadata-only not-comparable or gap seed |
| What should Phase 2 seed fixtures explicitly avoid? | Avoid external fixtures, oracle output, and benchmark profiles; allow small external expected values; planner discretion | Avoid external fixtures, oracle output, and benchmark profiles |

**Notes:** Phase 2 proves schema and harness only. External evidence import and
oracle work stay in later phases.

---

## the agent's Discretion

- The agent may choose exact Rust type/module names and concrete current-Rust
  seed shapes.

## Deferred Ideas

- File-based fixture import.
- Old C++ fixture translation.
- Clipper2 oracle output and polygon fixture import.
- Benchmark profile mapping.
