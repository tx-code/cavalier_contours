# Phase 08 Compatibility Audit

Date: 2026-05-12

## Scope

Audit compatibility impact of the Phase 7 absorbed API slice
(`rect-clip-convenience`) across Rust API, C FFI, and generated C header.

## Source Evidence

- `cavalier_contours/src/polyline/traits.rs`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi.h`
- Phase 7 implementation commit: `bb7026a`
- `.planning/phases/07-capability-absorption-pipeline/07-CAPABILITY-DESIGN.md`

## Rust API Delta

- Added `PlineSource::rect_clip(rect)` default method.
- Added `PlineSource::rect_clip_opt(rect, options)` default method.
- Both methods are additive and route to existing boolean behavior.

## Compatibility Classification

`non-breaking` for Rust consumers.

Reason: The change adds trait default methods and does not remove or change
existing signatures or behavior contracts.

## FFI Delta

No C ABI changes were introduced in Phase 7.

- No new `cavc_*` symbol for rectangle clipping convenience was added.
- Existing FFI API remains the same.

## Header Delta

No generated header delta is required for this phase.

- `cavalier_contours_ffi.h` does not need regeneration because ABI is unchanged.

## Release/Docs Implications

- Public docs and changelog should note new Rust `rect_clip` convenience API.
- FFI docs should explicitly state no new C wrapper for this API slice.

## Decision

Phase 8 will keep FFI/header unchanged and focus on compatibility notes and
migration guidance.
