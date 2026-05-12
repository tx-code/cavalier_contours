# Concerns

Generated: 2026-05-12

## Algorithm Sensitivity

Offset, boolean, and intersection code is epsilon-sensitive. Existing tests use
tolerances around `1e-5` and `1e-4`. Small changes can create regressions in
repeat vertices, tangencies, overlapping arcs, or stitch selection.

## Boolean Scope

The README documents that boolean operations operate on two closed,
non-self-intersecting polylines. Do not assume general polygon clipping parity
without first expanding the contract and tests.

## Arc Representation

Bulge arcs are documented as supporting values from `-1.0` to `1.0`, up to a
half-circle per segment. Larger arcs require multiple arc segments.

## FFI Drift

The generated root header can drift from `cavalier_contours_ffi/src/lib.rs`.
Any FFI surface change should update tests and regenerate `cavalier_contours_ffi.h`.

## Unsafe Boundary

The core crate forbids unsafe code. All pointer safety risk is concentrated in
the FFI crate, where null checks, bounds checks, and panic handling must stay
consistent.

## External Reference Code

The older C++ CavalierContours codebase is useful as historical algorithm
reference. Clipper2 is useful for polygon-only robustness comparisons. Neither
should be treated as a drop-in replacement for this crate's arc-aware polyline
model.

## UI Productization

The UI is a demo surface. It contains TODOs around scene caching and color
pickers, and it should not be treated as a production UX without additional
hardening.

## Documentation Details

The root Cargo metadata homepage appears to use `wwww.cavaliercontours.dev`,
while the Pages workflow uses `www.cavaliercontours.dev`. Confirm intended URL
before publishing metadata updates.

## Planning Interaction

`AGENTS.md` already exists as an untracked contributor guide. If GSD later
generates agent instructions, merge with that file instead of overwriting it.
