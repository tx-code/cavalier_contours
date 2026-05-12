# Stack

Generated: 2026-05-12

## Overview

`cavalier_contours` is a Rust workspace for a 2D computational geometry
library centered on line-and-arc polylines.

## Languages

- Rust is the primary implementation language.
- The workspace uses Rust 2024 edition and `rust-version = "1.88"`.
- The examples crate uses Rust 2021 edition.
- The FFI surface exports a C ABI from Rust.

## Workspace Crates

- `cavalier_contours`: core geometry library.
- `cavalier_contours_ffi`: C FFI crate and shared library target.
- `cavalier_contours_ui`: egui/eframe demo app for native and WASM builds.
- `examples`: runnable example programs.

## Core Dependencies

- `num-traits`: numeric traits used by generic geometry code.
- `static_aabb2d_index`: spatial index for bounding-box based searches.
- Optional `serde`: serialization support for public geometry types.

## UI Dependencies

- `egui`, `eframe`, `egui_plot`, and `egui_extras`: demo UI.
- `lyon`: rendering support.
- `wasm-bindgen-futures` and `web-sys`: WASM build support.
- `env_logger` and `log`: native logging.

## Build Profiles

- Standard dev and release profiles are used for library development.
- `web-release` inherits from release and lowers opt level for Trunk builds.
- Dev builds optimize dependencies with `[profile.dev.package."*"]`.

## CI Environment

- CI runs on Ubuntu, Windows, and macOS.
- Build/test jobs use stable Rust.
- Lint and docs jobs pin Rust `1.88.0`.
- CI sets `RUSTFLAGS=-D warnings` and `RUSTDOCFLAGS=-D warnings`.
