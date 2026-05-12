# Phase 01 Research: Absorption Contract Audit

**Date:** 2026-05-12
**Phase:** 01 - Absorption Contract Audit

## Purpose

Phase 1 is a documentation and decision phase. It creates the contract that later
fixture, benchmark, oracle, robustness, and capability-absorption phases will
consume. It must not import fixtures, port algorithms, or implement oracle
tooling.

## Source Snapshots

| Source | Path | Commit | License | Role |
|--------|------|--------|---------|------|
| Rust `cavalier_contours` | `E:/Coding/cavalier_contours` | `d2ba1c4e9d3eae4400701f0bf1033792a407e671` | `MIT OR Apache-2.0` | Fork-owned mainline |
| Old C++ CavalierContours | `E:/Coding/CavalierContours` | `31a012947aa2e7e9474e2ec90502825afe8b99a4` | `MIT` | Same-lineage historical reference |
| Clipper2 | `E:/Coding/Clipper2` | `f9c5eb6e14a59f6f5d65fbfb3564519a561cf4fd` | `Boost Software License 1.0` | Polygon-only oracle/reference |

## Audit Dimensions

The main audit should compare the three sources across geometry model,
construction and editing, line/arc segment math, intersections, containment and
winding, offsets, booleans, shape or multi-contour operations, spatial indexing,
cleanup, tests, benchmarks, examples, public APIs, C ABI/headers, documentation,
and limitations.

Every row needs a concrete source path or an explicit status: `not found`,
`not applicable`, or `deferred`.

## Recommended Artifacts

- `01-AUDIT.md`: cross-codebase matrix, source appendices, behavior taxonomy,
  candidate registry, and API/FFI surface comparison.
- `01-PROVENANCE.md`: repo snapshot table, license and acceptable-use rules,
  evidence ledger, and impact-note policy.

## Planning Recommendation

- `01-01`: Build the three-codebase capability inventory in `01-AUDIT.md`.
- `01-02`: Record license, provenance, and acceptable-use boundaries in
  `01-PROVENANCE.md`.
- `01-03`: Define behavior taxonomy and candidate registry in `01-AUDIT.md`.
- `01-04`: Compare API, FFI, and migration surfaces in `01-AUDIT.md`.

## Key Risks

- Clipper2 is polygon-focused and is not comparable to native bulge-arc behavior
  without an explicit approximation policy.
- Clipper2 triangulation is deferred and should not become a Phase 1 candidate.
- Old C++ is same-lineage but stale; it is a reference for behavior and evidence,
  not a porting target.
- Rust API and FFI are fork-owned and changeable, but future public surface
  changes still need an impact note.

## Validation Strategy

Validate the phase by checking that `01-AUDIT.md` and `01-PROVENANCE.md` exist,
all four `AUD-*` requirements are covered, all three source commits and licenses
are recorded, every main audit row contains evidence or an explicit status, and
triangulation appears only as deferred or out of scope.
