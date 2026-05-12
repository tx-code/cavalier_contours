# Phase 08 Discussion Log

Date: 2026-05-12

## Inputs

- Roadmap Phase 8 goals and plans.
- Requirements `API-01`, `API-02`, `API-03`.
- Phase 7 design and verification records.
- Current repository docs (`README.md`, `CHANGELOG.md`, `cavalier_contours_ffi/README.md`).

## Chosen Direction

1. Run an explicit compatibility audit first.
2. Keep FFI/header unchanged unless the audit reveals a required ABI delta.
3. Update release/docs compatibility notes for the new Rust API.
4. Add migration guidance targeted at old C++ CavalierContours users.

## Assumptions

- Phase 7 added Rust API (`rect_clip`, `rect_clip_opt`) without C ABI changes.
- Existing FFI ABI tests remain valid if the C surface is unchanged.
- Migration notes can be document-first in this phase.

## Risk Controls

- Verify no `cavalier_contours_ffi.h` change unless ABI changes.
- Keep changes scoped to docs and planning unless a concrete ABI gap appears.
- Run full workspace gates before phase completion.
