# Spike: Variable Offset (`#19`)

Date: 2026-05-16  
Branch: `spike/variable-offset-exploration`

## Goal

Explore how to add variable offset distance (per-vertex/per-segment) without destabilizing current uniform offset behavior.

## Current Baseline (Code Evidence)

1. Public offset API is scalar-only (`offset: T`) and options do not carry a distance profile.
   - `PlineOffsetOptions` in `cavalier_contours/src/polyline/pline_types.rs:43`
   - `parallel_offset(..., offset: T, ...)` in `cavalier_contours/src/polyline/internal/pline_offset.rs:1556`

2. Raw offset construction assumes a single global distance.
   - `create_untrimmed_raw_offset_segs(..., offset: T)` in `.../pline_offset.rs:34`
   - line segments use `safe_unit_perp().scale(offset)`
   - arc segments apply `arc_radius + offs` with one `offs` derived from scalar `offset`

3. Join/trim path is built around rounded arc joins and uniform direction.
   - `connect_using_arc` in `.../pline_offset.rs:123`
   - repeated arc-based join routing across `line_line_join`, `line_arc_join`, `arc_line_join`, `arc_arc_join`

4. Validity filtering uses a global distance threshold from original polyline.
   - `point_valid_for_offset` in `.../pline_offset.rs:651`
   - computes `abs_offset = offset.abs() - offset_tol` then checks `dist > abs_offset^2`
   - this is the exact place where variable offsets break assumptions

5. Closed/open slicing also assumes one global offset magnitude.
   - `slices_from_raw_offset` (`...:690`)
   - `slices_from_dual_raw_offsets` (`...:1022`)
   - dual path uses one circle radius `offset.abs()`

6. Maintainer context from issue `#19` aligns with above:
   - distance checks and join/trim flow would require modification, likely a deep dive.

## Why This Is Not a Small Patch

Variable offset is not only an API extension. It touches:

- geometry generation (segment-by-segment offset distance),
- join semantics (current rounded-join assumptions),
- global validity checks (currently one scalar threshold),
- dual-offset intersection logic for open polylines.

So this is an algorithm extension, not a localized fix.

## Proposed Delivery Strategy

### Phase A (MVP): line-only variable offset

Scope:

- support open/closed polylines where all segments are lines (`bulge == 0`)
- new API only (do not change existing scalar API behavior)
- keep current rounded-join behavior where applicable

Proposed API shape:

- `parallel_offset_profile(&self, profile: &[T], options: &PlineOffsetProfileOptions<T>)`
- `profile.len() == vertex_count`
- profile interpolation mode:
  - `StepPerSegment` (simpler)
  - `LinearPerSegment` (better continuity)

### Phase B: generalized validity model

Replace scalar threshold test with local expected distance:

- nearest source segment + segment parametric `t`
- expected distance = profile interpolation at `t`
- validity condition based on `abs(actual_dist - expected_dist) <= tol_local`

### Phase C: arc segment support

Hard part:

- variable offset of circular arcs is generally not circular
- current bulge representation cannot directly represent all variable-distance arc offsets

Likely options:

1. strict mode: reject arc input for profile offset initially
2. approximation mode: subdivide arcs then run line-only profile offset (explicitly documented approximation)

## Test/Benchmark Plan

1. Regression safety:
   - full existing `cargo test --workspace` must remain green

2. New tests:
   - line-only variable offset fixtures (open/closed, convex/concave, self-intersecting)
   - continuity at segment joins for step/linear profile modes
   - failure contract for arc input in MVP strict mode

3. Performance:
   - benchmark against uniform offset baseline on same line-only inputs
   - track allocation delta and runtime delta

## Effort Estimate

- Phase A (line-only MVP): ~3-5 engineering days
- Phase B (distance model + robustness hardening): ~3-5 days
- Phase C (arc strategy): ~1-2+ weeks depending on exact fidelity target

## Recommendation

Proceed with Phase A first behind a new API, keep current `parallel_offset` untouched, and explicitly gate arc-input behavior in MVP.
