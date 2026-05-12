# Phase 06 Ranked Robustness Backlog

## Ranking Method

Score is 1-5 for each column. Higher total means better Phase 6 target.

| Column | Meaning |
|--------|---------|
| Correctness risk | User-visible wrong geometry, panic, or lost result risk. |
| Evidence quality | Backed by existing tests, source evidence, or code inspection. |
| Reproducibility | Can be tested deterministically in this repo. |
| Low blast radius | Can be fixed without broad API or algorithm rewrite. |
| Semantic fit | Matches current Rust arc-aware and closed/open behavior model. |

## Ranked Candidates

| Rank | Candidate ID | Risk family | Evidence | Correctness | Evidence | Repro | Low blast | Fit | Total | Decision |
|------|--------------|-------------|----------|-------------|----------|-------|-----------|-----|-------|----------|
| 1 | `shape-offset-repeat-degenerate-input` | offset, degenerate, repeat vertices, open/closed | `Shape::from_plines` filters only `vertex_count() > 1`; polyline offset already has repeat-position regressions, but shape offset lacks equivalent coverage. | 5 | 4 | 5 | 5 | 5 | 24 | Promote for 06-02/06-03. |
| 2 | `boolean-collapsed-area-thresholding` | boolean, tolerance, overlap | `test_pline_boolean.rs` uses `collapsed_area_eps: Some(1e-5)` in broad boolean property tests to avoid threshold inconsistencies; public default remains `None`. | 4 | 3 | 4 | 2 | 4 | 17 | Rank for later focused investigation; no default change without failing public-case regression. |
| 3 | `clipper2-polygons-017-intersection-evenodd` | boolean, intersection, tolerance, overlap | Phase 5 selected `Polygons.txt` case 17 with `SOL_AREA=14779`, `SOL_COUNT=1`, but kept it metadata-only due mapping cost. | 4 | 3 | 3 | 3 | 3 | 16 | Defer executable promotion unless Phase 6 first fix finishes early; no parser work. |
| 4 | `historical-cpp-combine-circle-rectangle-union` | boolean, overlap, tangent | Phase 3 metadata-only `Gap`: old C++ expected `vertex_count=10`; current Rust keeps equivalent area/path/extents with `vertex_count=8`. | 2 | 5 | 5 | 4 | 2 | 18 | No production fix unless property/topology failure is proven; keep as divergence evidence. |
| 5 | `offset-round-orientation-exterior-corpus` | offset, tolerance, overlap | Phase 5 `Offsets.txt` round polygon cases skip stored area/count and validate qualitative orientation/exterior properties. | 3 | 3 | 2 | 2 | 3 | 13 | Defer; needs better expected properties before execution. |
| 6 | `open-path-clipper-lines-suite` | open/closed, boolean | Clipper2 `Lines.txt` is open-path clipping; current Rust boolean fixture path is closed area polylines. | 3 | 4 | 2 | 1 | 1 | 11 | Not comparable in this phase. |
| 7 | `spatial-index-query-behavior-record` | intersection, spatial index | Phase 3 records old C++ static spatial index query IDs and early-stop behavior as metadata; Phase 4 has benchmark coverage. | 2 | 3 | 4 | 3 | 3 | 15 | Keep as benchmark/reference evidence, not Phase 6 first fix. |

## Required Risk Family Coverage

| Required family | Covered by |
|-----------------|------------|
| offset | `shape-offset-repeat-degenerate-input`, `offset-round-orientation-exterior-corpus` |
| boolean | `boolean-collapsed-area-thresholding`, `clipper2-polygons-017-intersection-evenodd`, `historical-cpp-combine-circle-rectangle-union` |
| intersection | `clipper2-polygons-017-intersection-evenodd`, `spatial-index-query-behavior-record` |
| tolerance | `boolean-collapsed-area-thresholding`, `offset-round-orientation-exterior-corpus` |
| degenerate | `shape-offset-repeat-degenerate-input` |
| repeat | `shape-offset-repeat-degenerate-input` |
| tangent | `historical-cpp-combine-circle-rectangle-union` |
| overlap | `boolean-collapsed-area-thresholding`, `historical-cpp-combine-circle-rectangle-union` |
| open/closed | `open-path-clipper-lines-suite`, `shape-offset-repeat-degenerate-input` |

## Promoted Fix Target

`shape-offset-repeat-degenerate-input` is the Phase 6 promoted target.

### Why This First

- It is a narrow input-boundary risk, not a broad algorithm rewrite.
- Existing polyline offset tests prove repeat-position inputs are a real
  robustness class.
- Shape offset lacks the equivalent focused regressions.
- The likely fix is localized to `Shape::from_plines` or a helper it calls.
- It should not require public API, FFI, UI, benchmark, or Clipper2 changes.

### Regression Status

Pending 06-02.

## No-Fix / Defer Decisions

- `historical-cpp-combine-circle-rectangle-union`: keep as a metadata-only gap
  for now. The recorded mismatch is vertex count, while area/path/extents are
  equivalent. That is not enough evidence for a production fix.
- `clipper2-polygons-017-intersection-evenodd`: keep as future oracle evidence.
  It may become a manual fixture later, but broad `Polygons.txt` parsing is out
  of Phase 6.
- Phase 4 benchmark evidence is used for prioritization only. No performance
  optimization or benchmark threshold work is in this phase.

## Requirement Coverage

| Requirement | Coverage |
|-------------|----------|
| `ROB-01` | This backlog ranks robustness candidates across offsets, booleans, intersections, tolerances, degenerates, repeat vertices, tangencies, overlaps, and open/closed behavior. |

