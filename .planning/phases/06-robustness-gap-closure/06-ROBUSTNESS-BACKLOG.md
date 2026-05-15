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
| 2 | `boolean-collapsed-area-thresholding` | boolean, tolerance, overlap | Broad boolean property tests already used `collapsed_area_eps: Some(1e-5)`; dedicated public default-path regressions now reproduce tiny collapsed-loop instability and validate stabilized behavior. | 4 | 5 | 5 | 3 | 4 | 21 | Closed by default hardening: `PlineBooleanOptions::new()` now sets `collapsed_area_eps=Some(1e-5)` with focused public regressions. |
| 3 | `clipper2-polygons-017-intersection-evenodd` | boolean, intersection, tolerance, overlap | Phase 5 selected `Polygons.txt` case 17 with `SOL_AREA=14779`, `SOL_COUNT=1`, but kept it metadata-only due mapping cost. | 4 | 3 | 3 | 3 | 3 | 16 | Defer executable promotion unless Phase 6 first fix finishes early; no parser work. |
| 4 | `historical-cpp-combine-circle-rectangle-union` | boolean, overlap, tangent | Phase 3 executable parity now validates old C++ geometry (`area/path/extents`) with `compare_vertex_count=false`; remaining difference is topology/representation (`10` vs `8` vertices after normalization). | 1 | 5 | 5 | 5 | 4 | 20 | Closed as geometry parity; keep only as topology delta evidence (no production algorithm fix queued). |
| 5 | `offset-round-orientation-exterior-corpus` | offset, tolerance, overlap | Phase 5 `Offsets.txt` round polygon cases skip stored area/count and validate qualitative orientation/exterior properties. | 3 | 3 | 2 | 2 | 3 | 13 | Defer; needs better expected properties before execution. |
| 6 | `open-path-clipper-lines-suite` | open/closed, boolean | Clipper2 `Lines.txt` is open-path clipping; current Rust boolean fixture path is closed area polylines. | 3 | 4 | 2 | 1 | 1 | 11 | Not comparable in this phase. |
| 7 | `spatial-index-query-behavior-record` | intersection, spatial index | Old C++ static spatial index query IDs and early-stop behavior now have executable parity (`test_cpp_static_spatial_index_parity`); Phase 4 still owns throughput benchmark coverage. | 1 | 5 | 5 | 5 | 4 | 20 | Correctness parity closed; keep only as benchmark/reference evidence, not Phase 6 first fix. |

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

06-02 added focused regressions for all-repeat-position loops, collinear closed
loops, open polyline input, and valid loops mixed with each invalid input class.
Current Rust already passed repeat-position and collinear closed-loop coverage.
The open polyline cases failed before the fix: open input returned a zero-area
closed result, and valid-plus-open input panicked in `PlineViewData`. 06-03 fixed
the promoted boundary by making `Shape::from_plines` skip `PlineOrientation::Open`
instead of treating open input as clockwise area loops.

## No-Fix / Defer Decisions

- `historical-cpp-combine-circle-rectangle-union`: closed as executable
  geometry parity in Phase 3 fixture harness. Remaining vertex-count delta is a
  representation detail, not a proven geometry correctness failure.
- `clipper2-polygons-017-intersection-evenodd`: keep as future oracle evidence.
  It may become a manual fixture later, but broad `Polygons.txt` parsing is out
  of Phase 6.
- Phase 4 benchmark evidence is used for prioritization only. No performance
  optimization or benchmark threshold work is in this phase.

## Requirement Coverage

| Requirement | Coverage |
|-------------|----------|
| `ROB-01` | This backlog ranks robustness candidates across offsets, booleans, intersections, tolerances, degenerates, repeat vertices, tangencies, overlaps, and open/closed behavior. |
