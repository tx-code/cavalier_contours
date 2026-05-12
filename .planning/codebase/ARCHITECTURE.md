# Architecture

Generated: 2026-05-12

## High-Level Shape

The repository is organized around a safe Rust geometry kernel, with separate
FFI and UI crates layered on top.

## Core Library Boundary

`cavalier_contours/src/lib.rs` forbids unsafe code with `#![forbid(unsafe_code)]`.
This makes the core crate the safe algorithmic boundary. Unsafe pointer and ABI
work is isolated in `cavalier_contours_ffi`.

## Geometry Model

The central model is `Polyline<T = f64>`:

- vertices are stored as `Vec<PlineVertex<T>>`;
- each vertex stores position and bulge;
- `is_closed` distinguishes open and closed paths;
- optional user data is stored as `Vec<u64>`.

Bulge values represent arcs between vertices. Larger arcs are represented by
multiple arc segments according to the documented limitations.

## Trait-Based API

Polyline behavior is exposed through traits:

- `PlineSource`: read-only geometry operations and algorithms.
- `PlineSourceMut`: mutation operations.
- `PlineCreation`: constructors and conversion helpers.

Most algorithm entry points are default trait methods, so callers can use the
same behavior across owned polylines and views.

## Algorithm Areas

- `polyline/pline_seg.rs`: line and arc segment geometry.
- `polyline/pline_seg_intersect.rs`: line-line, line-arc, and arc-arc segment intersections.
- `polyline/internal/pline_offset.rs`: raw offsets, joins, slicing, and stitching.
- `polyline/internal/pline_boolean.rs`: boolean intersection processing and stitching.
- `shape_algorithms/`: multi-polyline offset shapes with filled and hole loops.

## Spatial Indexing

Algorithms use AABB indexing through `static_aabb2d_index` to reduce broad-phase
intersection and containment work.

## FFI Layer

The FFI crate wraps Rust types in opaque handles such as `cavc_pline`,
`cavc_shape`, and `cavc_aabbindex`. It translates C option structs into Rust
options and returns explicit status codes.

## UI Layer

The UI crate is a demo and exploration tool. It uses egui scenes for polyline
offsetting, boolean operations, and multi-polyline shape offsets.
