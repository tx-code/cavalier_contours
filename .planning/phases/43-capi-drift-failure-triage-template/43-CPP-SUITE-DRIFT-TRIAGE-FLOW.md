# Phase 43: C++ Suite Drift Triage Flow

## Trigger

Run:

```powershell
powershell -ExecutionPolicy Bypass -File .planning/tools/cpp_suite_drift_check.ps1
```

If exit code is non-zero, start triage.

## Deterministic Flow

1. Copy `.planning/tools/cpp_suite_drift_triage_template.md` into current phase
   workspace as a working triage document.
2. Paste drift command output into **Drift Snapshot**.
3. Map changed old-C++ blocks to current FFI evidence.
4. Classify each changed block as `covered`, `equivalent`, or `gap`.
5. For each `gap`, define required tests and target files/plans.
6. Execute fixes and close with gate checklist.

## Re-Audit Boundary

- Do not update drift baseline until gap items are resolved and closure gates are
  all green.
- Drift failure indicates alignment review work, not automatic algorithm failure.
