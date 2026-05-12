# Integrations

Generated: 2026-05-12

## Public Rust API

The main integration surface is the `cavalier_contours` crate. Public exports
are rooted in `cavalier_contours/src/lib.rs`, which re-exports core geometry
modules, polyline APIs, shape algorithms, and `static_aabb2d_index`.

## C FFI

The `cavalier_contours_ffi` crate exposes a C ABI through `unsafe extern "C"`
functions in `cavalier_contours_ffi/src/lib.rs`.

Key integration rules:

- Status codes are `i32`.
- Null pointer and bounds handling are part of the ABI contract.
- Panics are converted with `ffi_catch_unwind!`.
- The root header is `cavalier_contours_ffi.h`.
- Regenerate the header only when the FFI surface changes.

## Generated Header

The project includes `buildFFI.sh`, which runs `cbindgen` for the FFI crate.
The README also documents the manual `cbindgen` invocation.

## Web Demo

The UI crate supports native and web builds:

- Native: run from `cavalier_contours_ui/` with `cargo run`.
- Web: run `trunk serve` after installing Trunk and the WASM target.
- GitHub Pages builds from `cavalier_contours_ui/` and deploys `dist`.

## Examples

The `examples` crate provides runnable API examples, including:

- `basic_polyline`
- `parallel_offsets`
- `boolean_ops`
- `intersections`
- `segments`

Use these as integration smoke tests when changing public behavior.

## External Algorithm References

The README identifies this repository as a Rust rewrite of the original C++
CavalierContours project. That codebase is useful as historical reference, but
the Rust crate is now the primary implementation target.
