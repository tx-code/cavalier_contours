# Migration from C++ CavalierContours

This guide helps existing C++ CavalierContours users adopt the Rust
`cavalier_contours` workspace with minimal risk.

## Migration Paths

1. Rust-first path:
   Move geometry logic to the `cavalier_contours` Rust API and keep your
   application boundary in Rust.
2. FFI-first path:
   Keep host application language unchanged and integrate through
   `cavalier_contours_ffi` C ABI.

## Concept Mapping

| C++ CavalierContours concept | Rust surface | C FFI surface |
|---|---|---|
| Polyline model (line + bulge arcs) | `polyline` module (`PlineSource`, `Polyline`) | `cavc_pline` and related functions |
| Parallel offset | `PlineSource::parallel_offset(_opt)` | `cavc_pline_parallel_offset` |
| Boolean operations | `PlineSource::boolean(_opt)` | `cavc_pline_boolean` |
| Containment / winding | `contains(_opt)`, `winding_number` | `cavc_pline_contains`, `cavc_pline_eval_wn` |
| Multi-polyline offset (`Shape`) | `Shape::parallel_offset` | shape-related `cavc_shape_*` functions |

## Phase 7 Additive API

Phase 7 added Rust-only convenience methods:

- `PlineSource::rect_clip`
- `PlineSource::rect_clip_opt`

These are additive and non-breaking for Rust users. A dedicated C FFI wrapper
for rectangle clipping is not included yet.

## Behavior and Limitations to Recheck During Migration

- Boolean operations are for two closed, non-self-intersecting polylines.
- Offset currently supports rounded joins only.
- Bulge arc values are limited to `[-1.0, 1.0]` per segment.
- Clipper2 is used as oracle/reference evidence, not production backend.

## Practical Migration Checklist

1. Port representative C++ geometry cases to Rust integration tests (or FFI
   harness tests) first.
2. Validate numerical tolerances and edge cases (repeat vertices, degenerates,
   open/closed behavior).
3. Run `cargo test --workspace` and compare output properties (area, extents,
   path length, orientation) instead of relying on vertex order.
4. If using FFI, keep `cavalier_contours_ffi.h` and shared library versions in
   sync with your deployed build.
5. Add migration-specific regression tests for each adopted workflow before
   retiring the old C++ path.
