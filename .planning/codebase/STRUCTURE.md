# Structure

Generated: 2026-05-12

## Repository Root

- `Cargo.toml`: workspace package metadata, members, shared dependencies, and profiles.
- `Cargo.lock`: locked dependency graph.
- `README.md`: feature overview, limitations, examples, and FFI notes.
- `cavalier_contours_ffi.h`: generated C header committed at the root.
- `buildFFI.sh`: helper for regenerating the C header.

## Core Crate

- `cavalier_contours/src/lib.rs`: crate docs, public modules, and re-exports.
- `cavalier_contours/src/core/`: math primitives, traits, and numeric helpers.
- `cavalier_contours/src/polyline/`: polyline data model, public traits, algorithms, and internals.
- `cavalier_contours/src/shape_algorithms/`: shape offset support across multiple polylines.
- `cavalier_contours/src/macros.rs`: polyline construction and fuzzy assertion macros.

## Core Tests

Integration tests live under `cavalier_contours/tests/`.

Important groups:

- geometric primitives: line, circle, and segment intersections;
- polyline basics, views, winding, contains, and booleans;
- parallel offset and shape parallel offset regression coverage;
- `test_utils/`: shared property comparison and modified-polyline helpers.

## FFI Crate

- `cavalier_contours_ffi/src/lib.rs`: raw pointer API, option structs, and ABI functions.
- `cavalier_contours_ffi/tests/`: C-ABI-level behavior tests from Rust.
- `cavalier_contours_ffi/Cargo.toml`: `lib` and `cdylib` outputs.

## UI Crate

- `cavalier_contours_ui/src/app.rs`: persisted main app state.
- `cavalier_contours_ui/src/main.rs`: native and WASM entry points.
- `cavalier_contours_ui/src/editor/`: polyline editor components.
- `cavalier_contours_ui/src/plotting/`: plotting helpers.
- `cavalier_contours_ui/src/scenes/`: offset, boolean, and shape demo scenes.
- `cavalier_contours_ui/Trunk.toml`: web build configuration.

## Examples

Examples are in the `examples` crate. Each named example is declared in
`examples/Cargo.toml` and can be run with `cargo run --example <name>`.

## Planning

GSD planning artifacts are expected under `.planning/`. This map is the initial
brownfield codebase baseline for future long-running work.
