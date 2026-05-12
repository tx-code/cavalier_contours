# Phase 07 Capability Design

## Selected candidate

`rect-clip-convenience`

## Source Provenance

| Source | Evidence | Role |
|--------|----------|------|
| Phase 1 audit | `.planning/phases/01-absorption-contract-audit/01-AUDIT.md` records Clipper2 `TestRectClip.cpp` and `CPP/Examples/RectClipping` as reference sources. | Capability evidence |
| Phase 5 inventory | `.planning/phases/05-clipper2-oracle-boundary/05-CLIPPER2-INVENTORY.md` classifies rect clipping as deferred metadata-only evidence. | Eligibility boundary |
| Phase 7 candidates | `.planning/phases/07-capability-absorption-pipeline/07-CAPABILITY-CANDIDATES.md` selects this as the first `absorb-now` slice. | Selection decision |

## Behavior Contract

Add a convenience API for clipping a closed polyline to an axis-aligned rectangle.
The implementation must not call Clipper2. It should build a closed rectangle
polyline and use the existing Rust boolean intersection path:

- `rect_clip(rect)` calls `rect_clip_opt(rect, &Default::default())`.
- `rect_clip_opt(rect, options)` constructs a counter-clockwise rectangle from
  `rect.min_x`, `rect.min_y`, `rect.max_x`, and `rect.max_y`, normalizing swapped
  min/max coordinates.
- The operation is equivalent to `self.boolean_opt(&rect_pline, BooleanOp::And,
  options)`.
- Results use the existing `BooleanResult<Self::OutputPolyline>` type.
- The method is scoped to the current boolean assumptions: closed,
  non-self-intersecting area polylines.

## Semantic fit

`arc-aware`.

The source inspiration is Clipper2's polygon RectClip family, but the Rust slice
uses the existing arc-aware boolean implementation. Native bulge arcs are not
converted to Clipper2 paths, and Clipper2 does not become a backend.

## Implementation files

- `cavalier_contours/src/polyline/traits.rs`

## Test files

- `cavalier_contours/tests/test_pline_boolean.rs`

## Targeted test command

```powershell
cargo test -p cavalier_contours --test test_pline_boolean rect_clip -- --nocapture
```

## Public Rust API impact

Add two default methods to `PlineSource`:

- `fn rect_clip(&self, rect: static_aabb2d_index::AABB<Self::Num>) -> BooleanResult<Self::OutputPolyline>`
- `fn rect_clip_opt(&self, rect: static_aabb2d_index::AABB<Self::Num>, options: &PlineBooleanOptions<Self::Num>) -> BooleanResult<Self::OutputPolyline>`

Both methods reuse existing `PlineBooleanOptions` and `BooleanResult` types.

## FFI impact

none

No C ABI function is added in Phase 7. Phase 8 may decide whether the
convenience belongs in the C FFI.

## Generated header impact

none

Do not regenerate `cavalier_contours_ffi.h`.

## Example/docs impact

required

Update `examples/boolean_ops.rs` with a concise `rect_clip` example using
`static_aabb2d_index::AABB`.

## UI impact

none

The existing boolean demo can already visualize equivalent rectangle
intersection if needed. No dedicated UI control is required for this small API
slice.

## Execution Guardrails

- Keep the core crate free of unsafe code.
- Do not add a production Clipper2 dependency or backend.
- Do not add broad corpus parsers for Clipper2 rect clipping tests.
- Do not regenerate the FFI header because the ABI does not change.
- Keep UI untouched because `UI impact` is `none`.

## Requirement Coverage

| Requirement | Coverage |
|-------------|----------|
| `CAP-02` | The selected slice preserves the Rust arc-aware model by routing through existing boolean behavior. |
| `CAP-03` | Public Rust API impact, tests, and example/docs impact are explicit. |
| `DEM-01` | UI impact is explicitly marked `none`; demo UI changes are not needed for this slice. |
