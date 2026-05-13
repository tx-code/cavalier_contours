# Phase 43: C-API Logic Alignment Map

This map captures next steps after introducing drift-failure triage template
and flow.

## Triage Outcome

- Deterministic template and command flow now define how drift-check failures
  are converted into source-explicit parity actions.
- Drift response no longer depends on ad-hoc interpretation.

## Next Alignment Targets (No Clipper)

| Priority | Target | Rust file/module | Decision Boundary |
|----------|--------|------------------|-------------------|
| P1 | Options-path coincident vertex-output deepening | `cavalier_contours_ffi/tests/test_pline.rs` | Add only source-explicit coincident cases with concrete old-C++ provenance. |
| P1 | Drift report instantiation on first real drift | `.planning/phases/*` | Generate triage report from template at first actual drift event. |
| P2 | FFI parity helper extraction | `cavalier_contours_ffi/tests/test_pline.rs` | Refactor only when semantic behavior is unchanged. |

## File-Level Alignment Surface

- Triage artifacts:
  - `.planning/tools/cpp_suite_drift_triage_template.md`
  - `.planning/phases/43-capi-drift-failure-triage-template/43-CPP-SUITE-DRIFT-TRIAGE-FLOW.md`
- Next test target:
  - `cavalier_contours_ffi/tests/test_pline.rs`
