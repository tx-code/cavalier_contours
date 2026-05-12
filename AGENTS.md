# Repository Guidelines

## Project Structure & Module Organization

This is a Rust workspace for the `cavalier_contours` computational geometry library.

- `cavalier_contours/`: core Rust library, with public APIs in `src/lib.rs`, polyline code in `src/polyline/`, and math/traits in `src/core/`.
- `cavalier_contours/tests/`: integration tests and shared test utilities.
- `cavalier_contours_ffi/`: C FFI crate; the generated root header is `cavalier_contours_ffi.h`.
- `cavalier_contours_ui/`: egui/Trunk demo app for native and WASM builds.
- `examples/`: runnable examples such as `basic_polyline.rs` and `parallel_offsets.rs`.

## Build, Test, and Development Commands

- `cargo build --workspace`: build all workspace crates.
- `cargo test --workspace`: run all tests.
- `cargo fmt --all --check`: verify Rust formatting.
- `cargo clippy --all-targets -- -D warnings`: run lints with warnings treated as errors.
- `cargo doc --workspace --no-deps`: build documentation; CI uses `RUSTDOCFLAGS=-D warnings`.
- `cargo run --example basic_polyline`: run an example from `examples/`.
- `cargo build -p cavalier_contours_ffi --release`: build the shared FFI library.
- From `cavalier_contours_ui/`, run `trunk serve` for the web demo after installing `trunk` and the `wasm32-unknown-unknown` target.

## Coding Style & Naming Conventions

Use Rust 2024 edition and keep the MSRV at Rust 1.88 unless deliberately changed. Follow `rustfmt` defaults. Prefer descriptive snake_case for functions, modules, files, and tests; use CamelCase for types and traits. Keep public APIs documented when they are part of the library surface. The core crate should remain safe Rust unless a change explicitly justifies otherwise.

## Testing Guidelines

Add integration tests under `cavalier_contours/tests/` for geometry behavior and regression cases. Name test files by feature, following existing patterns such as `test_pline_parallel_offset.rs` and `test_line_circle_intersect.rs`. Put reusable helpers in `tests/test_utils/`. Run `cargo test --workspace` before submitting, and include targeted edge cases for numerical tolerance, degenerate geometry, and open/closed polyline behavior.

## Commit & Pull Request Guidelines

Recent history uses Conventional Commit style, for example `fix(offset): ...`, `docs: ...`, and `chore(examples): ...`. Keep commit subjects imperative and scoped when useful.

Pull requests should describe the behavior change, link relevant issues, list commands run, and call out any API or FFI header impact. Include screenshots or recordings for UI changes in `cavalier_contours_ui/`.

## Agent-Specific Instructions

Keep changes scoped to the relevant crate. Do not regenerate `cavalier_contours_ffi.h` unless the FFI surface changed. Prefer the workspace-level CI commands above for final verification.

## GSD Project Context

This repository is now tracked through GSD planning in `.planning/`. The active project is a long-running absorption roadmap: keep Rust `cavalier_contours` as the only mainline implementation while using old C++ CavalierContours and Clipper2 as references, fixture sources, benchmark sources, and polygon-only oracle evidence.

Before broad algorithm changes, build evidence first: audit behavior, classify comparability, add fixtures, establish benchmark baselines, and rank robustness gaps. Do not treat Clipper2 as an arc-aware backend, and keep triangulation out of scope until explicitly rescoped.

## GSD Workflow

Use GSD entry points for planned work so `.planning/ROADMAP.md`, `.planning/STATE.md`, and phase artifacts stay synchronized:

- `$gsd-discuss-phase <n>` before planning a phase.
- `$gsd-plan-phase <n>` to create executable phase plans.
- `$gsd-execute-phase <n>` to implement planned phase work.
- `$gsd-quick` or `$gsd-debug` for small fixes or investigations.

Avoid direct edits outside GSD for roadmap work unless the user explicitly asks to bypass the workflow.
