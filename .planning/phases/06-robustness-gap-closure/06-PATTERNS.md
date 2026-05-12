# Phase 06 Pattern Map

**Generated:** 2026-05-12
**Scope:** Robustness gap closure planning

## Planned Files and Closest Analogs

| Planned file | Role | Closest analog | Pattern to reuse |
|--------------|------|----------------|------------------|
| `.planning/phases/06-robustness-gap-closure/06-ROBUSTNESS-BACKLOG.md` | Ranked robustness backlog | `03-INVENTORY.md`, `05-ORACLE-EVIDENCE.md` | Tables with evidence, score, decision, and promoted test/fix action. |
| `cavalier_contours/tests/test_shape_parallel_offset.rs` | Focused shape robustness regressions | Existing shape offset tests and repeat-position polyline offset tests | Use `Shape::from_plines`, `parallel_offset`, and property assertions; avoid new test framework. |
| `cavalier_contours/src/shape_algorithms/mod.rs` | Narrow production fix | Existing `Shape::from_plines` and `IndexedPolyline::new` structure | Keep API unchanged; sanitize or skip invalid input before index bounds are expected. |
| `.planning/phases/06-robustness-gap-closure/06-VERIFICATION.md` | Phase completion evidence | `05-VERIFICATION.md` | Record exact commands, status, scope check, and requirement coverage. |

## Local Patterns

- Keep regression tests in existing integration test files when they naturally
  extend a behavior surface.
- Prefer property-based assertions over vertex-order expectations.
- Use existing default tolerances unless a specific source requires an
  override.
- Keep public API and FFI unchanged for robustness fixes.

## First Fix Pattern

For shape input hardening:

- Normalize repeat/redundant input before creating `IndexedPolyline`.
- Skip collapsed or empty normalized polylines.
- Preserve orientation classification for valid CCW/CW loops.
- Add tests that would fail by panic or wrong result without the hardening.

## Scope Guards

- No Clipper2 dependency.
- No FFI header regeneration.
- No UI changes.
- No benchmark changes beyond using Phase 4 data for ranking.

