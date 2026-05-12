# Phase 03 Pattern Map

**Generated:** 2026-05-12
**Scope:** Historical C++ evidence mining plans

## Planned Files and Closest Analogs

| Planned file | Role | Closest analog | Pattern to reuse |
|--------------|------|----------------|------------------|
| `.planning/phases/03-historical-c-evidence-mining/03-INVENTORY.md` | Phase evidence inventory | `.planning/phases/01-absorption-contract-audit/01-AUDIT.md` and `01-PROVENANCE.md` | Markdown tables with source path, usage label, risk, follow-up, and requirement coverage. |
| `cavalier_contours/tests/test_historical_cavalier_contours.rs` | Historical fixture integration test | `cavalier_contours/tests/test_fixture_harness.rs` | `mod test_utils;`, fixture builder functions, `run_fixture(&fixture)`, metadata assertions. |
| `cavalier_contours/tests/test_utils/fixture_schema.rs` | Test-only fixture schema extension | Existing `FixtureOperation`, `ExpectedFixtureData`, `UsageLabel` enums | Add narrow variants only when needed; keep fields explicit and operation-specific. |
| `cavalier_contours/tests/test_utils/fixture_harness.rs` | Test-only fixture runner extension | Existing offset/boolean/contains branches | Dispatch by `FixtureOperation` and compare `PlineProperties` through centralized tolerance. |
| `cavalier_contours/tests/test_fixture_harness.rs` | Regression proof for schema extension | Existing current-Rust seed tests | Add or update only if schema/harness extension needs a current-Rust proof. |

## Source-of-Truth Patterns

### Fixture Construction

- Import `FixtureCase`, `FixtureOperation`, `ExpectedFixtureData`,
  `FixtureProvenance`, `FixtureTolerance`, `GeometryModel`, and `UsageLabel`
  from `test_utils`.
- Build geometry using `pline_closed!`, `pline_open!`, or `Polyline::new()` as
  existing tests do.
- Use `FixtureTolerance::default()` unless the old C++ source gives a specific
  reason for a per-case override.

### Property Comparison

- Expected property rows should use `PlineProperties::new(vertex_count, area,
  path_length, min_x, min_y, max_x, max_y, userdata)`.
- Prefer unordered property-set comparison through the harness.
- For old boolean cases that used `EqIgnoreSignOfArea`, set
  `PropertyExpectationOptions { compare_abs_area: true, ..Default::default() }`
  instead of asserting signed area.

### Metadata-Only Records

- Use `ComparisonMode::Gap` for in-scope behavior that does not currently pass.
- Use `ComparisonMode::NotComparable` for C API or spatial-index evidence that
  cannot execute through the current fixture harness.
- Keep metadata records non-executable with `ExpectedFixtureData::MetadataOnly`.

## Implementation Landmines

- Do not change `cavalier_contours_ffi/src/lib.rs` or
  `cavalier_contours_ffi.h` for Phase 3.
- Do not copy broad C++ implementation code. Translate test data and expected
  properties with provenance.
- Do not rely on old C++ vertex order unless the selected case explicitly tests
  vertex order. Property parity is the default.
- Validate old combine cases before making them executable; one constructor in
  `TEST_cavc_combine_plines.cpp` has a suspicious `plineB` initialization.
