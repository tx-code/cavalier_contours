# Conventions

Generated: 2026-05-12

## Rust Style

- Use rustfmt defaults.
- Run `cargo fmt --all --check` before final submission.
- Treat warnings as errors; CI enforces `RUSTFLAGS=-D warnings`.
- Keep the workspace at Rust 2024 and MSRV 1.88 unless intentionally changed.

## Naming

- Functions, modules, files, and tests use `snake_case`.
- Types, traits, and enums use `CamelCase`.
- Public polyline types generally use the `Pline` prefix.
- FFI exported names use the `cavc_` prefix.

## Safety Boundary

- The core crate forbids unsafe code.
- Unsafe pointer work belongs in `cavalier_contours_ffi`.
- Do not introduce unsafe to the core crate without changing the crate-level
  safety contract deliberately.

## Public API Changes

- Document public APIs that are part of the library surface.
- Keep trait method behavior consistent across owned polylines and views.
- When changing public types with optional serde support, preserve serialized
  field compatibility unless a breaking change is intended.

## Geometry Style

- Preserve numeric tolerance intent in nearby code.
- Prefer existing epsilon option fields over introducing unrelated constants.
- For offset and boolean changes, add focused regression tests covering
  degenerate geometry, open and closed polylines, and arc behavior.

## FFI Style

- Validate null pointers and bounds before dereferencing.
- Preserve status-code meanings for existing functions.
- Wrap FFI entry points with panic catching.
- Regenerate `cavalier_contours_ffi.h` only when signatures or ABI structs change.

## UI Style

- Keep UI changes in `cavalier_contours_ui/`.
- Include screenshots or recordings for visible behavior changes.
- Avoid mixing demo-only state with core geometry logic.

## Commits

Recent history uses Conventional Commit style, such as `fix(offset): ...`,
`docs: ...`, and `chore(examples): ...`.
