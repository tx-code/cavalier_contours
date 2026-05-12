# Phase 06: robustness-gap-closure - Research

**Researched:** 2026-05-12
**Domain:** Rust geometry robustness gap closure
**Confidence:** MEDIUM-HIGH

## Summary

The strongest Phase 6 path is not a broad boolean rewrite. The evidence points
to a narrow, defensible loop:

1. Rank robustness candidates across all required geometry risk families.
2. Add focused regressions for a high-confidence input-boundary gap.
3. Harden the smallest production surface needed.
4. Re-run the full workspace gate.

The best first fix target is shape offset input sanitization. The core polyline
offset path already has repeat-position regressions, while `Shape::from_plines`
still admits any polyline with more than one vertex before building spatial
indexes and later expecting bounds. That creates a credible degenerate/repeat
input risk with limited blast radius and clear tests.

## Evidence Sources

| Evidence | Phase 6 use |
|----------|-------------|
| Phase 3 historical fixtures | Confirms offset collapse/property cases are green; preserves one boolean vertex-count gap for ranking. |
| Phase 4 benchmarks | Identifies offset, boolean, intersection, and spatial-index-heavy surfaces; no optimization work in Phase 6. |
| Phase 5 oracle evidence | Confirms dev-only oracle path and supplies future deferred Clipper2 candidates. |
| Existing repeat-position offset tests | Shows repeat/degenerate input is a known robustness class already fixed at the polyline boundary. |
| `Shape::from_plines` implementation | Reveals shape input filtering is thinner than polyline offset sanitization. |

## Recommended First Fix

Harden `Shape::from_plines` so repeated-position, redundant, or collapsed input
polylines are normalized or skipped before spatial indexes are built. The
implementation should preserve valid loop orientation and user data, avoid API
churn, and keep shape offset behavior unchanged for valid inputs.

## Pitfalls

- Do not treat the historical boolean vertex-count gap as a correctness bug
  without a property/topology failure.
- Do not convert Clipper2 broad text fixtures into a parser project.
- Do not change default boolean collapsed-area behavior globally unless a
  focused regression proves it is the right narrow fix.
- Do not add performance thresholds or optimization work.

## Validation

- Targeted shape offset robustness test.
- `cargo test --workspace`.
- `cargo fmt --all --check`.
- `cargo clippy --all-targets -- -D warnings`.
- `git diff --check`.
- `gsd-sdk query validate.health --project-dir E:\Coding\cavalier_contours`.

## RESEARCH COMPLETE

