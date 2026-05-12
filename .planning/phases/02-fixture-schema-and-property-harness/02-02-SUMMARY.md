---
phase: 02-fixture-schema-and-property-harness
plan: 02-02
subsystem: testing
tags: [rust, geometry-fixtures, property-comparison, tolerance]
requires:
  - phase: 02-01
    provides: typed fixture schema and operation-specific fixture inputs
provides:
  - Generic run_fixture(&FixtureCase) test runner
  - Central FixtureTolerance defaults and per-fixture option hooks
  - Layered property comparison with structured fixture failure output
  - Offset, boolean, and contains/properties dispatch paths
affects: [fixtures, regression-tests, old-cpp-mining, clipper2-oracle]
tech-stack:
  added: []
  patterns: [test-only fixture runner, layered property comparison]
key-files:
  created:
    - cavalier_contours/tests/test_utils/fixture_harness.rs
  modified:
    - cavalier_contours/tests/test_utils/fixture_schema.rs
    - cavalier_contours/tests/test_utils/pline_test_properties.rs
    - cavalier_contours/tests/test_utils/mod.rs
key-decisions:
  - "Kept fixture execution test-only and under tests/test_utils."
  - "Preserved existing PlineProperties comparison helpers while adding option-driven comparison for fixtures."
  - "Used FixtureTolerance defaults sourced from the existing property helper constants."
patterns-established:
  - "run_fixture dispatches executable exact/approx fixtures and records metadata-only cases without assertions."
  - "Property comparison failure messages include fixture id, provenance, operation, comparison mode, tolerance, actual, and expected data."
requirements-completed:
  - FIX-02
duration: 4 min
completed: 2026-05-12
---

# Phase 02 Plan 02: Add Comparison Harness and Tolerance Policy Summary

**Reusable fixture runner with centralized tolerance and layered property comparison**

## Performance

- **Duration:** 4 min
- **Started:** 2026-05-12T11:33:00+08:00
- **Completed:** 2026-05-12T11:37:24+08:00
- **Tasks:** 5
- **Files modified:** 4

## Accomplishments

- Added `run_fixture(&FixtureCase)` with offset, boolean, and contains/properties execution paths.
- Added `FixtureTolerance::default()` using the existing property, position, and remove-redundant epsilon constants.
- Extended property helpers with tolerance-aware creation and option-driven comparison.
- Added structured assertion output carrying fixture metadata and actual/expected property sets.

## Task Commits

1. **Fixture comparison harness** - `c17e980` (`feat(02-02)`)

**Plan metadata:** this summary commit.

## Files Created/Modified

- `cavalier_contours/tests/test_utils/fixture_harness.rs` - Runs typed fixtures, dispatches supported operations, and formats failures.
- `cavalier_contours/tests/test_utils/pline_test_properties.rs` - Adds tolerance-aware property extraction and layered comparison options.
- `cavalier_contours/tests/test_utils/fixture_schema.rs` - Adjusts comparison-option defaults for fixture-driven opt-ins.
- `cavalier_contours/tests/test_utils/mod.rs` - Exports the harness module.

## Decisions Made

- Kept existing `property_sets_match` behavior intact for existing tests.
- Added new option-driven comparison helpers instead of changing current tests.
- Routed metadata-only taxonomy fixtures through `run_fixture` without behavior assertions.

## Deviations from Plan

None - plan executed exactly as written.

**Total deviations:** 0 auto-fixed.
**Impact on plan:** No scope changes.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Verification

- `cargo fmt --all --check` - passed.
- `cargo test -p cavalier_contours --test test_pline_contains --no-run` - passed.
- `cargo test -p cavalier_contours --tests --no-run` - passed.
- `git diff --check` - passed with only repository line-ending warnings.

## Next Phase Readiness

Ready for Plan 02-03 to add current-Rust seed fixtures against the typed runner.

---
*Phase: 02-fixture-schema-and-property-harness*
*Completed: 2026-05-12*
