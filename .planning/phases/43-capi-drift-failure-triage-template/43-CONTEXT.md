# Phase 43: capi-drift-failure-triage-template - Context

**Gathered:** 2026-05-14  
**Status:** Ready for execution

## Phase Boundary

Phase 43 adds deterministic triage structure for old-suite drift-hook failures
so follow-up parity work remains source-explicit and repeatable.

## Decisions

- **D-01:** Triage template must capture drift evidence, mapping, classification,
  and action decision in one artifact.
- **D-02:** Keep command flow tied to existing drift-check hook.
- **D-03:** Treat this phase as planning/tooling only; no geometry algorithm
  changes.

## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/tools/cpp_suite_drift_check.ps1`
- `.planning/tools/cpp_suite_drift_baseline.json`
- `.planning/phases/40-capi-old-suite-drift-detection-hook/40-CPP-SUITE-DRIFT-HOOK.md`
