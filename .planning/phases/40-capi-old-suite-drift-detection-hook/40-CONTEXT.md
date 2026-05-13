# Phase 40: capi-old-suite-drift-detection-hook - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 40 adds a deterministic drift-detection hook for canonical old C++ suite
files so cross-suite parity closure can be revalidated immediately when upstream
test sources change.

## Decisions

- **D-01:** Drift baseline includes both file hashes and extracted test-block
  names to detect semantic and structural changes.
- **D-02:** Hook remains planning/tooling scope and does not add runtime
  dependency from Rust tests to external old C++ repository paths.
- **D-03:** Hook failure is treated as a re-audit trigger, not an automatic
  algorithm bug claim.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/38-capi-cross-suite-coverage-audit/38-CROSS-SUITE-COVERAGE-CHECKLIST.md`
- `.planning/phases/39-capi-equivalence-zone-regression-hardening/39-EQUIVALENCE-HARDENING-MAP.md`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline.cpp`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_pline_function.cpp`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_parallel_offset.cpp`
- `E:/Coding/CavalierContours/tests/tests/TEST_cavc_combine_plines.cpp`
