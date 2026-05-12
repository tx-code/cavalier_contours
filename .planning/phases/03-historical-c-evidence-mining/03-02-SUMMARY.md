---
phase: 03-historical-c-evidence-mining
plan: 03-02
subsystem: testing
tags: [rust, integration-tests, historical-cpp, fixture-harness]
requires:
  - phase: 03-01
    provides: selected historical C++ fixture IDs and metadata records
  - phase: 02
    provides: typed fixture schema and run_fixture harness
provides:
  - Test-only pure `Properties` fixture operation
  - Historical C++ offset and property executable fixtures
  - Metadata-only old C++ boolean gap, C API, and static spatial index records
affects: [fixtures, regression-tests, historical-cpp-mining]
tech-stack:
  added: []
  patterns: [test-only schema extension, historical fixture translation, gap fallback metadata]
key-files:
  created:
    - cavalier_contours/tests/test_historical_cavalier_contours.rs
  modified:
    - cavalier_contours/tests/test_utils/fixture_schema.rs
    - cavalier_contours/tests/test_utils/fixture_harness.rs
    - cavalier_contours/tests/test_fixture_harness.rs
key-decisions:
  - "Added a narrow test-only `Properties` operation for old C++ area/path/extents evidence."
  - "Converted the old C++ circle/rectangle union candidate to metadata-only `Gap` because current Rust preserves equivalent area/path/extents with fewer vertices."
patterns-established:
  - "Historical executable fixtures run through `run_fixture(&FixtureCase)`."
  - "Historical mismatches remain green by using `ComparisonMode::Gap` and `ExpectedFixtureData::MetadataOnly`."
requirements-completed:
  - FIX-03
duration: 4 min
completed: 2026-05-12
---

# Phase 03 Plan 02: Translate Curated Historical Fixtures Summary

**Typed Rust historical fixtures for old C++ offset/property evidence plus metadata-only gap and migration records**

## Performance

- **Duration:** 4 min
- **Started:** 2026-05-12T12:33:00+08:00
- **Completed:** 2026-05-12T12:36:59+08:00
- **Tasks:** 3
- **Files modified:** 4

## Accomplishments

- Added test-only `UsageLabel` variants and a pure `Properties` operation to the fixture schema.
- Extended `run_fixture` to execute `Properties` cases through centralized `PlineProperties` comparison.
- Added `test_historical_cavalier_contours.rs` with executable historical offset/property fixtures and metadata-only C API, spatial index, and boolean gap records.

## Task Commits

1. **Schema and runner support** - `7d8147b` (`feat(03-02)`)
2. **Historical fixture translation** - `1f98a68` (`test(03-02)`)

**Plan metadata:** this summary commit.

## Files Created/Modified

- `cavalier_contours/tests/test_utils/fixture_schema.rs` - Adds historical usage labels and pure `Properties` fixture data.
- `cavalier_contours/tests/test_utils/fixture_harness.rs` - Executes `Properties` fixtures through property-set matching.
- `cavalier_contours/tests/test_fixture_harness.rs` - Adds a current-Rust property seed for schema proof.
- `cavalier_contours/tests/test_historical_cavalier_contours.rs` - Defines historical executable fixtures and metadata-only records with old C++ provenance.

## Decisions Made

- Kept the schema extension inside test utilities only.
- Used old C++ commit `31a012947aa2e7e9474e2ec90502825afe8b99a4` and license `MIT` for every historical record.
- Classified `historical-cpp-combine-circle-rectangle-union` as a metadata-only gap after observing `vertex_count=8` in current Rust versus old C++ expected `vertex_count=10` with equivalent area, path length, and extents.

## Deviations from Plan

None - plan fallback rules were followed.

**Total deviations:** 0 auto-fixed.
**Impact on plan:** No scope changes.

## Issues Encountered

- The old C++ circle/rectangle union candidate did not satisfy vertex-count parity. It was converted to metadata-only `Gap`, keeping the suite green while preserving provenance and observed mismatch context.

## User Setup Required

None - no external service configuration required.

## Verification

- `cargo test -p cavalier_contours --test test_fixture_harness` - passed.
- `cargo test -p cavalier_contours --test test_fixture_harness -- --nocapture` - passed.
- `cargo test -p cavalier_contours --test test_historical_cavalier_contours -- --nocapture` - passed.
- `cargo fmt --all --check` - passed.
- `git diff --check` - passed.
- `git diff --name-only -- cavalier_contours/src cavalier_contours_ffi cavalier_contours_ffi.h` - no Phase 3 implementation changes.

## Next Phase Readiness

Plan 03-03 can add stronger metadata coverage assertions and synchronize final status in `03-INVENTORY.md`.

---
*Phase: 03-historical-c-evidence-mining*
*Completed: 2026-05-12*

