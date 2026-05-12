# Phase 07 Pattern Map

## Purpose

Map Phase 7 artifacts and likely code surfaces to existing repository patterns
so execution can stay narrow and consistent.

## Planning Artifacts

| New artifact | Closest analog | Pattern to follow |
|--------------|----------------|-------------------|
| `07-CAPABILITY-CANDIDATES.md` | `06-ROBUSTNESS-BACKLOG.md` | Ranked table with scoring, decision, and defer/no-fix notes. |
| `07-CAPABILITY-DESIGN.md` | `05-ORACLE-EVIDENCE.md`, `06-VERIFICATION.md` | Decision-oriented document with provenance, impact notes, and verification evidence. |
| `07-VERIFICATION.md` | `06-VERIFICATION.md` | Requirement closure table plus exact command results. |

## Test Patterns

| Test need | Existing analog | Pattern to reuse |
|-----------|-----------------|------------------|
| Imported/reference cases | `test_historical_cavalier_contours.rs` | Fixture provenance, executable fixtures, metadata-only records. |
| Oracle-derived cases | `test_clipper2_oracle_fixtures.rs` | Oracle classification and report-as-evidence behavior. |
| Geometry property checks | `tests/test_utils/pline_test_properties.rs` | Compare area, path length, extents, orientation, and user data where relevant. |
| Shape or offset regressions | `test_shape_parallel_offset.rs`, `test_pline_parallel_offset.rs` | Small focused inputs with exact expected property sets. |

## Code Patterns

| Code surface | Existing pattern | Notes |
|--------------|------------------|-------|
| `PlineSource` | Trait default methods in `polyline/traits.rs` | Public behavior should route through existing trait style when possible. |
| Internal algorithms | `polyline/internal/pline_offset.rs`, `pline_boolean.rs` | Keep implementation helpers private unless public API is deliberately needed. |
| Shape behavior | `shape_algorithms/mod.rs` | Preserve closed-area boundary semantics and explicit open/closed treatment. |
| FFI | `cavalier_contours_ffi/src/lib.rs` | ABI additions need status-code handling and header regeneration. |

## Execution Guidance

- Prefer adding tests before or with implementation.
- Keep selected capability code in the smallest existing module that owns the
  behavior.
- If the selected slice only adds evidence or examples, record why no production
  code changed.
- If public API changes, add example/docs coverage in the same phase.
