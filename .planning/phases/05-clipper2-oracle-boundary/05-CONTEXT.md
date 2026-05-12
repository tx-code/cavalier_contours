# Phase 05: clipper2-oracle-boundary - Context

**Gathered:** 2026-05-12
**Status:** Ready for planning

<domain>
## Phase Boundary

This phase uses Clipper2 as a dev-only polygon oracle for eligible boolean and
offset evidence. It classifies Clipper2 source tests, translates a small curated
set into Rust fixture representations, and records oracle comparison evidence
for later gap ranking. It does not vendor Clipper2, make Clipper2 a production
backend, redefine native bulge-arc behavior, import triangulation, change FFI,
or optimize Rust algorithms.

</domain>

<decisions>
## Implementation Decisions

### Oracle Role

- **D-01:** Clipper2 is `external-oracle` / `OracleComparable` evidence only.
  It must not become a production dependency or backend in this phase.
- **D-02:** Use Clipper2 expected test data and documented public behavior as
  oracle evidence for selected polygon-only cases. Do not copy Clipper2
  implementation code.
- **D-03:** The dev-only comparison path may be Rust-side and compare current
  Rust output against Clipper2-derived expected properties; live C++ Clipper2
  compilation is optional/manual and not required for normal workspace gates.

### Eligibility

- **D-04:** Eligible cases are closed polygon paths with straight segments that
  map to Rust line-only closed polylines. Arc-bearing Rust cases are only
  comparable after explicit arc-to-polygon approximation notes.
- **D-05:** Boolean oracle cases must map to current Rust boolean semantics:
  two closed area polylines, supported operation, no open-path result
  requirement, and property-based comparison.
- **D-06:** Offset oracle cases must record Clipper2 join/end type, delta,
  arc tolerance if applicable, and whether comparison is approximate.
- **D-07:** Polytree, hole ownership, open path clipping, rect clipping,
  Minkowski, cleanup-only, random, platform-specific, and very large generated
  cases should be inventoried but not made executable unless clearly comparable.
- **D-08:** Triangulation remains excluded.

### Fixture and Reporting Shape

- **D-09:** Reuse the existing test fixture harness and metadata model where
  possible. Prefer `UsageLabel::OracleComparable`,
  `GeometryModel::PolygonPath`, and `ComparisonMode::ApproximateParity` or
  `NotComparable` over adding new schema.
- **D-10:** Use a small curated executable set first. Broad parsers for
  `Polygons.txt` or `Offsets.txt` are out of scope unless implementation shows
  they are cheaper than manual translation.
- **D-11:** Oracle evidence reports should classify each selected case as
  pass, gap, not comparable, or deferred. Reports are planning evidence for
  Phase 6 ranking, not automatic production behavior.

### Verification and Scope

- **D-12:** Verification must include targeted Clipper2 oracle fixture tests,
  workspace tests, format, clippy, `git diff --check`, and GSD health.
- **D-13:** Generated oracle reports under `target/` are local artifacts and
  must not be committed; concise Phase 5 evidence docs may be committed.
- **D-14:** No production source, FFI header, UI, or benchmark baseline changes
  are expected unless a plan explicitly justifies them.

### the agent's Discretion

The agent may choose exact fixture IDs, selected Clipper2 case numbers, report
file names, and whether the dev-only report writer is implemented as an
integration-test helper, script, or small tool, as long as the decisions above
are preserved.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Planning

- `.planning/PROJECT.md` - core value, source roles, and no port-first rule.
- `.planning/REQUIREMENTS.md` - Phase 5 requirements `FIX-04`, `ORC-01`,
  `ORC-02`, and `ORC-03`.
- `.planning/ROADMAP.md` - Phase 5 goal, success criteria, and plan skeleton.
- `.planning/STATE.md` - current workflow position.

### Prior Phase Evidence

