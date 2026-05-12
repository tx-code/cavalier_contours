# Research Summary

**Domain:** Arc-aware 2D computational geometry library absorption
**Researched:** 2026-05-12
**Confidence:** HIGH for first roadmap direction

## Key Findings

**Stack:** Keep Rust `cavalier_contours` as the only production mainline. Use
old C++ CavalierContours and Clipper2 as reference inputs, fixture sources,
benchmark sources, and dev-only oracle tooling.

**Table Stakes:** The project needs audit, fixture schema, benchmark baseline,
historical C++ fixture mining, Clipper2 polygon-only oracle support, robustness
gap ranking, and FFI impact tracking before broad algorithm absorption.

**Watch Out For:** Do not treat Clipper2 as arc-aware, do not port algorithms
before evidence exists, do not widen numeric tolerances globally, and do not let
FFI/header drift occur silently.

## Recommended Roadmap Shape

1. Establish the absorption contract: scope, licenses, geometry model taxonomy,
   and behavior comparison rules.
2. Build fixture and benchmark infrastructure before implementation work.
3. Mine old C++ tests/benchmarks and Clipper2 polygon cases into classified
   evidence.
4. Rank robustness gaps by risk and fix the highest-value current Rust issues.
5. Absorb selected capabilities only after tests and behavior expectations are
   durable.
6. Update FFI, examples, docs, and UI only when behavior becomes externally
   visible.

## Initial Phase Implications

- Phase 1 should not be a code port. It should be an audit and taxonomy phase.
- Phase 2 should create the fixture/benchmark base.
- Phase 3 should import or translate reference cases.
- Phase 4 should establish the Clipper2 oracle boundary.
- Algorithm fixes should begin only after enough evidence exists to rank them.

## Requirements Seeds

- REQ: keep Rust crate as primary implementation.
- REQ: classify all external reference cases by comparability.
- REQ: preserve arc-aware behavior as the core value.
- REQ: add repeatable tests and benchmarks before changing algorithms broadly.
- REQ: exclude triangulation for this milestone.
- REQ: keep UI work tied to new geometry functionality.
- REQ: require FFI/header checks when ABI changes.

## Sources

- `.planning/PROJECT.md`
- `.planning/codebase/*.md`
- `.planning/research/STACK.md`
- `.planning/research/FEATURES.md`
- `.planning/research/ARCHITECTURE.md`
- `.planning/research/PITFALLS.md`

---
*Research summary for: arc-aware geometry absorption*
*Researched: 2026-05-12*
