# Phase 08: api-ffi-and-migration-readiness - Context

**Gathered:** 2026-05-12  
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 8 hardens external surfaces after Phase 7's `rect_clip` absorption. The
focus is compatibility notes, FFI/header impact audit, and migration guidance
for users coming from old C++ CavalierContours. This phase does not introduce a
new geometry capability, does not broaden UI scope, and does not force an FFI
surface expansion when the absorbed API is Rust-only.

</domain>

<decisions>
## Implementation Decisions

- **D-01:** Start with an explicit compatibility audit for the Phase 7 public
  Rust API delta and current FFI/header state.
- **D-02:** Treat additive Rust trait default methods as non-breaking unless an
  existing public contract is changed.
- **D-03:** Regenerate `cavalier_contours_ffi.h` only if C ABI changes are
  actually introduced.
- **D-04:** If no FFI ABI change is needed, record an explicit "no FFI delta"
  compatibility note rather than adding placeholder C API.
- **D-05:** Migration notes must map old C++ concepts to current Rust crate and
  C FFI usage guidance, including current limitations.
- **D-06:** Keep core crate safe Rust and avoid broad refactors.
- **D-07:** Keep demo UI unchanged unless visual validation is required (not
  expected for this phase).
- **D-08:** Phase closure requires full workspace verification plus GSD
  `state.validate` and `validate.health`.

</decisions>

<canonical_refs>
## Canonical References

- `.planning/ROADMAP.md`
- `.planning/REQUIREMENTS.md`
- `.planning/STATE.md`
- `.planning/phases/07-capability-absorption-pipeline/07-CAPABILITY-DESIGN.md`
- `.planning/phases/07-capability-absorption-pipeline/07-VERIFICATION.md`
- `.planning/codebase/INTEGRATIONS.md`
- `README.md`
- `CHANGELOG.md`
- `cavalier_contours/src/polyline/traits.rs`
- `cavalier_contours_ffi/src/lib.rs`
- `cavalier_contours_ffi.h`
- `cavalier_contours_ffi/README.md`
- `examples/boolean_ops.rs`

</canonical_refs>

<specifics>
## Specific Ideas

- Record API and FFI delta in a single compatibility audit artifact.
- Add release-facing compatibility notes for `rect_clip` in `CHANGELOG.md` and
  user-facing docs.
- Produce migration notes that explain old C++ to Rust/FFI mapping and
  practical adoption steps.

</specifics>

<deferred>
## Deferred

- Broad C API redesign.
- UI redesign.
- New algorithm absorption beyond Phase 7 slice.

</deferred>

---

*Phase: 08-api-ffi-and-migration-readiness*  
*Context gathered: 2026-05-12*
