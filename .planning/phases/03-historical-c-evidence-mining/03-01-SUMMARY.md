---
phase: 03-historical-c-evidence-mining
plan: 03-01
subsystem: testing
tags: [rust, geometry-fixtures, historical-cpp, evidence-inventory]
requires:
  - phase: 01
    provides: provenance and source usage contract
  - phase: 02
    provides: typed fixture schema and harness taxonomy
provides:
  - Phase 3 historical C++ evidence inventory
  - Selected executable fixture IDs for offset, boolean, and property evidence
  - Metadata-only C API and static spatial index records
affects: [fixtures, historical-cpp-mining, benchmarks, ffi-migration]
tech-stack:
  added: []
  patterns: [inventory-then-translate, metadata-only evidence, property parity selection]
key-files:
  created:
    - .planning/phases/03-historical-c-evidence-mining/03-INVENTORY.md
  modified: []
key-decisions:
  - "Selected a small curated old C++ import set covering offset, boolean, and pure property evidence."
  - "Classified C API and static spatial index evidence as metadata-only for Phase 3."
patterns-established:
  - "Historical evidence rows record source path, usage label, comparison mode, tolerance policy, and fallback classification."
requirements-completed:
  - FIX-03
duration: 2 min
completed: 2026-05-12
---

# Phase 03 Plan 01: Inventory Historical C++ Evidence Summary

**Curated historical C++ evidence inventory for executable fixture translation and metadata-only migration records**

## Performance

- **Duration:** 2 min
- **Started:** 2026-05-12T12:30:16+08:00
- **Completed:** 2026-05-12T12:32:12+08:00
- **Tasks:** 3
- **Files modified:** 1

## Accomplishments

- Created `03-INVENTORY.md` with the old C++ repo snapshot, commit, license, and default tolerance policy.
- Selected four concrete fixture IDs spanning offset, boolean/combine, and pure property evidence.
- Recorded C API and static spatial index evidence as metadata-only, with benchmark and FFI work deferred out of Phase 3.

## Task Commits

1. **Historical C++ evidence inventory** - `5d4c47f` (`docs(03-01)`)

**Plan metadata:** this summary commit.

## Files Created/Modified

- `.planning/phases/03-historical-c-evidence-mining/03-INVENTORY.md` - Inventories selected old C++ evidence, metadata-only records, deferred evidence, and requirement coverage.

## Decisions Made

- Used `historical-cpp-offset-closed-rectangle-inward`, `historical-cpp-offset-collapsed-rectangle`, `historical-cpp-combine-circle-rectangle-union`, and `historical-cpp-properties-ccw-circle-x-aligned` as the curated translation set.
- Kept C API and static spatial index evidence non-executable in Phase 3.
- Used Phase 2 default tolerance and property parity as the planned comparison policy.

## Deviations from Plan

None - plan executed exactly as written.

**Total deviations:** 0 auto-fixed.
**Impact on plan:** No scope changes.

## Issues Encountered

None.

## User Setup Required

None - no external service configuration required.

## Verification

- `Select-String -Path .planning\phases\03-historical-c-evidence-mining\03-INVENTORY.md -Pattern "31a012947aa2e7e9474e2ec90502825afe8b99a4","TEST_cavc_parallel_offset.cpp","TEST_cavc_combine_plines.cpp","TEST_cavc_pline_function.cpp","TEST_staticspatialindex.cpp","cavaliercontours.h"` - passed.
- `Select-String -Path .planning\phases\03-historical-c-evidence-mining\03-INVENTORY.md -Pattern "historical-cpp-offset-closed-rectangle-inward","historical-cpp-offset-collapsed-rectangle","historical-cpp-combine-circle-rectangle-union","historical-cpp-properties-ccw-circle-x-aligned"` - passed.
- `Select-String -Path .planning\phases\03-historical-c-evidence-mining\03-INVENTORY.md -Pattern "historical-cpp-c-api-surface-migration-record","historical-cpp-static-spatial-index-query-record","migration-sensitive","benchmark-candidate","Phase 4"` - passed.
- `Test-Path .planning/phases/03-historical-c-evidence-mining/03-INVENTORY.md` - passed.
- `Select-String -Path .planning\phases\03-historical-c-evidence-mining\03-INVENTORY.md -Pattern "FIX-03","Selected Executable Fixture Candidates","Metadata-Only Evidence"` - passed.
- `git diff --check` - passed.

## Next Phase Readiness

Plan 03-02 can translate the selected fixture IDs into typed Rust fixture cases.

---
*Phase: 03-historical-c-evidence-mining*
*Completed: 2026-05-12*

