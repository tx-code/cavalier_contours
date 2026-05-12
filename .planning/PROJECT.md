# Cavalier Contours Absorption Roadmap

## What This Is

This project evolves the Rust `cavalier_contours` crate as the single mainline
geometry kernel for line-and-arc 2D polylines. The older C++ CavalierContours
codebase and Clipper2 are reference sources for algorithms, behavior, fixtures,
benchmarks, and differential checks, not replacement implementations.

The work is intentionally larger than a few phases. It should be planned as a
multi-milestone effort that first builds evidence, then fixes high-risk current
behavior, then absorbs selected capabilities without destabilizing the existing
Rust API, FFI, or demo surfaces.

## Core Value

Make the Rust crate a robust, well-tested, arc-aware 2D geometry library whose
behavior is defensible against historical CavalierContours behavior and
polygon-only Clipper2 reference results.

## Requirements

### Validated

- [validated] Safe Rust core crate exists, with unsafe code forbidden at the
  library boundary.
- [validated] Core polyline model supports line and bulge-arc segments,
  open/closed polylines, offsets, booleans, containment, winding, intersections,
  and geometric properties.
- [validated] C FFI crate exists with opaque handles, C ABI functions, status
  codes, and a committed generated header.
- [validated] Demo UI exists for offset, boolean, and multi-polyline shape
  exploration.
- [validated] Integration tests cover primitive intersections, polyline basics,
  offsets, booleans, shape offsets, and FFI behavior.
- [validated] CI builds and tests the workspace on Linux, Windows, and macOS,
  with formatting, clippy, and docs checks.
- [validated] Phase 1 recorded the cross-source capability inventory,
  provenance boundaries, comparison taxonomy, and public API/FFI surface audit.
- [validated] Phase 2 added a test-only typed fixture schema, reusable property
  harness, and current-Rust seed fixtures for offset, boolean, and
  contains/properties behavior.

### Active

- [ ] Audit current Rust, old C++ CavalierContours, and Clipper2 for API,
  behavior, algorithm, fixture, benchmark, and license boundaries.
- [ ] Build a repeatable test and benchmark base that can import or translate
  historical C++ cases and compare polygon-only cases against Clipper2.
- [ ] Prioritize robustness work for offsets, booleans, intersections, numeric
  tolerances, degenerate geometry, repeat vertices, and open/closed behavior.
- [ ] Expand capabilities selectively where Clipper2 or old C++ behavior exposes
  valuable gaps that fit the Rust crate's arc-aware model.
- [ ] Preserve the Rust crate as the mainline implementation; external codebases
  should inform behavior and tests before any algorithm is ported.
- [ ] Keep FFI and public API impact explicit, tested, and documented when a
  phase changes externally visible behavior.
- [ ] Use the demo UI only when new functionality needs a visible exploration or
  validation path.

### Out of Scope

- Triangulation - explicitly deferred for now, including Clipper2 triangulation
  parity.
- Treating Clipper2 as a drop-in replacement backend - it is polygon-focused and
  does not model arc-aware polylines directly.
- Large UI redesigns unrelated to new geometry functionality - the existing demo
  should stay lightweight unless a new feature needs visual validation.
- Port-first implementation - tests, audits, and behavioral evidence should
  precede algorithm absorption.

## Context

The current repository is a Rust workspace with four members:
`cavalier_contours`, `cavalier_contours_ffi`, `cavalier_contours_ui`, and
`examples`. The codebase map in `.planning/codebase/` records the current stack,
architecture, structure, conventions, testing surface, integrations, and
concerns.

The old C++ CavalierContours repository is valuable as historical source
material because the Rust crate is a rewrite of that project. It should be mined
for algorithm intent, edge cases, fixtures, benchmark structure, C API behavior,
and migration expectations.

Clipper2 is valuable as a more active polygon geometry reference. It should be
used carefully for polygon-only clipping, offsetting, robustness comparisons,
and fixture generation. It should not set behavior for arc-specific operations
without an explicit adaptation design.

The first meaningful risk is not implementation difficulty alone; it is
behavioral drift. The project needs durable evidence before broad algorithm
changes: cross-library case inventory, normalized fixtures, expected-result
rules, benchmark baselines, and a clear distinction between exact parity,
approximate parity, and intentionally different arc-aware behavior.

Phase 2 established the first normalized fixture layer in Rust test utilities.
The harness is intentionally test-only and seeds only current Rust behavior;
old C++ mining, Clipper2 oracle output, and benchmarks remain downstream work.

## Constraints

- **Mainline implementation**: Rust `cavalier_contours` remains the only primary
  implementation target - old C++ and Clipper2 are references and oracles.
- **Safety boundary**: the core crate currently forbids unsafe code; unsafe work
  belongs in the FFI crate unless the policy is deliberately revisited.
- **Compatibility**: public Rust API, serde shape, FFI ABI, and generated header
  changes require explicit tests and documentation.
- **Numerics**: offset, boolean, and intersection behavior is epsilon-sensitive;
  phases that touch algorithms need regression cases around tolerance edges.
- **Scope**: this is a multi-milestone roadmap, not a short phase bundle.
- **UI**: demo UI changes are secondary and should be tied to new geometry
  functionality or validation needs.
- **Excluded capability**: triangulation is deferred.

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust `cavalier_contours` is the only mainline implementation | Keeps ownership, API, safety, and release direction clear | Pending |
| Old C++ CavalierContours is a historical reference source | It carries original algorithm intent, test ideas, and migration context | Pending |
| Clipper2 is a polygon-only oracle and reference | It is active and robust, but not arc-aware in the same model | Pending |
| Start with audit and test/benchmark base | Absorption without evidence risks silent behavior regressions | Pending |
| Cover robustness, capabilities, tests, and high-risk fixes | The user selected all four as initial output categories | Pending |
| Defer triangulation | The user explicitly excluded it for now | Pending |
| Keep UI changes feature-driven | Avoids turning the demo into a separate productization effort | Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition**:
1. Requirements invalidated? Move to Out of Scope with reason.
2. Requirements validated? Move to Validated with phase reference.
3. New requirements emerged? Add to Active.
4. Decisions to log? Add to Key Decisions.
5. "What This Is" still accurate? Update if drifted.

**After each milestone**:
1. Full review of all sections.
2. Core Value check - still the right priority?
3. Audit Out of Scope - reasons still valid?
4. Update Context with current state.

---
*Last updated: 2026-05-12 after Phase 2 completion*