- `.planning/phases/01-absorption-contract-audit/01-PROVENANCE.md` - Clipper2
  snapshot, Boost license, usage labels, and acceptable-use rules.
- `.planning/phases/01-absorption-contract-audit/01-AUDIT.md` - Clipper2
  classification as polygon-only oracle/reference and triangulation deferral.
- `.planning/phases/02-fixture-schema-and-property-harness/02-VERIFICATION.md`
  - fixture harness readiness.
- `.planning/phases/03-historical-c-evidence-mining/03-INVENTORY.md` - existing
  fixture metadata patterns and Phase 3 provenance style.
- `.planning/phases/04-benchmark-baseline/04-BENCHMARKS.md` - explicit
  exclusion of Clipper2 costs from Phase 4 and handoff to Phase 5.

### Codebase Maps

- `.planning/codebase/TESTING.md` - standard workspace verification commands.
- `.planning/codebase/STRUCTURE.md` - core crate and test layout.
- `cavalier_contours/tests/test_utils/fixture_schema.rs` - existing
  `OracleComparable`, polygon model, comparison, and operation structures.
- `cavalier_contours/tests/test_utils/fixture_harness.rs` - executable fixture
  runner pattern.

### Clipper2 Sources

- `E:/Coding/Clipper2/Tests/Polygons.txt` - polygon boolean expected area/count
  source.
- `E:/Coding/Clipper2/Tests/Offsets.txt` - offset source paths for selected
  cases.
- `E:/Coding/Clipper2/CPP/Tests/TestPolygons.cpp` - Clipper2 polygon test load
  and tolerance behavior.
- `E:/Coding/Clipper2/CPP/Tests/TestOffsets.cpp` - offset test setup and
  ClipperOffset usage.
- `E:/Coding/Clipper2/CPP/Clipper2Lib/include/clipper2/clipper.h` - public
  clipping API.
- `E:/Coding/Clipper2/CPP/Clipper2Lib/include/clipper2/clipper.offset.h` -
  public offset API.

</canonical_refs>

<code_context>
## Existing Code Insights

- The Phase 2/3 fixture schema already has `UsageLabel::OracleComparable`,
  `GeometryModel::PolygonPath`, `ComparisonMode::{ApproximateParity,
  NotComparable, Gap}`, and property-based expected data.
- `FixtureOperation::Boolean`, `FixtureOperation::Offset`, and
  `FixtureOperation::Properties` can represent selected Clipper2 oracle
  evidence without public API changes.
- Existing historical fixture tests demonstrate provenance assertions,
  executable fixtures, metadata-only records, and gap records.
- The current Rust boolean API operates on two closed polylines and reports
  positive/negative result polylines; Clipper2 can represent broader multi-path,
  open-path, and polytree cases that need careful exclusion.

</code_context>

<specifics>
## Specific Ideas

- Start with `05-CLIPPER2-INVENTORY.md` to classify `Polygons.txt`,
  `Offsets.txt`, `TestPolygons.cpp`, `TestOffsets.cpp`, polytree/hole tests,
  line/open-path tests, and triangulation.
- Add `test_clipper2_oracle_fixtures.rs` for selected executable cases and
  metadata-only not-comparable records.
- Add a dev-only report path that can emit `target/clipper2-oracle/report.md`
  or print equivalent evidence, while committing only concise planning evidence.
- Use area/result-count property comparison and tolerances that cite Clipper2
  source behavior.

</specifics>

<deferred>
## Deferred Ideas

- Live Clipper2 C++ build integration as a mandatory cargo gate.
- Broad parsers for all Clipper2 text fixtures.
- Triangulation, rect clipping, Minkowski, and UsingZ behavior.
- Production Clipper2 backend or public API/FFI changes.
- Robustness fixes based on oracle gaps; Phase 6 owns fixes and ranking.

</deferred>

---

*Phase: 05-clipper2-oracle-boundary*
*Context gathered: 2026-05-12*

