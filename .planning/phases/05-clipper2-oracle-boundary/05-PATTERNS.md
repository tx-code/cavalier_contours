# Phase 05 Pattern Map

**Generated:** 2026-05-12
**Scope:** Clipper2 oracle boundary planning

## Planned Files and Closest Analogs

| Planned file | Role | Closest analog | Pattern to reuse |
|--------------|------|----------------|------------------|
| `.planning/phases/05-clipper2-oracle-boundary/05-CLIPPER2-INVENTORY.md` | Classify Clipper2 sources and selected cases | `03-INVENTORY.md` and `04-BENCHMARK-MAP.md` | Markdown tables with source path, operation, eligibility, usage label, comparison mode, and rationale. |
| `cavalier_contours/tests/test_clipper2_oracle_fixtures.rs` | Rust oracle fixture and report tests | `test_historical_cavalier_contours.rs` | Local provenance helpers, typed `FixtureCase` builders, executable and metadata-only records. |
| `.planning/phases/05-clipper2-oracle-boundary/05-ORACLE-EVIDENCE.md` | Committed summary of oracle results | `04-BENCHMARKS.md` and `03-INVENTORY.md` | Record commands, selected fixture status, generated-output policy, and Phase 6 handoff. |
| `.planning/phases/05-clipper2-oracle-boundary/05-VERIFICATION.md` | Final phase gate evidence | `04-VERIFICATION.md` | List commands, status, artifacts, and requirement coverage. |

## Source-of-Truth Patterns

- Reuse `FixtureCase`, `FixtureProvenance`, `FixtureOperation`,
  `ExpectedFixtureData`, `FixtureTolerance`, and `run_fixture`.
- Use `GeometryModel::PolygonPath` for Clipper2 cases.
- Use `UsageLabel::OracleComparable` for executable and metadata-only oracle
  records.
- Use `ComparisonMode::ApproximateParity` for executable property comparison,
  `NotComparable` for excluded source families, and `Gap` only when a selected
  eligible case intentionally records current Rust mismatch.
- Keep generated reports under `target/clipper2-oracle/`.

## Candidate IDs

| Fixture ID | Source | Treatment |
|------------|--------|-----------|
| `clipper2-polytree-intersection-square-overlap` | `CPP/Tests/TestPolytreeIntersection.cpp` | executable boolean, one 4-vertex intersection polygon |
| `clipper2-offset-007-collapsed-square` | `CPP/Tests/TestOffsets.cpp` | executable offset collapse, Clipper2 miter negative delta mapped to Rust interior offset |
| `clipper2-polygons-017-intersection-evenodd` | `Tests/Polygons.txt`; `CPP/Tests/TestPolygons.cpp` | metadata selected candidate; executable only if manually verified |
| `clipper2-offsets-001-round-polygon` | `Tests/Offsets.txt`; `CPP/Tests/TestOffsets.cpp` | metadata/deferred because stored area/count are skipped |
| `clipper2-open-lines-suite` | `Tests/Lines.txt`; `CPP/Tests/TestLines.cpp` | not comparable for current closed-polyline fixture path |
| `clipper2-triangulation-suite` | triangulation headers/examples | deferred by project decision |

## Implementation Landmines

- Do not add Clipper2 as a Cargo dependency.
- Do not regenerate `cavalier_contours_ffi.h`.
- Do not change production files under `cavalier_contours/src/`.
- Do not treat `target/clipper2-oracle` output as a committed artifact.
- Do not use Clipper2 polygon output to redefine native bulge-arc semantics.

