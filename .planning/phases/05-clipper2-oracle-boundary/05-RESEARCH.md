# Phase 05: clipper2-oracle-boundary - Research

**Researched:** 2026-05-12
**Domain:** Clipper2 polygon oracle evidence
**Confidence:** HIGH

<user_constraints>
## User Constraints from CONTEXT.md

- Keep Clipper2 as external oracle evidence only; no production backend.
- Use polygon-only, straight-segment, closed-path cases unless an explicit
  approximation note makes an arc case comparable.
- Prefer existing fixture schema and harness; do not add a broad parser unless
  it is clearly cheaper than manual curation.
- Generated oracle reports belong under `target/` and must not be committed.
- Triangulation, open paths, polytree ownership, rect clipping, Minkowski, and
  cleanup-only sources are deferred unless a specific case is clearly
  comparable.
</user_constraints>

<research_summary>
## Summary

Phase 5 should use Clipper2 as a small, curated oracle corpus rather than a
runtime dependency. The current Phase 2/3 fixture model already supports the
needed metadata: `UsageLabel::OracleComparable`, `GeometryModel::PolygonPath`,
`ComparisonMode::ApproximateParity`, `ComparisonMode::NotComparable`, and
property-based expected output.

The safest implementation path is:

1. Inventory Clipper2 text tests, C++ tests, and examples by eligibility.
2. Add a small Rust integration test with executable fixture cases for one
   simple boolean case and one offset collapse case.
3. Add metadata-only records for broad, multi-path, polytree, open-path, and
   offset sources that are useful but not directly executable.
4. Add a dev-only report path that classifies selected records as pass, gap,
   not comparable, or deferred without making Clipper2 a normal build gate.
</research_summary>

<source_findings>
## Source Findings

| Source | Useful evidence | Phase 5 treatment |
|--------|-----------------|-------------------|
| `Tests/Polygons.txt` | `CLIPTYPE`, `FILLRULE`, `SOL_AREA`, `SOL_COUNT`, subject and clip paths | Inventory and selected metadata; broad parser deferred. |
| `CPP/Tests/TestPolygons.cpp` | Loads `Polygons.txt`, executes `Clipper64`, compares area/count with test-specific tolerances | Use as tolerance and expected-property policy. |
| `CPP/Tests/TestPolytreeIntersection.cpp` | Small square intersection with one 4-vertex solution polygon | Executable boolean candidate because no hole ownership is required. |
| `CPP/Tests/TestOffsets.cpp` | Offset setup, join/end type, delta, and count/orientation expectations | Use simple collapse case as executable; keep broad cases metadata-only. |
| `Tests/Offsets.txt` | Large offset subjects with skipped stored area/count in the first C++ loop | Inventory only unless a manually selected case has stable expected properties. |
| `CPP/Examples/*` | Public API examples for clipping, inflation, rect clipping, random, and triangulation | Reference/deferred unless matching current Rust semantics is obvious. |
</source_findings>

<architecture_patterns>
## Architecture Patterns

### Fixture Representation

Use one test file:

```text
cavalier_contours/tests/test_clipper2_oracle_fixtures.rs
```

Each fixture should cite:

- repo `Clipper2`
- commit `f9c5eb6e14a59f6f5d65fbfb3564519a561cf4fd`
- license `Boost Software License 1.0`
- exact source path
- `UsageLabel::OracleComparable`
- polygon model and comparison mode

### Report Path

The report path may live inside the integration test and emit a Markdown report
only when `CAVC_CLIPPER2_ORACLE_REPORT=1` is set:

```text
target/clipper2-oracle/clipper2-oracle-report.md
```

The committed evidence doc should summarize the run; generated target output
must stay untracked.
</architecture_patterns>

<common_pitfalls>
## Common Pitfalls

- Treating Clipper2 as production behavior instead of oracle evidence.
- Executing multi-path, open-path, hole, or polytree cases without a clear
  mapping to the current Rust boolean API.
- Comparing vertex order instead of stable properties such as area, extents,
  path length, and result count.
- Hiding approximation details for offsets or arc-to-polygon comparisons.
- Committing generated reports under `target/`.
</common_pitfalls>

<validation_architecture>
## Validation Architecture

- `cargo test -p cavalier_contours --test test_clipper2_oracle_fixtures -- --nocapture`
- `$env:CAVC_CLIPPER2_ORACLE_REPORT="1"; cargo test -p cavalier_contours --test test_clipper2_oracle_fixtures -- --nocapture`
- `cargo test --workspace`
- `cargo fmt --all --check`
- `cargo clippy --all-targets -- -D warnings`
- `git diff --check`
- `gsd-sdk query state.validate`
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours`

## RESEARCH COMPLETE

