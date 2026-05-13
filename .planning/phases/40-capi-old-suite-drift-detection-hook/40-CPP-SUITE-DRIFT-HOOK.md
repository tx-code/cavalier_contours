# Phase 40: C++ Suite Drift Hook Notes

## Command

```powershell
powershell -ExecutionPolicy Bypass -File .planning/tools/cpp_suite_drift_check.ps1
```

## Baseline Artifact

- `.planning/tools/cpp_suite_drift_baseline.json`

Contains:
- canonical old-suite source root path
- per-file SHA256 hash
- per-file extracted `TEST/TEST_F/TEST_P` block names

## Pass/Fail Semantics

- **Pass**: no file-hash or test-block drift from baseline.
- **Fail**: one or more files missing, hash changed, or test-block list changed.

## Failure Handling

When the command fails:
1. Re-run Phase 38 cross-suite checklist against updated source files.
2. Classify drift as covered/equivalent/gap.
3. Add or update FFI parity tests only for source-explicit deltas.
4. Refresh baseline only after re-audit closure.
