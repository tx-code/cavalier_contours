# Phase 02 Research: Fixture Schema and Property Harness

**Date:** 2026-05-12
**Phase:** 02 - Fixture Schema and Property Harness

## Purpose

Phase 2 should create a test-only Rust typed fixture schema and property
comparison harness. It should prove the shape with current Rust seed fixtures
only. It should not import old C++ or Clipper2 cases, should not add oracle
tooling, and should not touch public Rust or FFI APIs.

## Existing Test Assets

| Asset | Path | Use in Phase 2 |
|-------|------|----------------|
| Property helper | `cavalier_contours/tests/test_utils/pline_test_properties.rs` | Reuse `PlineProperties`, `create_property_set`, fuzzy AABB compare, and current epsilon semantics. |
| Polyline modifiers | `cavalier_contours/tests/test_utils/pline_modifiers.rs` | Keep available for future fixture variations; do not force all seed fixtures through modifiers. |
| Debug JSON helper | `cavalier_contours/tests/test_utils/debug.rs` | Use as inspiration for richer failure output, not as fixture format. |
| Offset tests | `cavalier_contours/tests/test_pline_parallel_offset.rs` | Shows current offset execution pattern and repeat-position assertions. |
| Boolean tests | `cavalier_contours/tests/test_pline_boolean.rs` | Shows current boolean result property extraction and `abs_area` comparison pattern. |

## Recommended Type Shape

- Add test-only modules under `cavalier_contours/tests/test_utils/`, such as
  `fixture_schema.rs` and `fixture_harness.rs`.
- Use a central `FixtureCase` type with:
  - `id`;
  - `provenance` with repo, commit, path, license, and usage label;
  - `geometry_model` enum;
  - `operation` enum with operation-specific input structs;
  - `comparison` enum matching the Phase 1 taxonomy;
  - `tolerance` policy;
  - expected property data required by comparison mode.
- Add a runner shaped like `run_fixture(&FixtureCase)` that dispatches by
  operation and comparison mode.

## Comparison Strategy

The existing `PlineProperties` compares vertex count, area, path length,
extents, and user data with one property epsilon. Phase 2 should centralize
those constants in a fixture-level policy while preserving defaults:

- property epsilon: `1e-4`;
- position epsilon: `1e-5`;
- remove-redundant epsilon: `1e-4`.

The default comparison should assert result count and per-result vertex count,
area, path length, and extents. Optional flags can include orientation,
open/closed state, repeat vertices, user data, and absolute-area comparison.

## Scope Guidance

- Use only current Rust behavior for seed fixtures.
- Add one executable seed each for offset, boolean, and contains/properties.
- Add one metadata-only `gap` or `not comparable` seed to prove non-executable
  taxonomy support.
- Do not create a fixture data directory in this phase.
- Do not add serde, JSON, RON, TOML, or schema-generation dependencies.
- Do not modify `cavalier_contours/src/lib.rs`, the FFI crate, or the generated
  header.

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Fixture schema becomes too broad and optional-field heavy. | Use operation-specific structs and minimum expected-property requirements per comparison mode. |
| Harness can pass without verifying behavior. | Exact and approximate executable modes must require non-empty expected properties. |
| Tolerance values become scattered. | Add one tolerance policy/helper and route comparisons through it. |
| Phase 2 drifts into old C++ or Clipper2 mining. | Keep all seed fixtures sourced from current Rust behavior and provenance-labeled accordingly. |
| Public API or FFI changes slip in. | Keep modules under `tests/test_utils/` only and verify no crate/FFI files changed. |

## Plan Recommendation

- `02-01`: Define typed fixture schema, provenance, taxonomy, operation inputs,
  and metadata collection.
- `02-02`: Extend property comparison and runner helpers with centralized
  tolerance policy and structured failures.
- `02-03`: Add current-Rust seed fixtures proving offset, boolean,
  contains/properties, and metadata-only taxonomy support.
